use std::fmt::{self, Debug, Display};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use actix_http::{Error, Payload, Response};
use brace_web_core::error::UrlencodedError;
use brace_web_core::http::header::ContentType;
use brace_web_core::http::StatusCode;
use brace_web_core::{FromRequest, HttpRequest, Responder};
use futures::future::{err, ok, FutureExt, LocalBoxFuture, Ready};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::url_encoded::UrlEncoded;

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

#[cfg(test)]
mod tests {
    use brace_web_core::body::{Body, ResponseBody};
    use brace_web_core::http::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
    use brace_web_core::http::StatusCode;
    use brace_web_core::test::TestRequest;
    use brace_web_core::{FromRequest, Responder};
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};

    use super::Form;

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
