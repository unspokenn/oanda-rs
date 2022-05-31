use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{serfloats, serdates};

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientPrice {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub otype: Option<String>,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub time: Option<DateTime<Utc>>,
    #[serde(rename = "tradeable", skip_serializing_if = "Option::is_none")]
    pub tradeable: Option<bool>,
    #[serde(rename = "bids", skip_serializing_if = "Option::is_none")]
    pub bids: Option<Vec<PriceBucket>>,
    #[serde(rename = "asks", skip_serializing_if = "Option::is_none")]
    pub asks: Option<Vec<PriceBucket>>,
    #[serde(rename = "closeoutBid", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub closeout_bid: Option<f32>,
    #[serde(rename = "closeoutAsk", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub closeout_ask: Option<f32>,
}

impl TryFrom<&String> for ClientPrice {
    type Error = serde_json::error::Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        serde_json::from_slice::<Self>(value.as_bytes())
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PriceBucket {
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub price: Option<f32>,
    #[serde(rename = "liquidity", skip_serializing_if = "Option::is_none")]
    pub liquidity: Option<i32>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PricingHeartbeat {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub otype: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub time: Option<DateTime<Utc>>,
}

impl TryFrom<&String> for PricingHeartbeat {
    type Error = serde_json::error::Error;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        serde_json::from_slice::<Self>(value.as_bytes())
    }
}
