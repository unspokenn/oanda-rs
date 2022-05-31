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

impl AccountProperties {
    pub fn new() -> AccountProperties {
        AccountProperties {
            id: None,
            mt4_account_id: None,
            tags: None,
        }
    }
    pub fn with_id(mut self, x: String) -> Self {
        self.id = Some(x);
        self
    }
    pub fn with_mt4_account_id(mut self, x: i32) -> Self {
        self.mt4_account_id = Some(x);
        self
    }
    pub fn with_tags(mut self, x: Vec<String>) -> Self {
        self.tags = Some(x);
        self
    }
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

impl Account {
    pub fn new() -> Account {
        Account {
            id: None,
            alias: None,
            currency: None,
            balance: None,
            created_by_user_id: None,
            created_time: None,
            guaranteed_stop_loss_order_mode: None,
            pl: None,
            resettable_pl: None,
            resettable_pl_time: None,
            financing: None,
            commission: None,
            guaranteed_execution_fees: None,
            margin_rate: None,
            margin_call_enter_time: None,
            margin_call_extension_count: None,
            last_margin_call_extension_time: None,
            open_trade_count: None,
            open_position_count: None,
            pending_order_count: None,
            hedging_enabled: None,
            last_order_fill_timestamp: None,
            unrealized_pl: None,
            nav: None,
            margin_used: None,
            margin_available: None,
            position_value: None,
            margin_closeout_unrealized_pl: None,
            margin_closeout_nav: None,
            margin_closeout_margin_used: None,
            margin_closeout_percent: None,
            margin_closeout_position_value: None,
            withdrawal_limit: None,
            margin_call_margin_used: None,
            margin_call_percent: None,
            last_transaction_id: None,
            trades: None,
            positions: None,
            orders: None,
        }
    }

    pub fn with_id(mut self, x: String) -> Self {
        self.id = Some(x);
        self
    }

    pub fn with_alias(mut self, x: String) -> Self {
        self.alias = Some(x);
        self
    }

    pub fn with_currency(mut self, x: String) -> Self {
        self.currency = Some(x);
        self
    }

    pub fn with_balance(mut self, x: f32) -> Self {
        self.balance = Some(x);
        self
    }

    pub fn with_created_by_user_id(mut self, x: i32) -> Self {
        self.created_by_user_id = Some(x);
        self
    }

    pub fn with_created_time(mut self, x: DateTime<Utc>) -> Self {
        self.created_time = Some(x);
        self
    }

    pub fn with_guaranteed_stop_loss_order_mode(mut self, x: String) -> Self {
        self.guaranteed_stop_loss_order_mode = Some(x);
        self
    }

    pub fn with_pl(mut self, x: f32) -> Self {
        self.pl = Some(x);
        self
    }

    pub fn with_resettable_pl(mut self, x: f32) -> Self {
        self.resettable_pl = Some(x);
        self
    }

    pub fn with_resettable_pl_time(mut self, x: DateTime<Utc>) -> Self {
        self.resettable_pl_time = Some(x);
        self
    }

    pub fn with_financing(mut self, x: f32) -> Self {
        self.financing = Some(x);
        self
    }

    pub fn with_commission(mut self, x: f32) -> Self {
        self.commission = Some(x);
        self
    }

    pub fn with_guaranteed_execution_fees(mut self, x: f32) -> Self {
        self.guaranteed_execution_fees = Some(x);
        self
    }

    pub fn with_margin_rate(mut self, x: f32) -> Self {
        self.margin_rate = Some(x);
        self
    }

    pub fn with_margin_call_enter_time(mut self, x: DateTime<Utc>) -> Self {
        self.margin_call_enter_time = Some(x);
        self
    }

    pub fn with_margin_call_extension_count(mut self, x: i32) -> Self {
        self.margin_call_extension_count = Some(x);
        self
    }

    pub fn with_last_margin_call_extension_time(mut self, x: DateTime<Utc>) -> Self {
        self.last_margin_call_extension_time = Some(x);
        self
    }

    pub fn with_open_trade_count(mut self, x: i32) -> Self {
        self.open_trade_count = Some(x);
        self
    }

    pub fn with_open_position_count(mut self, x: i32) -> Self {
        self.open_position_count = Some(x);
        self
    }

    pub fn with_pending_order_count(mut self, x: i32) -> Self {
        self.pending_order_count = Some(x);
        self
    }

    pub fn with_hedging_enabled(mut self, x: bool) -> Self {
        self.hedging_enabled = Some(x);
        self
    }

    pub fn with_last_order_fill_timestamp(mut self, x: DateTime<Utc>) -> Self {
        self.last_order_fill_timestamp = Some(x);
        self
    }

