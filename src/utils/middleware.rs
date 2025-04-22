use axum::{
    body::Body, extract::Request, http::{HeaderValue, Method, Response}, middleware::Next
};

use crate::utils::{outcomes::Outcome, shared_truth::{AUTH_TOKEN_HEADER, USER_CLAIM_AUTH}, shared_utilities::verify_token};

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Response<Body> {
    println!("hello");
    if request.method() != Method::GET {
        let early_response = Response::builder().status(404).body(Outcome::VerificationFailure.to_string().into()).unwrap_or_default();
        println!("my middleware is running");
        println!("{}", request.uri());
        let headers = request.headers_mut();
        let Some(auth_header) = headers.get(AUTH_TOKEN_HEADER) else {return early_response};
        let Ok(trusted_token) = verify_token(auth_header.to_str().unwrap_or_default()) else {return early_response};
        
        let Some(claims) = trusted_token.payload_claims() else {return early_response};
        let Some(email) = claims.get_claim(USER_CLAIM_AUTH) else {return early_response};
        let Some(email) = email.as_str() else {return early_response};

        let Ok(email) = HeaderValue::from_str(email) else {return early_response};

        headers.append(USER_CLAIM_AUTH, email);
    };

    let response = next.run(request).await;

    // do something with `response`...

    response
}

// use axum::body::Body;
// use http::Request;
// use pin_project_lite::pin_project;
// use std::{
//     future::Future,
//     pin::Pin,
//     task::{Context, Poll},
// };
// use tower::{Layer, Service};

// pub struct LoggingLayer;

// impl<S> Layer<S> for LoggingLayer {
//     type Service = LoggingService<S>;

//     fn layer(&self, inner: S) -> Self::Service {
//         LoggingService { inner }
//     }
// }

// pub struct LoggingService<T> {
//     inner: T,
// }

// impl<T> Service<Request<Body>> for LoggingService<T>
// where
//     T: Service<Request<Body>>,
// {
//     type Response = T::Response;
//     type Error = T::Error;
//     type Future = LoggingServiceFuture<T::Future>;

//     fn poll_ready(
//         &mut self,
//         cx: &mut Context<'_>,
//     ) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         println!("1. Running my middleware!");

//         LoggingServiceFuture {
//             inner: self.inner.call(req),
//         }
//     }
// }

// pin_project! {
//     pub struct LoggingServiceFuture<T> {
//         #[pin]
//         inner: T,
//     }
// }

// impl<T> Future for LoggingServiceFuture<T>
// where
//     T: Future,
// {
//     type Output = T::Output;

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let this = self.project();
//         match this.inner.poll(cx) {
//             Poll::Pending => Poll::Pending,
//             Poll::Ready(output) => {
//                 println!("3. Running my middleware!");
//                 Poll::Ready(output)
//             }
//         }
//     }
// }
