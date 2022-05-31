// use warp::hyper::Method;
// use crate::{Client, RequestBuilder};

// mod account;

// pub use account::*;
// pub struct Request(Inner);
// pub static APP_KEY: &'static str = env!("OANDA_KEY");

// pub enum Request {
//     ListAccountsRequest,
//
// }
//
// impl Request {
//     // pub const  LIST_ACCOUNT: Method = Method(Get);
//     pub fn builder(&self) -> RequestBuilder {
//         match &self {
//             Request::ListAccountsRequest => RequestBuilder::api("/v3/accounts",Method::GET,APP_KEY)
//         }
//     }
//     // pub async fn send(&self, client: &Client) -> Result<impl Responses> {
//     //     match &self {
//     //         Request::ListAccountsRequest => RequestBuilder::api("/v3/accounts",Method::POST,APP_KEY).send::<ListAccountsResponse200Body>(client).await
//     //     }
//     // }
// }


// pub struct InvalidRequest {
//     _priv: (),
// }
// impl InvalidRequest {
//     fn new() -> InvalidRequest {
//         InvalidRequest { _priv: () }
//     }
// }
//
// impl std::fmt::Debug for InvalidRequest {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         f.debug_struct("InvalidRequest")
//             // skip _priv noise
//          .finish()
//     }
// }
//
// impl std::fmt::Display for InvalidRequest {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("invalid Request")
//     }
// }
//
// impl std::error::Error for InvalidRequest {}
