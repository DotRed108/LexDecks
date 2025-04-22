use std::future::Future;

use leptos::prelude::{expect_context, Action, GetUntracked};
use server_fn::{
    client::{browser::BrowserClient, Client}, 
    error::FromServerFnError, 
    request::browser::BrowserRequest, 
    response::browser::BrowserResponse
};
use crate::app::UpdateUserState;
use super::{shared_truth::AUTH_TOKEN_HEADER, shared_utilities::UserState};

pub struct AuthClient;

impl<E> Client<E> for AuthClient
where
    E: FromServerFnError,
{
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(
        req: Self::Request,
    ) -> impl Future<Output = Result<Self::Response, E>> + Send {
        let user_action = expect_context::<Action<UpdateUserState, UserState>>();
        // BrowserRequest derefs to the underlying Request type from gloo-net,
        // so we can get access to the headers here
        let headers = req.headers();
        // modify the headers by appending one
        headers.append(AUTH_TOKEN_HEADER, user_action.value().get_untracked().unwrap_or_default().token());
        // delegate back out to BrowserClient to send the modified request
        <BrowserClient as Client<E>>::send(req)
    }

    fn spawn(future: impl Future<Output = ()> + Send + 'static) {
        <BrowserClient as Client<E>>::spawn(future)
    }
    
    fn open_websocket(
        path: &str,
    ) -> impl Future<
        Output = Result<
            (
                impl futures::Stream<Item = Result<server_fn::Bytes, server_fn::Bytes>> + Send + 'static,
                impl futures::Sink<Result<server_fn::Bytes, server_fn::Bytes>> + Send + 'static,
            ),
            E,
        >,
    > + Send {
        <BrowserClient as Client<E>>::open_websocket(path)
    }
}