    pub fn with_unrealized_pl(mut self, x: f32) -> Self {
        self.unrealized_pl = Some(x);
        self
    }

    pub fn with_nav(mut self, x: f32) -> Self {
        self.nav = Some(x);
        self
    }

    pub fn with_margin_used(mut self, x: f32) -> Self {
        self.margin_used = Some(x);
        self
    }

    pub fn with_margin_available(mut self, x: f32) -> Self {
        self.margin_available = Some(x);
        self
    }

    pub fn with_position_value(mut self, x: f32) -> Self {
        self.position_value = Some(x);
        self
    }

    pub fn with_margin_closeout_unrealized_pl(mut self, x: f32) -> Self {
        self.margin_closeout_unrealized_pl = Some(x);
        self
    }

    pub fn with_margin_closeout_nav(mut self, x: f32) -> Self {
        self.margin_closeout_nav = Some(x);
        self
    }

    pub fn with_margin_closeout_margin_used(mut self, x: f32) -> Self {
        self.margin_closeout_margin_used = Some(x);
        self
    }

    pub fn with_margin_closeout_percent(mut self, x: f32) -> Self {
        self.margin_closeout_percent = Some(x);
        self
    }

    pub fn with_margin_closeout_position_value(mut self, x: f32) -> Self {
        self.margin_closeout_position_value = Some(x);
        self
    }

    pub fn with_withdrawal_limit(mut self, x: f32) -> Self {
        self.withdrawal_limit = Some(x);
        self
    }

    pub fn with_margin_call_margin_used(mut self, x: f32) -> Self {
        self.margin_call_margin_used = Some(x);
        self
    }

    pub fn with_margin_call_percent(mut self, x: f32) -> Self {
        self.margin_call_percent = Some(x);
        self
    }

    pub fn with_last_transaction_id(mut self, x: String) -> Self {
        self.last_transaction_id = Some(x);
        self
    }

    pub fn with_trades(mut self, x: Vec<TradeSummary>) -> Self {
        self.trades = Some(x);
        self
    }

    pub fn with_positions(mut self, x: Vec<Position>) -> Self {
        self.positions = Some(x);
        self
    }

    pub fn with_orders(mut self, x: Vec<Order>) -> Self {
        self.orders = Some(x);
        self
    }
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

impl TradeSummary {
    pub fn new() -> TradeSummary {
        TradeSummary {
            id: None,
            instrument: None,
            price: None,
            open_time: None,
            state: None,
            initial_units: None,
            initial_margin_required: None,
            current_units: None,
            realized_pl: None,
            unrealized_pl: None,
            margin_used: None,
            average_close_price: None,
            closing_transaction_i_ds: None,
            financing: None,
            close_time: None,
            client_extensions: None,
            take_profit_order_id: None,
            stop_loss_order_id: None,
            trailing_stop_loss_order_id: None,
        }
    }

    pub fn with_id(mut self, x: String) -> Self {
        self.id = Some(x);
        self
    }

    pub fn with_instrument(mut self, x: String) -> Self {
        self.instrument = Some(x);
        self
    }

    pub fn with_price(mut self, x: f32) -> Self {
        self.price = Some(x);
        self
    }

    pub fn with_open_time(mut self, x: DateTime<Utc>) -> Self {
        self.open_time = Some(x);
        self
    }

    pub fn with_state(mut self, x: String) -> Self {
        self.state = Some(x);
        self
    }

    pub fn with_initial_units(mut self, x: f32) -> Self {
        self.initial_units = Some(x);
        self
    }

    pub fn with_initial_margin_required(mut self, x: f32) -> Self {
        self.initial_margin_required = Some(x);
        self
    }

    pub fn with_current_units(mut self, x: f32) -> Self {
        self.current_units = Some(x);
        self
    }

    pub fn with_realized_pl(mut self, x: f32) -> Self {
        self.realized_pl = Some(x);
        self
    }

    pub fn with_unrealized_pl(mut self, x: f32) -> Self {
        self.unrealized_pl = Some(x);
        self
    }

    pub fn with_margin_used(mut self, x: f32) -> Self {
        self.margin_used = Some(x);
        self
    }

    pub fn with_average_close_price(mut self, x: f32) -> Self {
        self.average_close_price = Some(x);
        self
    }

    pub fn with_closing_transaction_i_ds(mut self, x: Vec<String>) -> Self {
        self.closing_transaction_i_ds = Some(x);
        self
    }

    pub fn with_financing(mut self, x: f32) -> Self {
        self.financing = Some(x);
        self
    }

