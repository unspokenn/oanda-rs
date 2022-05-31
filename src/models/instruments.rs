use serde::{Deserialize, Serialize};
use crate::serfloats;

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
