use std::fmt::{self, Debug, Display};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use actix_http::{Error, HttpMessage, Payload, Response};
use brace_web_core::dev::Decompress;
use brace_web_core::error::UrlencodedError;
use brace_web_core::http::header::{ContentType, CONTENT_LENGTH};
use brace_web_core::http::StatusCode;
use brace_web_core::{FromRequest, HttpRequest, Responder};
use bytes::BytesMut;
use encoding_rs::{Encoding, UTF_8};
use futures::future::{err, ok, FutureExt, LocalBoxFuture, Ready};
use futures::stream::StreamExt;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Form<T>(pub T);

impl<T> Form<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Form<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for Form<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T> FromRequest for Form<T>
where
    T: DeserializeOwned + 'static,
{
    type Config = FormConfig;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let req2 = req.clone();
        let (limit, err) = req
            .app_data::<FormConfig>()
            .map(|c| (c.limit, c.ehandler.clone()))
            .unwrap_or((16384, None));

        UrlEncoded::new(req, payload)
            .limit(limit)
            .map(move |res| match res {
                Err(e) => {
                    if let Some(err) = err {
                        Err((*err)(e, &req2))
                    } else {
                        Err(e.into())
                    }
                }
                Ok(item) => Ok(Form(item)),
            })
            .boxed_local()
    }
}

impl<T> Debug for Form<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Display for Form<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> Responder for Form<T>
where
    T: Serialize,
{
    type Error = Error;
    type Future = Ready<Result<Response, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        let body = match serde_urlencoded::to_string(&self.0) {
            Ok(body) => body,
            Err(e) => return err(e.into()),
        };

        ok(Response::build(StatusCode::OK)
            .set(ContentType::form_url_encoded())
            .body(body))
    }
}

#[derive(Clone)]
pub struct FormConfig {
    limit: usize,
    ehandler: Option<Rc<dyn Fn(UrlencodedError, &HttpRequest) -> Error>>,
}

impl FormConfig {
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }

    pub fn error_handler<F>(mut self, f: F) -> Self
    where
        F: Fn(UrlencodedError, &HttpRequest) -> Error + 'static,
    {
        self.ehandler = Some(Rc::new(f));
        self
    }
}

impl Default for FormConfig {
    fn default() -> Self {
        FormConfig {
            limit: 16384,
            ehandler: None,
        }
    }
}

pub struct UrlEncoded<U> {
    stream: Option<Decompress<Payload>>,
    limit: usize,
    length: Option<usize>,
    encoding: &'static Encoding,
    err: Option<UrlencodedError>,
    fut: Option<LocalBoxFuture<'static, Result<U, UrlencodedError>>>,
}

impl<U> UrlEncoded<U> {
    pub fn new(req: &HttpRequest, payload: &mut Payload) -> UrlEncoded<U> {
        if req.content_type().to_lowercase() != "application/x-www-form-urlencoded" {
            return Self::err(UrlencodedError::ContentType);
        }

        let encoding = match req.encoding() {
            Ok(enc) => enc,
            Err(_) => return Self::err(UrlencodedError::ContentType),
        };

        let mut len = None;

        if let Some(l) = req.headers().get(CONTENT_LENGTH) {
            if let Ok(s) = l.to_str() {
                if let Ok(l) = s.parse::<usize>() {
                    len = Some(l)
                } else {
                    return Self::err(UrlencodedError::UnknownLength);
                }
            } else {
                return Self::err(UrlencodedError::UnknownLength);
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

    fn err(e: UrlencodedError) -> Self {
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
    type Output = Result<U, UrlencodedError>;

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
                return Poll::Ready(Err(UrlencodedError::Overflow { size: len, limit }));
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
                        return Err(UrlencodedError::Overflow {
                            size: body.len() + chunk.len(),
                            limit,
                        });
                    } else {
                        body.extend_from_slice(&chunk);
                    }
                }

                if encoding == UTF_8 {
                    serde_urlencoded::from_bytes::<U>(&body).map_err(|_| UrlencodedError::Parse)
                } else {
                    let body = encoding
                        .decode_without_bom_handling_and_without_replacement(&body)
                        .map(|s| s.into_owned())
                        .ok_or(UrlencodedError::Parse)?;

                    serde_urlencoded::from_str::<U>(&body).map_err(|_| UrlencodedError::Parse)
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
    use brace_web_core::http::header::{HeaderValue, CONTENT_TYPE};
    use brace_web_core::test::TestRequest;
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};

    use super::*;

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

    #[actix_rt::test]
    async fn test_form() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "11")
                .set_payload(Bytes::from_static(b"hello=world&counter=123"))
                .to_http_parts();

        let Form(s) = Form::<Info>::from_request(&req, &mut pl).await.unwrap();

        assert_eq!(
            s,
            Info {
                hello: "world".into(),
                counter: 123
            }
        );
    }

    fn eq(err: UrlencodedError, other: UrlencodedError) -> bool {
        match err {
            UrlencodedError::Overflow { .. } => match other {
                UrlencodedError::Overflow { .. } => true,
                _ => false,
            },
            UrlencodedError::UnknownLength => match other {
                UrlencodedError::UnknownLength => true,
                _ => false,
            },
            UrlencodedError::ContentType => match other {
                UrlencodedError::ContentType => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[actix_rt::test]
    async fn test_urlencoded_error() {
        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "xxxx")
                .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await;

        assert!(eq(info.err().unwrap(), UrlencodedError::UnknownLength));

        let (req, mut pl) =
            TestRequest::with_header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .header(CONTENT_LENGTH, "1000000")
                .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await;

        assert!(eq(
            info.err().unwrap(),
            UrlencodedError::Overflow { size: 0, limit: 0 }
        ));

        let (req, mut pl) = TestRequest::with_header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, "10")
            .to_http_parts();

        let info = UrlEncoded::<Info>::new(&req, &mut pl).await;

        assert!(eq(info.err().unwrap(), UrlencodedError::ContentType));
    }

    #[actix_rt::test]
    async fn test_urlencoded() {
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

    #[actix_rt::test]
    async fn test_responder() {
        let req = TestRequest::default().to_http_request();
        let form = Form(Info {
            hello: "world".to_string(),
            counter: 123,
        });

        let res = form.respond_to(&req).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            res.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("application/x-www-form-urlencoded")
        );
        assert_eq!(res.body().bin_ref(), b"hello=world&counter=123");
    }
}
