use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use actix_http::{HttpMessage, Payload};
use brace_web_core::dev::Decompress;
use brace_web_core::http::header::CONTENT_LENGTH;
use brace_web_core::HttpRequest;
use bytes::BytesMut;
use encoding_rs::{Encoding, UTF_8};
use futures::future::{FutureExt, LocalBoxFuture};
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;

use self::error::UrlEncodedError;

pub mod error;

pub struct UrlEncoded<U> {
    stream: Option<Decompress<Payload>>,
    limit: usize,
    length: Option<usize>,
    encoding: &'static Encoding,
    err: Option<UrlEncodedError>,
    fut: Option<LocalBoxFuture<'static, Result<U, UrlEncodedError>>>,
}

impl<U> UrlEncoded<U> {
    pub fn new(req: &HttpRequest, payload: &mut Payload) -> UrlEncoded<U> {
        if req.content_type().to_lowercase() != "application/x-www-form-urlencoded" {
            return Self::err(UrlEncodedError::ContentType);
        }

        let encoding = match req.encoding() {
            Ok(enc) => enc,
            Err(_) => return Self::err(UrlEncodedError::ContentType),
        };

        let mut len = None;

        if let Some(l) = req.headers().get(CONTENT_LENGTH) {
            if let Ok(s) = l.to_str() {
                if let Ok(l) = s.parse::<usize>() {
                    len = Some(l)
                } else {
                    return Self::err(UrlEncodedError::UnknownLength);
                }
            } else {
                return Self::err(UrlEncodedError::UnknownLength);
            }
        };

        let payload = Decompress::from_headers(payload.take(), req.headers());

        UrlEncoded {
            encoding,
            stream: Some(payload),
            limit: 32_768,
            length: len,
            fut: None,
            err: None,
        }
    }

    fn err(e: UrlEncodedError) -> Self {
        UrlEncoded {
            stream: None,
            limit: 32_768,
            fut: None,
            err: Some(e),
            length: None,
            encoding: UTF_8,
        }
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
}

impl<U> Future for UrlEncoded<U>
where
    U: DeserializeOwned + 'static,
{
    type Output = Result<U, UrlEncodedError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(ref mut fut) = self.fut {
            return Pin::new(fut).poll(cx);
        }

        if let Some(err) = self.err.take() {
            return Poll::Ready(Err(err));
        }

        let limit = self.limit;

        if let Some(len) = self.length.take() {
            if len > limit {
                return Poll::Ready(Err(UrlEncodedError::Overflow { size: len, limit }));
            }
        }

        let encoding = self.encoding;
        let mut stream = self.stream.take().unwrap();

        self.fut = Some(
            async move {
                let mut body = BytesMut::with_capacity(8192);

                while let Some(item) = stream.next().await {
                    let chunk = item?;
                    if (body.len() + chunk.len()) > limit {
                        return Err(UrlEncodedError::Overflow {
                            size: body.len() + chunk.len(),
                            limit,
                        });
                    } else {
                        body.extend_from_slice(&chunk);
                    }
                }

                if encoding == UTF_8 {
                    serde_qs::from_bytes::<U>(&body).map_err(|_| UrlEncodedError::Parse)
                } else {
                    let body = encoding
                        .decode_without_bom_handling_and_without_replacement(&body)
                        .map(|s| s.into_owned())
                        .ok_or(UrlEncodedError::Parse)?;

                    serde_qs::from_str::<U>(&body).map_err(|_| UrlEncodedError::Parse)
                }
            }
            .boxed_local(),
        );

        self.poll(cx)
    }
}

#[cfg(test)]
mod tests {
    use brace_web_core::body::{Body, ResponseBody};
    use brace_web_core::http::header::{CONTENT_LENGTH, CONTENT_TYPE};
    use brace_web_core::test::TestRequest;
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};

    use super::{UrlEncoded, UrlEncodedError};

    pub(crate) trait BodyTest {
        fn bin_ref(&self) -> &[u8];
    }

    impl BodyTest for ResponseBody<Body> {
        fn bin_ref(&self) -> &[u8] {
            match self {
                ResponseBody::Body(ref b) => match b {
                    Body::Bytes(ref bin) => &bin,
                    _ => panic!(),
                },
                ResponseBody::Other(ref b) => match b {
                    Body::Bytes(ref bin) => &bin,
                    _ => panic!(),
                },
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Info {
        hello: String,
        counter: i64,
    }

    fn eq(err: UrlEncodedError, other: UrlEncodedError) -> bool {
        match err {
            UrlEncodedError::Overflow { .. } => match other {
                UrlEncodedError::Overflow { .. } => true,
                _ => false,
            },
            UrlEncodedError::UnknownLength => match other {
                UrlEncodedError::UnknownLength => true,
                _ => false,
            },
            UrlEncodedError::ContentType => match other {
                UrlEncodedError::ContentType => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[actix_rt::test]
    async fn test_url_encoded_error() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "xxxx")
                .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await;

        assert!(eq(info.err().unwrap(), UrlEncodedError::UnknownLength));

        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "1000000")
                .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await;

        assert!(eq(
            info.err().unwrap(),
            UrlEncodedError::Overflow { size: 0, limit: 0 }
        ));

        let (req, mut pl) = TestRequest::with_header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, "10")
            .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await;

        assert!(eq(info.err().unwrap(), UrlEncodedError::ContentType));
    }

    #[actix_rt::test]
    async fn test_url_encoded() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "11")
                .set_payload(Bytes::from_static(b"hello=world&counter=123"))
                .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await.unwrap();

        assert_eq!(
            info,
            Info {
                hello: "world".to_owned(),
                counter: 123
            }
        );

        let (req, mut pl) = TestRequest::with_header(
            CONTENT_TYPE,
            "application/x-www-form-urlencoded; charset=utf-8",
        )
        .header(CONTENT_LENGTH, "11")
        .set_payload(Bytes::from_static(b"hello=world&counter=123"))
        .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await.unwrap();

        assert_eq!(
            info,
            Info {
                hello: "world".to_owned(),
                counter: 123
            }
        );
    }
}
