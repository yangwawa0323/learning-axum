use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request},
    http::HeaderValue,
    response::Response,
};
use futures_util::future::BoxFuture;
use tower::{Layer, Service};
use tracing::info;

#[derive(Debug, Clone)]
pub struct MyLayer;

//  S -> a service
impl<S> Layer<S> for MyLayer {
    type Service = MyMiddleware<S>; // My middleware
    fn layer(&self, inner: S) -> Self::Service {
        MyMiddleware { inner }
    }
}

//  request -> md1 -> md2 -> md3 -> md2 -> md1 -> response

#[derive(Debug, Clone)]
pub struct MyMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for MyMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let uri = req.uri().path().to_string();
        let method = req.method().to_string();

        if let Some(ConnectInfo(conn)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
            info!("{} - {} : {}\n", conn, method, uri);
        } else {
            info!(" {} : {}\n", method, uri);
        }

        Box::pin(async move {
            let mut response = inner.call(req).await?;
            response.headers_mut().append(
                "X-custom-header",
                HeaderValue::from_static("Hello from custom header middleware"),
            );
            Ok(response)
        })
    }
}
