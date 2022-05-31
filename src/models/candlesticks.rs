use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::models::{serfloats, serdates};
use chrono::prelude::*;

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum CandlestickGranularity {
    #[serde(rename = "S5")]
    S5,
    #[serde(rename = "S10")]
    S10,
    #[serde(rename = "S15")]
    S15,
    #[serde(rename = "S30")]
    S30,
    #[serde(rename = "M1")]
    M1,
    #[serde(rename = "M2")]
    M2,
    #[serde(rename = "M4")]
    M4,
    #[serde(rename = "M5")]
    M5,
    #[serde(rename = "M10")]
    M10,
    #[serde(rename = "M15")]
    M15,
    #[serde(rename = "M30")]
    M30,
    #[serde(rename = "H1")]
    H1,
    #[serde(rename = "H2")]
    H2,
    #[serde(rename = "H3")]
    H3,
    #[serde(rename = "H4")]
    H4,
    #[serde(rename = "H6")]
    H6,
    #[serde(rename = "H8")]
    H8,
    #[serde(rename = "H12")]
    H12,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "W")]
    W,
    #[serde(rename = "M")]
    M,
}

impl FromStr for CandlestickGranularity {
    type Err = ();
    fn from_str(s: &str) -> Result<CandlestickGranularity, ()> {
        match s {
            "S5" => Ok(CandlestickGranularity::S5),
            "S10" => Ok(CandlestickGranularity::S10),
            "S15" => Ok(CandlestickGranularity::S15),
            "S30" => Ok(CandlestickGranularity::S30),
            "M1" => Ok(CandlestickGranularity::M1),
            "M2" => Ok(CandlestickGranularity::M2),
            "M4" => Ok(CandlestickGranularity::M4),
            "M5" => Ok(CandlestickGranularity::M5),
            "M10" => Ok(CandlestickGranularity::M10),
            "M15" => Ok(CandlestickGranularity::M15),
            "M30" => Ok(CandlestickGranularity::M30),
            "H1" => Ok(CandlestickGranularity::H1),
            "H2" => Ok(CandlestickGranularity::H2),
            "H3" => Ok(CandlestickGranularity::H3),
            "H4" => Ok(CandlestickGranularity::H4),
            "H6" => Ok(CandlestickGranularity::H6),
            "H8" => Ok(CandlestickGranularity::H8),
            "H12" => Ok(CandlestickGranularity::H12),
            "D" => Ok(CandlestickGranularity::D),
            "W" => Ok(CandlestickGranularity::W),
            "M" => Ok(CandlestickGranularity::M),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for CandlestickGranularity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum WeeklyAlignment {
    #[serde(rename = "Monday")]
    Monday,
    #[serde(rename = "Tuesday")]
    Tuesday,
    #[serde(rename = "Wednesday")]
    Wednesday,
    #[serde(rename = "Thursday")]
    Thursday,
    #[serde(rename = "Friday")]
    Friday,
    #[serde(rename = "Saturday")]
    Saturday,
    #[serde(rename = "Sunday")]
    Sunday,
}

impl FromStr for WeeklyAlignment {
    type Err = ();
    fn from_str(s: &str) -> Result<WeeklyAlignment, ()> {
        match s {
            "Monday" => Ok(WeeklyAlignment::Monday),
            "Tuesday" => Ok(WeeklyAlignment::Tuesday),
            "Wednesday" => Ok(WeeklyAlignment::Wednesday),
            "Thursday" => Ok(WeeklyAlignment::Thursday),
            "Friday" => Ok(WeeklyAlignment::Friday),
            "Saturday" => Ok(WeeklyAlignment::Saturday),
            "Sunday" => Ok(WeeklyAlignment::Sunday),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for WeeklyAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Candlestick {
    #[serde(default)]
    #[serde(rename = "time", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub time: Option<DateTime<Utc>>,

    #[serde(default)]
    #[serde(rename = "bid", skip_serializing_if = "Option::is_none")]
    pub bid: Option<CandlestickData>,

    #[serde(default)]
    #[serde(rename = "ask", skip_serializing_if = "Option::is_none")]
    pub ask: Option<CandlestickData>,

    #[serde(default)]
    #[serde(rename = "mid", skip_serializing_if = "Option::is_none")]
    pub mid: Option<CandlestickData>,

    #[serde(default)]
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i32>,

    #[serde(default)]
    #[serde(rename = "complete", skip_serializing_if = "Option::is_none")]
    pub complete: Option<bool>,
}

impl Candlestick {
    pub fn new() -> Candlestick {
        Candlestick {
            time: None,
            bid: None,
            ask: None,
            mid: None,
            volume: None,
            complete: None,
        }
    }
    pub fn with_time(mut self, x: DateTime<Utc>) -> Self {
        self.time = Some(x);
        self
    }
    pub fn with_bid(mut self, x: CandlestickData) -> Self {
        self.bid = Some(x);
        self
    }
    pub fn with_ask(mut self, x: CandlestickData) -> Self {
        self.ask = Some(x);
        self
    }
    pub fn with_mid(mut self, x: CandlestickData) -> Self {
        self.mid = Some(x);
        self
    }
    pub fn with_volume(mut self, x: i32) -> Self {
        self.volume = Some(x);
        self
    }
    pub fn with_complete(mut self, x: bool) -> Self {
        self.complete = Some(x);
        self
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct CandlestickData {
    #[serde(default)]
    #[serde(rename = "o", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub o: Option<f32>,

    #[serde(default)]
    #[serde(rename = "h", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub h: Option<f32>,

    #[serde(default)]
    #[serde(rename = "l", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub l: Option<f32>,

    #[serde(default)]
    #[serde(rename = "c", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub c: Option<f32>,
}

impl CandlestickData {
    pub fn new() -> CandlestickData {
        CandlestickData {
            o: None,
            h: None,
            l: None,
            c: None,
        }
    }

    pub fn with_o(mut self, x: f32) -> Self {
        self.o = Some(x);
        self
    }

    pub fn with_h(mut self, x: f32) -> Self {
        self.h = Some(x);
        self
    }

    pub fn with_l(mut self, x: f32) -> Self {
        self.l = Some(x);
        self
    }

    pub fn with_c(mut self, x: f32) -> Self {
        self.c = Some(x);
        self
    }
}
