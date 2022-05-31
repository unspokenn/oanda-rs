use serde::{Deserialize, Serialize};
use crate::models::{serfloats, serdates};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInstrumentsResponseBody {
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<Instrument>>,
    #[serde(rename = "lastTransactionID", skip_deserializing, skip_serializing)]
    pub last_transaction_id: Option<String>,
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    #[serde(default)]
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub otype: Option<String>,
    #[serde(default)]
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(default)]
    #[serde(rename = "pipLocation", skip_serializing_if = "Option::is_none")]
    pub pip_location: Option<i32>,
    #[serde(default)]
    #[serde(rename = "displayPrecision", skip_serializing_if = "Option::is_none")]
    pub display_precision: Option<i32>,
    #[serde(default)]
    #[serde(rename = "tradeUnitsPrecision", skip_serializing_if = "Option::is_none")]
    pub trade_units_precision: Option<i32>,
    #[serde(default)]
    #[serde(rename = "minimumTradeSize", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub minimum_trade_size: Option<f32>,
    #[serde(default)]
    #[serde(rename = "maximumTrailingStopDistance", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub maximum_trailing_stop_distance: Option<f32>,
    #[serde(default)]
    #[serde(rename = "minimumGuaranteedStopLossDistance", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub minimum_guaranteed_stop_loss_distance: Option<f32>,
    #[serde(default)]
    #[serde(rename = "minimumTrailingStopDistance", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub minimum_trailing_stop_distance: Option<f32>,
    #[serde(default)]
    #[serde(rename = "maximumPositionSize", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub maximum_position_size: Option<f32>,
    #[serde(default)]
    #[serde(rename = "maximumOrderUnits", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub maximum_order_units: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_rate: Option<f32>,
    #[serde(default)]
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<InstrumentCommission>,
    #[serde(default)]
    #[serde(rename = "guaranteedStopLossOrderMode", skip_serializing_if = "Option::is_none")]
    pub guaranteed_stop_loss_order_mode: Option<String>,
    #[serde(default)]
    #[serde(rename = "guaranteedStopLossOrderExecutionPremium", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub guaranteed_stop_loss_order_execution_premium: Option<f32>,
    #[serde(default)]
    #[serde(rename = "guaranteedStopLossOrderLevelRestriction", skip_serializing_if = "Option::is_none")]
    pub guaranteed_stop_loss_order_level_restriction: Option<GuaranteedStopLossOrderLevelRestriction>,
    #[serde(default)]
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none")]
    pub financing: Option<InstrumentFinancing>,
    #[serde(default)]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

impl Instrument {
    pub fn new() -> Instrument {
        Instrument {
            name: None,
            otype: None,
            display_name: None,
            pip_location: None,
            display_precision: None,
            trade_units_precision: None,
            minimum_trade_size: None,
            maximum_trailing_stop_distance: None,
            minimum_guaranteed_stop_loss_distance: None,
            minimum_trailing_stop_distance: None,
            maximum_position_size: None,
            maximum_order_units: None,
            margin_rate: None,
            commission: None,
            guaranteed_stop_loss_order_mode: None,
            guaranteed_stop_loss_order_execution_premium: None,
            guaranteed_stop_loss_order_level_restriction: None,
            financing: None,
            tags: None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentCommission {
    #[serde(default)]
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub commission: Option<f32>,
    #[serde(default)]
    #[serde(rename = "unitsTraded", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub units_traded: Option<f32>,
    #[serde(default)]
    #[serde(rename = "minimumCommission", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub minimum_commission: Option<f32>,
}

impl InstrumentCommission {
    pub fn new() -> InstrumentCommission {
        InstrumentCommission {
            commission: None,
            units_traded: None,
            minimum_commission: None,
        }
    }

    pub fn with_commission(mut self, x: f32) -> Self {
        self.commission = Some(x);
        self
    }

    pub fn with_units_traded(mut self, x: f32) -> Self {
        self.units_traded = Some(x);
        self
    }

    pub fn with_minimum_commission(mut self, x: f32) -> Self {
        self.minimum_commission = Some(x);
        self
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuaranteedStopLossOrderLevelRestriction {
    #[serde(default)]
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub volume: Option<f32>,

    #[serde(default)]
    #[serde(rename = "priceRange", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub price_range: Option<f32>,
}

impl GuaranteedStopLossOrderLevelRestriction {
    pub fn new() -> GuaranteedStopLossOrderLevelRestriction {
        GuaranteedStopLossOrderLevelRestriction {
            volume: None,
            price_range: None,
        }
    }

    pub fn with_volume(mut self, x: f32) -> Self {
        self.volume = Some(x);
        self
    }

    pub fn with_price_range(mut self, x: f32) -> Self {
        self.price_range = Some(x);
        self
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentFinancing {
    #[serde(default)]
    #[serde(rename = "longRate", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub long_rate: Option<f32>,

    #[serde(default)]
    #[serde(rename = "shortRate", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub short_rate: Option<f32>,

    #[serde(default)]
    #[serde(rename = "financingDaysOfWeek", skip_serializing_if = "Option::is_none")]
    pub financing_days_of_week: Option<Vec<FinancingDayOfWeek>>,
}

impl InstrumentFinancing {
    pub fn new() -> InstrumentFinancing {
        InstrumentFinancing {
            long_rate: None,
            short_rate: None,
            financing_days_of_week: None,
        }
    }

    pub fn with_long_rate(mut self, x: f32) -> Self {
        self.long_rate = Some(x);
        self
    }

    pub fn with_short_rate(mut self, x: f32) -> Self {
        self.short_rate = Some(x);
        self
    }

    pub fn with_financing_days_of_week(mut self, x: Vec<FinancingDayOfWeek>) -> Self {
        self.financing_days_of_week = Some(x);
        self
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancingDayOfWeek {
    #[serde(default)]
    #[serde(rename = "dayOfWeek", skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<String>,

    #[serde(default)]
    #[serde(rename = "daysCharged", skip_serializing_if = "Option::is_none")]
    pub days_charged: Option<i32>,
}

impl FinancingDayOfWeek {
    pub fn new() -> FinancingDayOfWeek {
        FinancingDayOfWeek {
            day_of_week: None,
            days_charged: None,
        }
    }

    pub fn with_day_of_week(mut self, x: String) -> Self {
        self.day_of_week = Some(x);
        self
    }

    pub fn with_days_charged(mut self, x: i32) -> Self {
        self.days_charged = Some(x);
        self
    }
}

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    #[serde(default)]
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub otype: Option<String>,

    #[serde(default)]
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Tag {
    pub fn new() -> Tag {
        Tag {
            otype: None,
            name: None,
        }
    }

    pub fn with_otype(mut self, x: String) -> Self {
        self.otype = Some(x);
        self
    }

    pub fn with_name(mut self, x: String) -> Self {
        self.name = Some(x);
        self
    }
}
