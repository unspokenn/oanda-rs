use serde::{Deserialize, Serialize};
use crate::{ClientPrice, PricingHeartbeat};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamPricingResponse200Header {
    #[serde(rename = "Link", skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "RequestID", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamPricingResponse200Body {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<ClientPrice>,
    #[serde(rename = "heartbeat", skip_serializing_if = "Option::is_none")]
    pub heartbeat: Option<PricingHeartbeat>,
}

pub trait Streams {
    fn new() -> Self;
    fn push(&mut self, response: StreamPricingResponse200Body) -> &mut Self;
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct StreamPricingResponses {
    pub responses: Vec<StreamPricingResponse200Body>,
}

impl Streams for StreamPricingResponses {
    fn new() -> Self {
        StreamPricingResponses {
            responses: vec![]
        }
    }
    fn push(&mut self, response: StreamPricingResponse200Body) -> &mut Self {
        self.responses.push(response);
        self
    }
}
