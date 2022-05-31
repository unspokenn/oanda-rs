use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::{serfloats, serdates};

#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProperties {
    #[serde(default)]
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(rename = "mt4AccountID", skip_serializing_if = "Option::is_none")]
    pub mt4_account_id: Option<i32>,
    #[serde(default)]
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    #[serde(default)]
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(default)]
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default)]
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub balance: Option<f32>,
    #[serde(default)]
    #[serde(rename = "createdByUserID", skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<i32>,
    #[serde(default)]
    #[serde(rename = "createdTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub created_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "guaranteedStopLossOrderMode", skip_serializing_if = "Option::is_none")]
    pub guaranteed_stop_loss_order_mode: Option<String>,
    #[serde(default)]
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub resettable_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "resettablePLTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub resettable_pl_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub financing: Option<f32>,
    #[serde(default)]
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub commission: Option<f32>,
    #[serde(default)]
    #[serde(rename = "guaranteedExecutionFees", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub guaranteed_execution_fees: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginRate", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_rate: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCallEnterTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub margin_call_enter_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "marginCallExtensionCount", skip_serializing_if = "Option::is_none")]
    pub margin_call_extension_count: Option<i32>,
    #[serde(default)]
    #[serde(rename = "lastMarginCallExtensionTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub last_margin_call_extension_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "openTradeCount", skip_serializing_if = "Option::is_none")]
    pub open_trade_count: Option<i32>,
    #[serde(default)]
    #[serde(rename = "openPositionCount", skip_serializing_if = "Option::is_none")]
    pub open_position_count: Option<i32>,
    #[serde(default)]
    #[serde(rename = "pendingOrderCount", skip_serializing_if = "Option::is_none")]
    pub pending_order_count: Option<i32>,
    #[serde(default)]
    #[serde(rename = "hedgingEnabled", skip_serializing_if = "Option::is_none")]
    pub hedging_enabled: Option<bool>,
    #[serde(default)]
    #[serde(rename = "lastOrderFillTimestamp", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub last_order_fill_timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub unrealized_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "NAV", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub nav: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_used: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginAvailable", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_available: Option<f32>,
    #[serde(default)]
    #[serde(rename = "positionValue", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub position_value: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCloseoutUnrealizedPL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_closeout_unrealized_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCloseoutNAV", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_closeout_nav: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCloseoutMarginUsed", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_closeout_margin_used: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCloseoutPercent", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_closeout_percent: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCloseoutPositionValue", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_closeout_position_value: Option<f32>,
    #[serde(default)]
    #[serde(rename = "withdrawalLimit", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub withdrawal_limit: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCallMarginUsed", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_call_margin_used: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginCallPercent", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_call_percent: Option<f32>,
    #[serde(default)]
    #[serde(rename = "lastTransactionID", skip_serializing_if = "Option::is_none")]
    pub last_transaction_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "trades", skip_serializing_if = "Option::is_none")]
    pub trades: Option<Vec<TradeSummary>>,
    #[serde(default)]
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<Position>>,
    #[serde(default)]
    #[serde(rename = "orders", skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<Order>>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TradeSummary {
    #[serde(default)]
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(default)]
    #[serde(rename = "price", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub price: Option<f32>,
    #[serde(default)]
    #[serde(rename = "openTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub open_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(rename = "initialUnits", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub initial_units: Option<f32>,
    #[serde(default)]
    #[serde(rename = "initialMarginRequired", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub initial_margin_required: Option<f32>,
    #[serde(default)]
    #[serde(rename = "currentUnits", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub current_units: Option<f32>,
    #[serde(default)]
    #[serde(rename = "realizedPL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub realized_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub unrealized_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_used: Option<f32>,
    #[serde(default)]
    #[serde(rename = "averageClosePrice", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub average_close_price: Option<f32>,
    #[serde(default)]
    #[serde(rename = "closingTransactionIDs", skip_serializing_if = "Option::is_none")]
    pub closing_transaction_i_ds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub financing: Option<f32>,
    #[serde(default)]
    #[serde(rename = "closeTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub close_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
    #[serde(default)]
    #[serde(rename = "takeProfitOrderID", skip_serializing_if = "Option::is_none")]
    pub take_profit_order_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "stopLossOrderID", skip_serializing_if = "Option::is_none")]
    pub stop_loss_order_id: Option<String>,
    #[serde(default)]
    #[serde(rename = "trailingStopLossOrderID", skip_serializing_if = "Option::is_none")]
    pub trailing_stop_loss_order_id: Option<String>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ClientExtensions {
    #[serde(default)]
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default)]
    #[serde(rename = "comment", skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    #[serde(default)]
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<String>,
    #[serde(default)]
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub unrealized_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "marginUsed", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub margin_used: Option<f32>,
    #[serde(default)]
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub resettable_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub financing: Option<f32>,
    #[serde(default)]
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub commission: Option<f32>,
    #[serde(default)]
    #[serde(rename = "guaranteedExecutionFees", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub guaranteed_execution_fees: Option<f32>,
    #[serde(default)]
    #[serde(rename = "long", skip_serializing_if = "Option::is_none")]
    pub long: Option<PositionSide>,
    #[serde(default)]
    #[serde(rename = "short", skip_serializing_if = "Option::is_none")]
    pub short: Option<PositionSide>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PositionSide {
    #[serde(default)]
    #[serde(rename = "units", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub units: Option<f32>,
    #[serde(default)]
    #[serde(rename = "averagePrice", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub average_price: Option<f32>,
    #[serde(default)]
    #[serde(rename = "tradeIDs", skip_serializing_if = "Option::is_none")]
    pub trade_i_ds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "pl", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "unrealizedPL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub unrealized_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "resettablePL", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub resettable_pl: Option<f32>,
    #[serde(default)]
    #[serde(rename = "financing", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub financing: Option<f32>,
    #[serde(default)]
    #[serde(rename = "guaranteedExecutionFees", skip_serializing_if = "Option::is_none", with = "serfloats")]
    pub guaranteed_execution_fees: Option<f32>,
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    #[serde(default)]
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    #[serde(rename = "createTime", skip_serializing_if = "Option::is_none", with = "serdates")]
    pub create_time: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(rename = "clientExtensions", skip_serializing_if = "Option::is_none")]
    pub client_extensions: Option<ClientExtensions>,
}
