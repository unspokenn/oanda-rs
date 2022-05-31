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
impl ClientPrice {
    pub fn new() -> ClientPrice {
        ClientPrice {
            otype: None,
            instrument: None,
            time: None,
            tradeable: None,
            bids: None,
            asks: None,
            closeout_bid: None,
            closeout_ask: None,
        }
    }

    pub fn with_otype(mut self, x: String) -> Self {
        self.otype = Some(x);
        self
    }

    pub fn with_instrument(mut self, x: String) -> Self {
        self.instrument = Some(x);
        self
    }

    pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
        self.time = Some(x);
        self
    }

    pub fn with_tradeable(mut self, x: bool) -> Self {
        self.tradeable = Some(x);
        self
    }

    pub fn with_bids(mut self, x: Vec<PriceBucket>) -> Self {
        self.bids = Some(x);
        self
    }

    pub fn with_asks(mut self, x: Vec<PriceBucket>) -> Self {
        self.asks = Some(x);
        self
    }

    pub fn with_closeout_bid(mut self, x: f32) -> Self {
        self.closeout_bid = Some(x);
        self
    }

    pub fn with_closeout_ask(mut self, x: f32) -> Self {
        self.closeout_ask = Some(x);
        self
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
impl PriceBucket {
    pub fn new() -> PriceBucket {
        PriceBucket {
            price: None,
            liquidity: None,
        }
    }
    pub fn with_price(mut self, x: f32) -> Self {
        self.price = Some(x);
        self
    }

    pub fn with_liquidity(mut self, x: i32) -> Self {
        self.liquidity = Some(x);
        self
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PricingHeartbeat {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub otype: Option<String>,
    #[serde(rename = "time", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub time: Option<DateTime<Utc>>,
}
impl PricingHeartbeat {
    pub fn new() -> PricingHeartbeat {
        PricingHeartbeat {
            otype: None,
            time: None,
        }
    }

    pub fn with_otype(mut self, x: String) -> Self {
        self.otype = Some(x);
        self
    }

    pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
        self.time = Some(x);
        self
    }
}
