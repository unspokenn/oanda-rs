mod account;
mod price;
// mod price;

pub use account::*;
pub use price::*;

// pub trait Responses {}


// pub type ListAccountsResponse = crate::response::account::ListAccountsResponse200Body;
// pub type StreamPricingResponse = crate::response::price::StreamPricingResponse200Body;

// impl Responses for ListAccountsResponse {}
// impl Responses for StreamPricingResponse {}

// pub enum Response {
//     ListAccountsRequest,
//
// }
//
// impl Response {
//     pub type ListAccountsResponse = crate::response::account::ListAccountsResponse200Body;
//     pub type StreamPricingResponse = crate::response::price::StreamPricingResponse200Body;
//     // pub const  LIST_ACCOUNT: Method = Method(Get);
//     // fn builder(&self) -> Self {
//     //     match &self {
//     //         Response::ListAccountsRequest => RequestBuilder::api("/v3/accounts",Method::POST)
//     //     }
//     // }
// }
