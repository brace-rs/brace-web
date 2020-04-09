use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};

#[derive(Clone, Debug, PartialEq)]
pub struct HttpRedirect(StatusCode, String);

impl HttpRedirect {
    pub fn to<U>(uri: U) -> Self
    where
        U: Into<String>,
    {
        Self(StatusCode::SEE_OTHER, uri.into())
    }

    pub fn temporary<U>(uri: U) -> Self
    where
        U: Into<String>,
    {
        Self(StatusCode::TEMPORARY_REDIRECT, uri.into())
    }

    pub fn permanent<U>(uri: U) -> Self
    where
        U: Into<String>,
    {
        Self(StatusCode::PERMANENT_REDIRECT, uri.into())
    }

    pub fn found<U>(uri: U) -> Self
    where
        U: Into<String>,
    {
        Self(StatusCode::FOUND, uri.into())
    }

    pub fn moved<U>(uri: U) -> Self
    where
        U: Into<String>,
    {
        Self(StatusCode::MOVED_PERMANENTLY, uri.into())
    }

    pub fn into_response(self) -> HttpResponse {
        HttpResponse::build(self.0)
            .header(LOCATION, self.1)
            .finish()
    }
}

impl Responder for HttpRedirect {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(self.into_response())
    }
}

impl From<HttpRedirect> for HttpResponse {
    fn from(from: HttpRedirect) -> Self {
        from.into_response()
    }
}

#[cfg(test)]
mod tests {
    use crate::http::header::LOCATION;
    use crate::http::StatusCode;
    use crate::test::TestRequest;
    use crate::{HttpRedirect, HttpResponse, Responder};

    #[actix_rt::test]
    async fn test_http_redirect_to() {
        let req = TestRequest::default().to_http_request();
        let res = HttpRedirect::to("www.example.com")
            .respond_to(&req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::SEE_OTHER);
        assert_eq!(res.headers().get(LOCATION).unwrap(), "www.example.com");
    }

    #[actix_rt::test]
    async fn test_http_redirect_temporary() {
        let req = TestRequest::default().to_http_request();
        let res = HttpRedirect::temporary("www.example.com")
            .respond_to(&req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::TEMPORARY_REDIRECT);
        assert_eq!(res.headers().get(LOCATION).unwrap(), "www.example.com");
    }

    #[actix_rt::test]
    async fn test_http_redirect_permanent() {
        let req = TestRequest::default().to_http_request();
        let res = HttpRedirect::permanent("www.example.com")
            .respond_to(&req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::PERMANENT_REDIRECT);
        assert_eq!(res.headers().get(LOCATION).unwrap(), "www.example.com");
    }

    #[actix_rt::test]
    async fn test_http_redirect_found() {
        let req = TestRequest::default().to_http_request();
        let res = HttpRedirect::found("www.example.com")
            .respond_to(&req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::FOUND);
        assert_eq!(res.headers().get(LOCATION).unwrap(), "www.example.com");
    }

    #[actix_rt::test]
    async fn test_http_redirect_moved() {
        let req = TestRequest::default().to_http_request();
        let res = HttpRedirect::moved("www.example.com")
            .respond_to(&req)
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::MOVED_PERMANENTLY);
        assert_eq!(res.headers().get(LOCATION).unwrap(), "www.example.com");
    }

    #[test]
    fn test_http_redirect_into() {
        let res: HttpResponse = HttpRedirect::to("www.example.com").into();

        assert_eq!(res.status(), StatusCode::SEE_OTHER);
        assert_eq!(res.headers().get(LOCATION).unwrap(), "www.example.com");
    }
}