    pub fn with_close_time(mut self, x: DateTime<Utc>) -> Self {
        self.close_time = Some(x);
        self
    }

    pub fn with_client_extensions(mut self, x: ClientExtensions) -> Self {
        self.client_extensions = Some(x);
        self
    }

    pub fn with_take_profit_order_id(mut self, x: String) -> Self {
        self.take_profit_order_id = Some(x);
        self
    }

    pub fn with_stop_loss_order_id(mut self, x: String) -> Self {
        self.stop_loss_order_id = Some(x);
        self
    }

    pub fn with_trailing_stop_loss_order_id(mut self, x: String) -> Self {
        self.trailing_stop_loss_order_id = Some(x);
        self
    }
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

impl ClientExtensions {
    pub fn new() -> ClientExtensions {
        ClientExtensions {
            id: None,
            tag: None,
            comment: None,
        }
    }

    pub fn with_id(mut self, x: String) -> Self {
        self.id = Some(x);
        self
    }

    pub fn with_tag(mut self, x: String) -> Self {
        self.tag = Some(x);
        self
    }

    pub fn with_comment(mut self, x: String) -> Self {
        self.comment = Some(x);
        self
    }
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

impl Position {
    pub fn new() -> Position {
        Position {
            instrument: None,
            pl: None,
            unrealized_pl: None,
            margin_used: None,
            resettable_pl: None,
            financing: None,
            commission: None,
            guaranteed_execution_fees: None,
            long: None,
            short: None,
        }
    }

    pub fn with_instrument(mut self, x: String) -> Self {
        self.instrument = Some(x);
        self
    }

    pub fn with_pl(mut self, x: f32) -> Self {
        self.pl = Some(x);
        self
    }

    pub fn with_unrealized_pl(mut self, x: f32) -> Self {
        self.unrealized_pl = Some(x);
        self
    }

    pub fn with_margin_used(mut self, x: f32) -> Self {
        self.margin_used = Some(x);
        self
    }

    pub fn with_resettable_pl(mut self, x: f32) -> Self {
        self.resettable_pl = Some(x);
        self
    }

    pub fn with_financing(mut self, x: f32) -> Self {
        self.financing = Some(x);
        self
    }

    pub fn with_commission(mut self, x: f32) -> Self {
        self.commission = Some(x);
        self
    }

    pub fn with_guaranteed_execution_fees(mut self, x: f32) -> Self {
        self.guaranteed_execution_fees = Some(x);
        self
    }

    pub fn with_long(mut self, x: PositionSide) -> Self {
        self.long = Some(x);
        self
    }

    pub fn with_short(mut self, x: PositionSide) -> Self {
        self.short = Some(x);
        self
    }
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

impl PositionSide {
    pub fn new() -> PositionSide {
        PositionSide {
            units: None,
            average_price: None,
            trade_i_ds: None,
            pl: None,
            unrealized_pl: None,
            resettable_pl: None,
            financing: None,
            guaranteed_execution_fees: None,
        }
    }

    pub fn with_units(mut self, x: f32) -> Self {
        self.units = Some(x);
        self
    }

    pub fn with_average_price(mut self, x: f32) -> Self {
        self.average_price = Some(x);
        self
    }

    pub fn with_trade_i_ds(mut self, x: Vec<String>) -> Self {
        self.trade_i_ds = Some(x);
        self
    }

    pub fn with_pl(mut self, x: f32) -> Self {
        self.pl = Some(x);
        self
    }

    pub fn with_unrealized_pl(mut self, x: f32) -> Self {
        self.unrealized_pl = Some(x);
        self
    }

    pub fn with_resettable_pl(mut self, x: f32) -> Self {
        self.resettable_pl = Some(x);
        self
    }

    pub fn with_financing(mut self, x: f32) -> Self {
        self.financing = Some(x);
        self
    }

    pub fn with_guaranteed_execution_fees(mut self, x: f32) -> Self {
        self.guaranteed_execution_fees = Some(x);
        self
    }
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

impl Order {
    pub fn new() -> Order {
        Order {
            id: None,
            create_time: None,
            state: None,
            client_extensions: None,
        }
    }

    pub fn with_id(mut self, x: String) -> Self {
        self.id = Some(x);
        self
    }

    pub fn with_create_time(mut self, x: DateTime<Utc>) -> Self {
        self.create_time = Some(x);
        self
    }

    pub fn with_state(mut self, x: String) -> Self {
        self.state = Some(x);
        self
    }

    pub fn with_client_extensions(mut self, x: ClientExtensions) -> Self {
        self.client_extensions = Some(x);
        self
    }
}
