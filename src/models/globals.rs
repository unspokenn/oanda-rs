use serde::{Deserialize, Serialize};

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

impl std::str::FromStr for CandlestickGranularity {
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

impl std::str::FromStr for WeeklyAlignment {
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
pub enum TradeState {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "CLOSE_WHEN_TRADEABLE")]
    CloseWhenTradeable,
}

impl std::str::FromStr for TradeState {
    type Err = ();
    fn from_str(s: &str) -> Result<TradeState, ()> {
        match s {
            "OPEN" => Ok(TradeState::Open),
            "CLOSED" => Ok(TradeState::Closed),
            "CLOSE_WHEN_TRADEABLE" => Ok(TradeState::CloseWhenTradeable),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TradeState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TradeStateFilter {
    #[serde(rename = "OPEN")]
    Open,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "CLOSE_WHEN_TRADEABLE")]
    CloseWhenTradeable,
    #[serde(rename = "ALL")]
    All,
}

impl std::str::FromStr for TradeStateFilter {
    type Err = ();
    fn from_str(s: &str) -> Result<TradeStateFilter, ()> {
        match s {
            "OPEN" => Ok(TradeStateFilter::Open),
            "CLOSED" => Ok(TradeStateFilter::Closed),
            "CLOSE_WHEN_TRADEABLE" => Ok(TradeStateFilter::CloseWhenTradeable),
            "ALL" => Ok(TradeStateFilter::All),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TradeStateFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TradePL {
    #[serde(rename = "POSITIVE")]
    Positive,
    #[serde(rename = "NEGATIVE")]
    Negative,
    #[serde(rename = "ZERO")]
    Zero,
}

impl std::str::FromStr for TradePL {
    type Err = ();
    fn from_str(s: &str) -> Result<TradePL, ()> {
        match s {
            "POSITIVE" => Ok(TradePL::Positive),
            "NEGATIVE" => Ok(TradePL::Negative),
            "ZERO" => Ok(TradePL::Zero),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TradePL {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum InstrumentType {
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "METAL")]
    Metal,
}

impl std::str::FromStr for InstrumentType {
    type Err = ();
    fn from_str(s: &str) -> Result<InstrumentType, ()> {
        match s {
            "CURRENCY" => Ok(InstrumentType::Currency),
            "CFD" => Ok(InstrumentType::Cfd),
            "METAL" => Ok(InstrumentType::Metal),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum AcceptDatetimeFormat {
    #[serde(rename = "UNIX")]
    Unix,
    #[serde(rename = "RFC3339")]
    Rfc3339,
}

impl std::str::FromStr for AcceptDatetimeFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<AcceptDatetimeFormat, ()> {
        match s {
            "UNIX" => Ok(AcceptDatetimeFormat::Unix),
            "RFC3339" => Ok(AcceptDatetimeFormat::Rfc3339),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for AcceptDatetimeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum GuaranteedStopLossOrderModeForInstrument {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ALLOWED")]
    Allowed,
    #[serde(rename = "REQUIRED")]
    Required,
}

impl std::str::FromStr for GuaranteedStopLossOrderModeForInstrument {
    type Err = ();
    fn from_str(s: &str) -> Result<GuaranteedStopLossOrderModeForInstrument, ()> {
        match s {
            "DISABLED" => Ok(GuaranteedStopLossOrderModeForInstrument::Disabled),
            "ALLOWED" => Ok(GuaranteedStopLossOrderModeForInstrument::Allowed),
            "REQUIRED" => Ok(GuaranteedStopLossOrderModeForInstrument::Required),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for GuaranteedStopLossOrderModeForInstrument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum DayOfWeek {
    #[serde(rename = "SUNDAY")]
    Sunday,
    #[serde(rename = "MONDAY")]
    Monday,
    #[serde(rename = "TUESDAY")]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    Thursday,
    #[serde(rename = "FRIDAY")]
    Friday,
    #[serde(rename = "SATURDAY")]
    Saturday,
}

impl std::str::FromStr for DayOfWeek {
    type Err = ();
    fn from_str(s: &str) -> Result<DayOfWeek, ()> {
        match s {
            "SUNDAY" => Ok(DayOfWeek::Sunday),
            "MONDAY" => Ok(DayOfWeek::Monday),
            "TUESDAY" => Ok(DayOfWeek::Tuesday),
            "WEDNESDAY" => Ok(DayOfWeek::Wednesday),
            "THURSDAY" => Ok(DayOfWeek::Thursday),
            "FRIDAY" => Ok(DayOfWeek::Friday),
            "SATURDAY" => Ok(DayOfWeek::Saturday),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "LONG")]
    Long,
    #[serde(rename = "SHORT")]
    Short,
}

impl std::str::FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "LONG" => Ok(Direction::Long),
            "SHORT" => Ok(Direction::Short),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum GuaranteedStopLossOrderMode {
    #[serde(rename = "DISABLED")]
    Disabled,
    #[serde(rename = "ALLOWED")]
    Allowed,
    #[serde(rename = "REQUIRED")]
    Required,
}

impl std::str::FromStr for GuaranteedStopLossOrderMode {
    type Err = ();
    fn from_str(s: &str) -> Result<GuaranteedStopLossOrderMode, ()> {
        match s {
            "DISABLED" => Ok(GuaranteedStopLossOrderMode::Disabled),
            "ALLOWED" => Ok(GuaranteedStopLossOrderMode::Allowed),
            "REQUIRED" => Ok(GuaranteedStopLossOrderMode::Required),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for GuaranteedStopLossOrderMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountFinancingMode {
    #[serde(rename = "NO_FINANCING")]
    NoFinancing,
    #[serde(rename = "SECOND_BY_SECOND")]
    SecondBySecond,
    #[serde(rename = "DAILY")]
    Daily,
}

impl std::str::FromStr for AccountFinancingMode {
    type Err = ();
    fn from_str(s: &str) -> Result<AccountFinancingMode, ()> {
        match s {
            "NO_FINANCING" => Ok(AccountFinancingMode::NoFinancing),
            "SECOND_BY_SECOND" => Ok(AccountFinancingMode::SecondBySecond),
            "DAILY" => Ok(AccountFinancingMode::Daily),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for AccountFinancingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum PositionAggregationMode {
    #[serde(rename = "ABSOLUTE_SUM")]
    AbsoluteSum,
    #[serde(rename = "MAXIMAL_SIDE")]
    MaximalSide,
    #[serde(rename = "NET_SUM")]
    NetSum,
}

impl std::str::FromStr for PositionAggregationMode {
    type Err = ();
    fn from_str(s: &str) -> Result<PositionAggregationMode, ()> {
        match s {
            "ABSOLUTE_SUM" => Ok(PositionAggregationMode::AbsoluteSum),
            "MAXIMAL_SIDE" => Ok(PositionAggregationMode::MaximalSide),
            "NET_SUM" => Ok(PositionAggregationMode::NetSum),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for PositionAggregationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "CLOSE")]
    Close,
    #[serde(rename = "REOPEN")]
    Reopen,
    #[serde(rename = "CLIENT_CONFIGURE")]
    ClientConfigure,
    #[serde(rename = "CLIENT_CONFIGURE_REJECT")]
    ClientConfigureReject,
    #[serde(rename = "TRANSFER_FUNDS")]
    TransferFunds,
    #[serde(rename = "TRANSFER_FUNDS_REJECT")]
    TransferFundsReject,
    #[serde(rename = "MARKET_ORDER")]
    MarketOrder,
    #[serde(rename = "MARKET_ORDER_REJECT")]
    MarketOrderReject,
    #[serde(rename = "FIXED_PRICE_ORDER")]
    FixedPriceOrder,
    #[serde(rename = "LIMIT_ORDER")]
    LimitOrder,
    #[serde(rename = "LIMIT_ORDER_REJECT")]
    LimitOrderReject,
    #[serde(rename = "STOP_ORDER")]
    StopOrder,
    #[serde(rename = "STOP_ORDER_REJECT")]
    StopOrderReject,
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER")]
    MarketIfTouchedOrder,
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER_REJECT")]
    MarketIfTouchedOrderReject,
    #[serde(rename = "TAKE_PROFIT_ORDER")]
    TakeProfitOrder,
    #[serde(rename = "TAKE_PROFIT_ORDER_REJECT")]
    TakeProfitOrderReject,
    #[serde(rename = "STOP_LOSS_ORDER")]
    StopLossOrder,
    #[serde(rename = "STOP_LOSS_ORDER_REJECT")]
    StopLossOrderReject,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER")]
    TrailingStopLossOrder,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER_REJECT")]
    TrailingStopLossOrderReject,
    #[serde(rename = "ORDER_FILL")]
    OrderFill,
    #[serde(rename = "ORDER_CANCEL")]
    OrderCancel,
    #[serde(rename = "ORDER_CANCEL_REJECT")]
    OrderCancelReject,
    #[serde(rename = "ORDER_CLIENT_EXTENSIONS_MODIFY")]
    OrderClientExtensionsModify,
    #[serde(rename = "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    OrderClientExtensionsModifyReject,
    #[serde(rename = "TRADE_CLIENT_EXTENSIONS_MODIFY")]
    TradeClientExtensionsModify,
    #[serde(rename = "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    TradeClientExtensionsModifyReject,
    #[serde(rename = "MARGIN_CALL_ENTER")]
    MarginCallEnter,
    #[serde(rename = "MARGIN_CALL_EXTEND")]
    MarginCallExtend,
    #[serde(rename = "MARGIN_CALL_EXIT")]
    MarginCallExit,
    #[serde(rename = "DELAYED_TRADE_CLOSURE")]
    DelayedTradeClosure,
    #[serde(rename = "DAILY_FINANCING")]
    DailyFinancing,
    #[serde(rename = "RESET_RESETTABLE_PL")]
    ResetResettablePl,
}

impl std::str::FromStr for TransactionType {
    type Err = ();
    fn from_str(s: &str) -> Result<TransactionType, ()> {
        match s {
            "CREATE" => Ok(TransactionType::Create),
            "CLOSE" => Ok(TransactionType::Close),
            "REOPEN" => Ok(TransactionType::Reopen),
            "CLIENT_CONFIGURE" => Ok(TransactionType::ClientConfigure),
            "CLIENT_CONFIGURE_REJECT" => Ok(TransactionType::ClientConfigureReject),
            "TRANSFER_FUNDS" => Ok(TransactionType::TransferFunds),
            "TRANSFER_FUNDS_REJECT" => Ok(TransactionType::TransferFundsReject),
            "MARKET_ORDER" => Ok(TransactionType::MarketOrder),
            "MARKET_ORDER_REJECT" => Ok(TransactionType::MarketOrderReject),
            "FIXED_PRICE_ORDER" => Ok(TransactionType::FixedPriceOrder),
            "LIMIT_ORDER" => Ok(TransactionType::LimitOrder),
            "LIMIT_ORDER_REJECT" => Ok(TransactionType::LimitOrderReject),
            "STOP_ORDER" => Ok(TransactionType::StopOrder),
            "STOP_ORDER_REJECT" => Ok(TransactionType::StopOrderReject),
            "MARKET_IF_TOUCHED_ORDER" => Ok(TransactionType::MarketIfTouchedOrder),
            "MARKET_IF_TOUCHED_ORDER_REJECT" => Ok(TransactionType::MarketIfTouchedOrderReject),
            "TAKE_PROFIT_ORDER" => Ok(TransactionType::TakeProfitOrder),
            "TAKE_PROFIT_ORDER_REJECT" => Ok(TransactionType::TakeProfitOrderReject),
            "STOP_LOSS_ORDER" => Ok(TransactionType::StopLossOrder),
            "STOP_LOSS_ORDER_REJECT" => Ok(TransactionType::StopLossOrderReject),
            "TRAILING_STOP_LOSS_ORDER" => Ok(TransactionType::TrailingStopLossOrder),
            "TRAILING_STOP_LOSS_ORDER_REJECT" => Ok(TransactionType::TrailingStopLossOrderReject),
            "ORDER_FILL" => Ok(TransactionType::OrderFill),
            "ORDER_CANCEL" => Ok(TransactionType::OrderCancel),
            "ORDER_CANCEL_REJECT" => Ok(TransactionType::OrderCancelReject),
            "ORDER_CLIENT_EXTENSIONS_MODIFY" => Ok(TransactionType::OrderClientExtensionsModify),
            "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT" => {
                Ok(TransactionType::OrderClientExtensionsModifyReject)
            }
            "TRADE_CLIENT_EXTENSIONS_MODIFY" => Ok(TransactionType::TradeClientExtensionsModify),
            "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT" => {
                Ok(TransactionType::TradeClientExtensionsModifyReject)
            }
            "MARGIN_CALL_ENTER" => Ok(TransactionType::MarginCallEnter),
            "MARGIN_CALL_EXTEND" => Ok(TransactionType::MarginCallExtend),
            "MARGIN_CALL_EXIT" => Ok(TransactionType::MarginCallExit),
            "DELAYED_TRADE_CLOSURE" => Ok(TransactionType::DelayedTradeClosure),
            "DAILY_FINANCING" => Ok(TransactionType::DailyFinancing),
            "RESET_RESETTABLE_PL" => Ok(TransactionType::ResetResettablePl),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum FundingReason {
    #[serde(rename = "CLIENT_FUNDING")]
    ClientFunding,
    #[serde(rename = "ACCOUNT_TRANSFER")]
    AccountTransfer,
    #[serde(rename = "DIVISION_MIGRATION")]
    DivisionMigration,
    #[serde(rename = "SITE_MIGRATION")]
    SiteMigration,
    #[serde(rename = "ADJUSTMENT")]
    Adjustment,
}

impl std::str::FromStr for FundingReason {
    type Err = ();
    fn from_str(s: &str) -> Result<FundingReason, ()> {
        match s {
            "CLIENT_FUNDING" => Ok(FundingReason::ClientFunding),
            "ACCOUNT_TRANSFER" => Ok(FundingReason::AccountTransfer),
            "DIVISION_MIGRATION" => Ok(FundingReason::DivisionMigration),
            "SITE_MIGRATION" => Ok(FundingReason::SiteMigration),
            "ADJUSTMENT" => Ok(FundingReason::Adjustment),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for FundingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum MarketOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "TRADE_CLOSE")]
    TradeClose,
    #[serde(rename = "POSITION_CLOSEOUT")]
    PositionCloseout,
    #[serde(rename = "MARGIN_CLOSEOUT")]
    MarginCloseout,
    #[serde(rename = "DELAYED_TRADE_CLOSE")]
    DelayedTradeClose,
}

impl std::str::FromStr for MarketOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<MarketOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(MarketOrderReason::ClientOrder),
            "TRADE_CLOSE" => Ok(MarketOrderReason::TradeClose),
            "POSITION_CLOSEOUT" => Ok(MarketOrderReason::PositionCloseout),
            "MARGIN_CLOSEOUT" => Ok(MarketOrderReason::MarginCloseout),
            "DELAYED_TRADE_CLOSE" => Ok(MarketOrderReason::DelayedTradeClose),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for MarketOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum FixedPriceOrderReason {
    #[serde(rename = "PLATFORM_ACCOUNT_MIGRATION")]
    PlatformAccountMigration,
}

impl std::str::FromStr for FixedPriceOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<FixedPriceOrderReason, ()> {
        match s {
            "PLATFORM_ACCOUNT_MIGRATION" => Ok(FixedPriceOrderReason::PlatformAccountMigration),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for FixedPriceOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum LimitOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "REPLACEMENT")]
    Replacement,
}

impl std::str::FromStr for LimitOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<LimitOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(LimitOrderReason::ClientOrder),
            "REPLACEMENT" => Ok(LimitOrderReason::Replacement),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for LimitOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum StopOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "REPLACEMENT")]
    Replacement,
}

impl std::str::FromStr for StopOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<StopOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(StopOrderReason::ClientOrder),
            "REPLACEMENT" => Ok(StopOrderReason::Replacement),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for StopOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum MarketIfTouchedOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "REPLACEMENT")]
    Replacement,
}

impl std::str::FromStr for MarketIfTouchedOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<MarketIfTouchedOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(MarketIfTouchedOrderReason::ClientOrder),
            "REPLACEMENT" => Ok(MarketIfTouchedOrderReason::Replacement),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for MarketIfTouchedOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TakeProfitOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "REPLACEMENT")]
    Replacement,
    #[serde(rename = "ON_FILL")]
    OnFill,
}

impl std::str::FromStr for TakeProfitOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<TakeProfitOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(TakeProfitOrderReason::ClientOrder),
            "REPLACEMENT" => Ok(TakeProfitOrderReason::Replacement),
            "ON_FILL" => Ok(TakeProfitOrderReason::OnFill),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TakeProfitOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum StopLossOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "REPLACEMENT")]
    Replacement,
    #[serde(rename = "ON_FILL")]
    OnFill,
}

impl std::str::FromStr for StopLossOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<StopLossOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(StopLossOrderReason::ClientOrder),
            "REPLACEMENT" => Ok(StopLossOrderReason::Replacement),
            "ON_FILL" => Ok(StopLossOrderReason::OnFill),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for StopLossOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TrailingStopLossOrderReason {
    #[serde(rename = "CLIENT_ORDER")]
    ClientOrder,
    #[serde(rename = "REPLACEMENT")]
    Replacement,
    #[serde(rename = "ON_FILL")]
    OnFill,
}

impl std::str::FromStr for TrailingStopLossOrderReason {
    type Err = ();
    fn from_str(s: &str) -> Result<TrailingStopLossOrderReason, ()> {
        match s {
            "CLIENT_ORDER" => Ok(TrailingStopLossOrderReason::ClientOrder),
            "REPLACEMENT" => Ok(TrailingStopLossOrderReason::Replacement),
            "ON_FILL" => Ok(TrailingStopLossOrderReason::OnFill),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TrailingStopLossOrderReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderFillReason {
    #[serde(rename = "LIMIT_ORDER")]
    LimitOrder,
    #[serde(rename = "STOP_ORDER")]
    StopOrder,
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER")]
    MarketIfTouchedOrder,
    #[serde(rename = "TAKE_PROFIT_ORDER")]
    TakeProfitOrder,
    #[serde(rename = "STOP_LOSS_ORDER")]
    StopLossOrder,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER")]
    TrailingStopLossOrder,
    #[serde(rename = "MARKET_ORDER")]
    MarketOrder,
    #[serde(rename = "MARKET_ORDER_TRADE_CLOSE")]
    MarketOrderTradeClose,
    #[serde(rename = "MARKET_ORDER_POSITION_CLOSEOUT")]
    MarketOrderPositionCloseout,
    #[serde(rename = "MARKET_ORDER_MARGIN_CLOSEOUT")]
    MarketOrderMarginCloseout,
    #[serde(rename = "MARKET_ORDER_DELAYED_TRADE_CLOSE")]
    MarketOrderDelayedTradeClose,
}

impl std::str::FromStr for OrderFillReason {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderFillReason, ()> {
        match s {
            "LIMIT_ORDER" => Ok(OrderFillReason::LimitOrder),
            "STOP_ORDER" => Ok(OrderFillReason::StopOrder),
            "MARKET_IF_TOUCHED_ORDER" => Ok(OrderFillReason::MarketIfTouchedOrder),
            "TAKE_PROFIT_ORDER" => Ok(OrderFillReason::TakeProfitOrder),
            "STOP_LOSS_ORDER" => Ok(OrderFillReason::StopLossOrder),
            "TRAILING_STOP_LOSS_ORDER" => Ok(OrderFillReason::TrailingStopLossOrder),
            "MARKET_ORDER" => Ok(OrderFillReason::MarketOrder),
            "MARKET_ORDER_TRADE_CLOSE" => Ok(OrderFillReason::MarketOrderTradeClose),
            "MARKET_ORDER_POSITION_CLOSEOUT" => Ok(OrderFillReason::MarketOrderPositionCloseout),
            "MARKET_ORDER_MARGIN_CLOSEOUT" => Ok(OrderFillReason::MarketOrderMarginCloseout),
            "MARKET_ORDER_DELAYED_TRADE_CLOSE" => Ok(OrderFillReason::MarketOrderDelayedTradeClose),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderFillReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderCancelReason {
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[serde(rename = "ACCOUNT_LOCKED")]
    AccountLocked,
    #[serde(rename = "ACCOUNT_NEW_POSITIONS_LOCKED")]
    AccountNewPositionsLocked,
    #[serde(rename = "ACCOUNT_ORDER_CREATION_LOCKED")]
    AccountOrderCreationLocked,
    #[serde(rename = "ACCOUNT_ORDER_FILL_LOCKED")]
    AccountOrderFillLocked,
    #[serde(rename = "CLIENT_REQUEST")]
    ClientRequest,
    #[serde(rename = "MIGRATION")]
    Migration,
    #[serde(rename = "MARKET_HALTED")]
    MarketHalted,
    #[serde(rename = "LINKED_TRADE_CLOSED")]
    LinkedTradeClosed,
    #[serde(rename = "TIME_IN_FORCE_EXPIRED")]
    TimeInForceExpired,
    #[serde(rename = "INSUFFICIENT_MARGIN")]
    InsufficientMargin,
    #[serde(rename = "FIFO_VIOLATION")]
    FifoViolation,
    #[serde(rename = "BOUNDS_VIOLATION")]
    BoundsViolation,
    #[serde(rename = "CLIENT_REQUEST_REPLACED")]
    ClientRequestReplaced,
    #[serde(rename = "INSUFFICIENT_LIQUIDITY")]
    InsufficientLiquidity,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST")]
    TakeProfitOnFillGtdTimestampInPast,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_LOSS")]
    TakeProfitOnFillLoss,
    #[serde(rename = "LOSING_TAKE_PROFIT")]
    LosingTakeProfit,
    #[serde(rename = "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST")]
    StopLossOnFillGtdTimestampInPast,
    #[serde(rename = "STOP_LOSS_ON_FILL_LOSS")]
    StopLossOnFillLoss,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED")]
    StopLossOnFillPriceDistanceMaximumExceeded,
    #[serde(rename = "STOP_LOSS_ON_FILL_REQUIRED")]
    StopLossOnFillRequired,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED")]
    StopLossOnFillGuaranteedRequired,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED")]
    StopLossOnFillGuaranteedNotAllowed,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET")]
    StopLossOnFillGuaranteedMinimumDistanceNotMet,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED")]
    StopLossOnFillGuaranteedLevelRestrictionExceeded,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED")]
    StopLossOnFillGuaranteedHedgingNotAllowed,
    #[serde(rename = "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID")]
    StopLossOnFillTimeInForceInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID")]
    StopLossOnFillTriggerConditionInvalid,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED")]
    TakeProfitOnFillPriceDistanceMaximumExceeded,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST")]
    TrailingStopLossOnFillGtdTimestampInPast,
    #[serde(rename = "CLIENT_TRADE_ID_ALREADY_EXISTS")]
    ClientTradeIdAlreadyExists,
    #[serde(rename = "POSITION_CLOSEOUT_FAILED")]
    PositionCloseoutFailed,
    #[serde(rename = "OPEN_TRADES_ALLOWED_EXCEEDED")]
    OpenTradesAllowedExceeded,
    #[serde(rename = "PENDING_ORDERS_ALLOWED_EXCEEDED")]
    PendingOrdersAllowedExceeded,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS")]
    TakeProfitOnFillClientOrderIdAlreadyExists,
    #[serde(rename = "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS")]
    StopLossOnFillClientOrderIdAlreadyExists,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS")]
    TrailingStopLossOnFillClientOrderIdAlreadyExists,
    #[serde(rename = "POSITION_SIZE_EXCEEDED")]
    PositionSizeExceeded,
    #[serde(rename = "HEDGING_GSLO_VIOLATION")]
    HedgingGsloViolation,
    #[serde(rename = "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED")]
    AccountPositionValueLimitExceeded,
    #[serde(rename = "INSTRUMENT_BID_REDUCE_ONLY")]
    InstrumentBidReduceOnly,
    #[serde(rename = "INSTRUMENT_ASK_REDUCE_ONLY")]
    InstrumentAskReduceOnly,
    #[serde(rename = "INSTRUMENT_BID_HALTED")]
    InstrumentBidHalted,
    #[serde(rename = "INSTRUMENT_ASK_HALTED")]
    InstrumentAskHalted,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED")]
    StopLossOnFillGuaranteedBidHalted,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED")]
    StopLossOnFillGuaranteedAskHalted,
}

impl std::str::FromStr for OrderCancelReason {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderCancelReason, ()> {
        match s {
            "INTERNAL_SERVER_ERROR" => Ok(OrderCancelReason::InternalServerError),
            "ACCOUNT_LOCKED" => Ok(OrderCancelReason::AccountLocked),
            "ACCOUNT_NEW_POSITIONS_LOCKED" => Ok(OrderCancelReason::AccountNewPositionsLocked),
            "ACCOUNT_ORDER_CREATION_LOCKED" => Ok(OrderCancelReason::AccountOrderCreationLocked),
            "ACCOUNT_ORDER_FILL_LOCKED" => Ok(OrderCancelReason::AccountOrderFillLocked),
            "CLIENT_REQUEST" => Ok(OrderCancelReason::ClientRequest),
            "MIGRATION" => Ok(OrderCancelReason::Migration),
            "MARKET_HALTED" => Ok(OrderCancelReason::MarketHalted),
            "LINKED_TRADE_CLOSED" => Ok(OrderCancelReason::LinkedTradeClosed),
            "TIME_IN_FORCE_EXPIRED" => Ok(OrderCancelReason::TimeInForceExpired),
            "INSUFFICIENT_MARGIN" => Ok(OrderCancelReason::InsufficientMargin),
            "FIFO_VIOLATION" => Ok(OrderCancelReason::FifoViolation),
            "BOUNDS_VIOLATION" => Ok(OrderCancelReason::BoundsViolation),
            "CLIENT_REQUEST_REPLACED" => Ok(OrderCancelReason::ClientRequestReplaced),
            "INSUFFICIENT_LIQUIDITY" => Ok(OrderCancelReason::InsufficientLiquidity),
            "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Ok(OrderCancelReason::TakeProfitOnFillGtdTimestampInPast)
            }
            "TAKE_PROFIT_ON_FILL_LOSS" => Ok(OrderCancelReason::TakeProfitOnFillLoss),
            "LOSING_TAKE_PROFIT" => Ok(OrderCancelReason::LosingTakeProfit),
            "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Ok(OrderCancelReason::StopLossOnFillGtdTimestampInPast)
            }
            "STOP_LOSS_ON_FILL_LOSS" => Ok(OrderCancelReason::StopLossOnFillLoss),
            "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Ok(OrderCancelReason::StopLossOnFillPriceDistanceMaximumExceeded)
            }
            "STOP_LOSS_ON_FILL_REQUIRED" => Ok(OrderCancelReason::StopLossOnFillRequired),
            "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedRequired)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedNotAllowed)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedMinimumDistanceNotMet)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedLevelRestrictionExceeded)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_HEDGING_NOT_ALLOWED" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedHedgingNotAllowed)
            }
            "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Ok(OrderCancelReason::StopLossOnFillTimeInForceInvalid)
            }
            "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Ok(OrderCancelReason::StopLossOnFillTriggerConditionInvalid)
            }
            "TAKE_PROFIT_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Ok(OrderCancelReason::TakeProfitOnFillPriceDistanceMaximumExceeded)
            }
            "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Ok(OrderCancelReason::TrailingStopLossOnFillGtdTimestampInPast)
            }
            "CLIENT_TRADE_ID_ALREADY_EXISTS" => Ok(OrderCancelReason::ClientTradeIdAlreadyExists),
            "POSITION_CLOSEOUT_FAILED" => Ok(OrderCancelReason::PositionCloseoutFailed),
            "OPEN_TRADES_ALLOWED_EXCEEDED" => Ok(OrderCancelReason::OpenTradesAllowedExceeded),
            "PENDING_ORDERS_ALLOWED_EXCEEDED" => {
                Ok(OrderCancelReason::PendingOrdersAllowedExceeded)
            }
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Ok(OrderCancelReason::TakeProfitOnFillClientOrderIdAlreadyExists)
            }
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Ok(OrderCancelReason::StopLossOnFillClientOrderIdAlreadyExists)
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Ok(OrderCancelReason::TrailingStopLossOnFillClientOrderIdAlreadyExists)
            }
            "POSITION_SIZE_EXCEEDED" => Ok(OrderCancelReason::PositionSizeExceeded),
            "HEDGING_GSLO_VIOLATION" => Ok(OrderCancelReason::HedgingGsloViolation),
            "ACCOUNT_POSITION_VALUE_LIMIT_EXCEEDED" => {
                Ok(OrderCancelReason::AccountPositionValueLimitExceeded)
            }
            "INSTRUMENT_BID_REDUCE_ONLY" => Ok(OrderCancelReason::InstrumentBidReduceOnly),
            "INSTRUMENT_ASK_REDUCE_ONLY" => Ok(OrderCancelReason::InstrumentAskReduceOnly),
            "INSTRUMENT_BID_HALTED" => Ok(OrderCancelReason::InstrumentBidHalted),
            "INSTRUMENT_ASK_HALTED" => Ok(OrderCancelReason::InstrumentAskHalted),
            "STOP_LOSS_ON_FILL_GUARANTEED_BID_HALTED" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedBidHalted)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_ASK_HALTED" => {
                Ok(OrderCancelReason::StopLossOnFillGuaranteedAskHalted)
            }
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderCancelReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum MarketOrderMarginCloseoutReason {
    #[serde(rename = "MARGIN_CHECK_VIOLATION")]
    MarginCheckViolation,
    #[serde(rename = "REGULATORY_MARGIN_CALL_VIOLATION")]
    RegulatoryMarginCallViolation,
    #[serde(rename = "REGULATORY_MARGIN_CHECK_VIOLATION")]
    RegulatoryMarginCheckViolation,
}

impl std::str::FromStr for MarketOrderMarginCloseoutReason {
    type Err = ();
    fn from_str(s: &str) -> Result<MarketOrderMarginCloseoutReason, ()> {
        match s {
            "MARGIN_CHECK_VIOLATION" => Ok(MarketOrderMarginCloseoutReason::MarginCheckViolation),
            "REGULATORY_MARGIN_CALL_VIOLATION" => {
                Ok(MarketOrderMarginCloseoutReason::RegulatoryMarginCallViolation)
            }
            "REGULATORY_MARGIN_CHECK_VIOLATION" => {
                Ok(MarketOrderMarginCloseoutReason::RegulatoryMarginCheckViolation)
            }
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for MarketOrderMarginCloseoutReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionRejectReason {
    #[serde(rename = "INTERNAL_SERVER_ERROR")]
    InternalServerError,
    #[serde(rename = "INSTRUMENT_PRICE_UNKNOWN")]
    InstrumentPriceUnknown,
    #[serde(rename = "ACCOUNT_NOT_ACTIVE")]
    AccountNotActive,
    #[serde(rename = "ACCOUNT_LOCKED")]
    AccountLocked,
    #[serde(rename = "ACCOUNT_ORDER_CREATION_LOCKED")]
    AccountOrderCreationLocked,
    #[serde(rename = "ACCOUNT_CONFIGURATION_LOCKED")]
    AccountConfigurationLocked,
    #[serde(rename = "ACCOUNT_DEPOSIT_LOCKED")]
    AccountDepositLocked,
    #[serde(rename = "ACCOUNT_WITHDRAWAL_LOCKED")]
    AccountWithdrawalLocked,
    #[serde(rename = "ACCOUNT_ORDER_CANCEL_LOCKED")]
    AccountOrderCancelLocked,
    #[serde(rename = "INSTRUMENT_NOT_TRADEABLE")]
    InstrumentNotTradeable,
    #[serde(rename = "PENDING_ORDERS_ALLOWED_EXCEEDED")]
    PendingOrdersAllowedExceeded,
    #[serde(rename = "ORDER_ID_UNSPECIFIED")]
    OrderIdUnspecified,
    #[serde(rename = "ORDER_DOESNT_EXIST")]
    OrderDoesntExist,
    #[serde(rename = "ORDER_IDENTIFIER_INCONSISTENCY")]
    OrderIdentifierInconsistency,
    #[serde(rename = "TRADE_ID_UNSPECIFIED")]
    TradeIdUnspecified,
    #[serde(rename = "TRADE_DOESNT_EXIST")]
    TradeDoesntExist,
    #[serde(rename = "TRADE_IDENTIFIER_INCONSISTENCY")]
    TradeIdentifierInconsistency,
    #[serde(rename = "INSUFFICIENT_MARGIN")]
    InsufficientMargin,
    #[serde(rename = "INSTRUMENT_MISSING")]
    InstrumentMissing,
    #[serde(rename = "INSTRUMENT_UNKNOWN")]
    InstrumentUnknown,
    #[serde(rename = "UNITS_MISSING")]
    UnitsMissing,
    #[serde(rename = "UNITS_INVALID")]
    UnitsInvalid,
    #[serde(rename = "UNITS_PRECISION_EXCEEDED")]
    UnitsPrecisionExceeded,
    #[serde(rename = "UNITS_LIMIT_EXCEEDED")]
    UnitsLimitExceeded,
    #[serde(rename = "UNITS_MIMIMUM_NOT_MET")]
    UnitsMimimumNotMet,
    #[serde(rename = "PRICE_MISSING")]
    PriceMissing,
    #[serde(rename = "PRICE_INVALID")]
    PriceInvalid,
    #[serde(rename = "PRICE_PRECISION_EXCEEDED")]
    PricePrecisionExceeded,
    #[serde(rename = "PRICE_DISTANCE_MISSING")]
    PriceDistanceMissing,
    #[serde(rename = "PRICE_DISTANCE_INVALID")]
    PriceDistanceInvalid,
    #[serde(rename = "PRICE_DISTANCE_PRECISION_EXCEEDED")]
    PriceDistancePrecisionExceeded,
    #[serde(rename = "PRICE_DISTANCE_MAXIMUM_EXCEEDED")]
    PriceDistanceMaximumExceeded,
    #[serde(rename = "PRICE_DISTANCE_MINIMUM_NOT_MET")]
    PriceDistanceMinimumNotMet,
    #[serde(rename = "TIME_IN_FORCE_MISSING")]
    TimeInForceMissing,
    #[serde(rename = "TIME_IN_FORCE_INVALID")]
    TimeInForceInvalid,
    #[serde(rename = "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING")]
    TimeInForceGtdTimestampMissing,
    #[serde(rename = "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST")]
    TimeInForceGtdTimestampInPast,
    #[serde(rename = "PRICE_BOUND_INVALID")]
    PriceBoundInvalid,
    #[serde(rename = "PRICE_BOUND_PRECISION_EXCEEDED")]
    PriceBoundPrecisionExceeded,
    #[serde(rename = "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS")]
    OrdersOnFillDuplicateClientOrderIds,
    #[serde(rename = "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED")]
    TradeOnFillClientExtensionsNotSupported,
    #[serde(rename = "CLIENT_ORDER_ID_INVALID")]
    ClientOrderIdInvalid,
    #[serde(rename = "CLIENT_ORDER_ID_ALREADY_EXISTS")]
    ClientOrderIdAlreadyExists,
    #[serde(rename = "CLIENT_ORDER_TAG_INVALID")]
    ClientOrderTagInvalid,
    #[serde(rename = "CLIENT_ORDER_COMMENT_INVALID")]
    ClientOrderCommentInvalid,
    #[serde(rename = "CLIENT_TRADE_ID_INVALID")]
    ClientTradeIdInvalid,
    #[serde(rename = "CLIENT_TRADE_ID_ALREADY_EXISTS")]
    ClientTradeIdAlreadyExists,
    #[serde(rename = "CLIENT_TRADE_TAG_INVALID")]
    ClientTradeTagInvalid,
    #[serde(rename = "CLIENT_TRADE_COMMENT_INVALID")]
    ClientTradeCommentInvalid,
    #[serde(rename = "ORDER_FILL_POSITION_ACTION_MISSING")]
    OrderFillPositionActionMissing,
    #[serde(rename = "ORDER_FILL_POSITION_ACTION_INVALID")]
    OrderFillPositionActionInvalid,
    #[serde(rename = "TRIGGER_CONDITION_MISSING")]
    TriggerConditionMissing,
    #[serde(rename = "TRIGGER_CONDITION_INVALID")]
    TriggerConditionInvalid,
    #[serde(rename = "ORDER_PARTIAL_FILL_OPTION_MISSING")]
    OrderPartialFillOptionMissing,
    #[serde(rename = "ORDER_PARTIAL_FILL_OPTION_INVALID")]
    OrderPartialFillOptionInvalid,
    #[serde(rename = "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL")]
    InvalidReissueImmediatePartialFill,
    #[serde(rename = "TAKE_PROFIT_ORDER_ALREADY_EXISTS")]
    TakeProfitOrderAlreadyExists,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_PRICE_MISSING")]
    TakeProfitOnFillPriceMissing,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_PRICE_INVALID")]
    TakeProfitOnFillPriceInvalid,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED")]
    TakeProfitOnFillPricePrecisionExceeded,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING")]
    TakeProfitOnFillTimeInForceMissing,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID")]
    TakeProfitOnFillTimeInForceInvalid,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING")]
    TakeProfitOnFillGtdTimestampMissing,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST")]
    TakeProfitOnFillGtdTimestampInPast,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID")]
    TakeProfitOnFillClientOrderIdInvalid,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID")]
    TakeProfitOnFillClientOrderTagInvalid,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID")]
    TakeProfitOnFillClientOrderCommentInvalid,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING")]
    TakeProfitOnFillTriggerConditionMissing,
    #[serde(rename = "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID")]
    TakeProfitOnFillTriggerConditionInvalid,
    #[serde(rename = "STOP_LOSS_ORDER_ALREADY_EXISTS")]
    StopLossOrderAlreadyExists,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_REQUIRED")]
    StopLossOrderGuaranteedRequired,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD")]
    StopLossOrderGuaranteedPriceWithinSpread,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED")]
    StopLossOrderGuaranteedNotAllowed,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION")]
    StopLossOrderGuaranteedHaltedCreateViolation,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION")]
    StopLossOrderGuaranteedHaltedTightenViolation,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED")]
    StopLossOrderGuaranteedHedgingNotAllowed,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET")]
    StopLossOrderGuaranteedMinimumDistanceNotMet,
    #[serde(rename = "STOP_LOSS_ORDER_NOT_CANCELABLE")]
    StopLossOrderNotCancelable,
    #[serde(rename = "STOP_LOSS_ORDER_NOT_REPLACEABLE")]
    StopLossOrderNotReplaceable,
    #[serde(rename = "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED")]
    StopLossOrderGuaranteedLevelRestrictionExceeded,
    #[serde(rename = "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED")]
    StopLossOrderPriceAndDistanceBothSpecified,
    #[serde(rename = "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING")]
    StopLossOrderPriceAndDistanceBothMissing,
    #[serde(rename = "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER")]
    StopLossOnFillRequiredForPendingOrder,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED")]
    StopLossOnFillGuaranteedNotAllowed,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED")]
    StopLossOnFillGuaranteedRequired,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_MISSING")]
    StopLossOnFillPriceMissing,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_INVALID")]
    StopLossOnFillPriceInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED")]
    StopLossOnFillPricePrecisionExceeded,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET")]
    StopLossOnFillGuaranteedMinimumDistanceNotMet,
    #[serde(rename = "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED")]
    StopLossOnFillGuaranteedLevelRestrictionExceeded,
    #[serde(rename = "STOP_LOSS_ON_FILL_DISTANCE_INVALID")]
    StopLossOnFillDistanceInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED")]
    StopLossOnFillPriceDistanceMaximumExceeded,
    #[serde(rename = "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED")]
    StopLossOnFillDistancePrecisionExceeded,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED")]
    StopLossOnFillPriceAndDistanceBothSpecified,
    #[serde(rename = "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING")]
    StopLossOnFillPriceAndDistanceBothMissing,
    #[serde(rename = "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING")]
    StopLossOnFillTimeInForceMissing,
    #[serde(rename = "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID")]
    StopLossOnFillTimeInForceInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING")]
    StopLossOnFillGtdTimestampMissing,
    #[serde(rename = "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST")]
    StopLossOnFillGtdTimestampInPast,
    #[serde(rename = "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID")]
    StopLossOnFillClientOrderIdInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID")]
    StopLossOnFillClientOrderTagInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID")]
    StopLossOnFillClientOrderCommentInvalid,
    #[serde(rename = "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING")]
    StopLossOnFillTriggerConditionMissing,
    #[serde(rename = "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID")]
    StopLossOnFillTriggerConditionInvalid,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS")]
    TrailingStopLossOrderAlreadyExists,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING")]
    TrailingStopLossOnFillPriceDistanceMissing,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID")]
    TrailingStopLossOnFillPriceDistanceInvalid,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED")]
    TrailingStopLossOnFillPriceDistancePrecisionExceeded,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED")]
    TrailingStopLossOnFillPriceDistanceMaximumExceeded,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET")]
    TrailingStopLossOnFillPriceDistanceMinimumNotMet,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING")]
    TrailingStopLossOnFillTimeInForceMissing,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID")]
    TrailingStopLossOnFillTimeInForceInvalid,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING")]
    TrailingStopLossOnFillGtdTimestampMissing,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST")]
    TrailingStopLossOnFillGtdTimestampInPast,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID")]
    TrailingStopLossOnFillClientOrderIdInvalid,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID")]
    TrailingStopLossOnFillClientOrderTagInvalid,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID")]
    TrailingStopLossOnFillClientOrderCommentInvalid,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED")]
    TrailingStopLossOrdersNotSupported,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING")]
    TrailingStopLossOnFillTriggerConditionMissing,
    #[serde(rename = "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID")]
    TrailingStopLossOnFillTriggerConditionInvalid,
    #[serde(rename = "CLOSE_TRADE_TYPE_MISSING")]
    CloseTradeTypeMissing,
    #[serde(rename = "CLOSE_TRADE_PARTIAL_UNITS_MISSING")]
    CloseTradePartialUnitsMissing,
    #[serde(rename = "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE")]
    CloseTradeUnitsExceedTradeSize,
    #[serde(rename = "CLOSEOUT_POSITION_DOESNT_EXIST")]
    CloseoutPositionDoesntExist,
    #[serde(rename = "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION")]
    CloseoutPositionIncompleteSpecification,
    #[serde(rename = "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE")]
    CloseoutPositionUnitsExceedPositionSize,
    #[serde(rename = "CLOSEOUT_POSITION_REJECT")]
    CloseoutPositionReject,
    #[serde(rename = "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING")]
    CloseoutPositionPartialUnitsMissing,
    #[serde(rename = "MARKUP_GROUP_ID_INVALID")]
    MarkupGroupIdInvalid,
    #[serde(rename = "POSITION_AGGREGATION_MODE_INVALID")]
    PositionAggregationModeInvalid,
    #[serde(rename = "ADMIN_CONFIGURE_DATA_MISSING")]
    AdminConfigureDataMissing,
    #[serde(rename = "MARGIN_RATE_INVALID")]
    MarginRateInvalid,
    #[serde(rename = "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT")]
    MarginRateWouldTriggerCloseout,
    #[serde(rename = "ALIAS_INVALID")]
    AliasInvalid,
    #[serde(rename = "CLIENT_CONFIGURE_DATA_MISSING")]
    ClientConfigureDataMissing,
    #[serde(rename = "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL")]
    MarginRateWouldTriggerMarginCall,
    #[serde(rename = "AMOUNT_INVALID")]
    AmountInvalid,
    #[serde(rename = "INSUFFICIENT_FUNDS")]
    InsufficientFunds,
    #[serde(rename = "AMOUNT_MISSING")]
    AmountMissing,
    #[serde(rename = "FUNDING_REASON_MISSING")]
    FundingReasonMissing,
    #[serde(rename = "CLIENT_EXTENSIONS_DATA_MISSING")]
    ClientExtensionsDataMissing,
    #[serde(rename = "REPLACING_ORDER_INVALID")]
    ReplacingOrderInvalid,
    #[serde(rename = "REPLACING_TRADE_ID_INVALID")]
    ReplacingTradeIdInvalid,
}

impl std::str::FromStr for TransactionRejectReason {
    type Err = ();
    fn from_str(s: &str) -> Result<TransactionRejectReason, ()> {
        match s {
            "INTERNAL_SERVER_ERROR" => Ok(TransactionRejectReason::InternalServerError),
            "INSTRUMENT_PRICE_UNKNOWN" => Ok(TransactionRejectReason::InstrumentPriceUnknown),
            "ACCOUNT_NOT_ACTIVE" => Ok(TransactionRejectReason::AccountNotActive),
            "ACCOUNT_LOCKED" => Ok(TransactionRejectReason::AccountLocked),
            "ACCOUNT_ORDER_CREATION_LOCKED" => {
                Ok(TransactionRejectReason::AccountOrderCreationLocked)
            }
            "ACCOUNT_CONFIGURATION_LOCKED" => {
                Ok(TransactionRejectReason::AccountConfigurationLocked)
            }
            "ACCOUNT_DEPOSIT_LOCKED" => Ok(TransactionRejectReason::AccountDepositLocked),
            "ACCOUNT_WITHDRAWAL_LOCKED" => Ok(TransactionRejectReason::AccountWithdrawalLocked),
            "ACCOUNT_ORDER_CANCEL_LOCKED" => Ok(TransactionRejectReason::AccountOrderCancelLocked),
            "INSTRUMENT_NOT_TRADEABLE" => Ok(TransactionRejectReason::InstrumentNotTradeable),
            "PENDING_ORDERS_ALLOWED_EXCEEDED" => {
                Ok(TransactionRejectReason::PendingOrdersAllowedExceeded)
            }
            "ORDER_ID_UNSPECIFIED" => Ok(TransactionRejectReason::OrderIdUnspecified),
            "ORDER_DOESNT_EXIST" => Ok(TransactionRejectReason::OrderDoesntExist),
            "ORDER_IDENTIFIER_INCONSISTENCY" => {
                Ok(TransactionRejectReason::OrderIdentifierInconsistency)
            }
            "TRADE_ID_UNSPECIFIED" => Ok(TransactionRejectReason::TradeIdUnspecified),
            "TRADE_DOESNT_EXIST" => Ok(TransactionRejectReason::TradeDoesntExist),
            "TRADE_IDENTIFIER_INCONSISTENCY" => {
                Ok(TransactionRejectReason::TradeIdentifierInconsistency)
            }
            "INSUFFICIENT_MARGIN" => Ok(TransactionRejectReason::InsufficientMargin),
            "INSTRUMENT_MISSING" => Ok(TransactionRejectReason::InstrumentMissing),
            "INSTRUMENT_UNKNOWN" => Ok(TransactionRejectReason::InstrumentUnknown),
            "UNITS_MISSING" => Ok(TransactionRejectReason::UnitsMissing),
            "UNITS_INVALID" => Ok(TransactionRejectReason::UnitsInvalid),
            "UNITS_PRECISION_EXCEEDED" => Ok(TransactionRejectReason::UnitsPrecisionExceeded),
            "UNITS_LIMIT_EXCEEDED" => Ok(TransactionRejectReason::UnitsLimitExceeded),
            "UNITS_MIMIMUM_NOT_MET" => Ok(TransactionRejectReason::UnitsMimimumNotMet),
            "PRICE_MISSING" => Ok(TransactionRejectReason::PriceMissing),
            "PRICE_INVALID" => Ok(TransactionRejectReason::PriceInvalid),
            "PRICE_PRECISION_EXCEEDED" => Ok(TransactionRejectReason::PricePrecisionExceeded),
            "PRICE_DISTANCE_MISSING" => Ok(TransactionRejectReason::PriceDistanceMissing),
            "PRICE_DISTANCE_INVALID" => Ok(TransactionRejectReason::PriceDistanceInvalid),
            "PRICE_DISTANCE_PRECISION_EXCEEDED" => {
                Ok(TransactionRejectReason::PriceDistancePrecisionExceeded)
            }
            "PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Ok(TransactionRejectReason::PriceDistanceMaximumExceeded)
            }
            "PRICE_DISTANCE_MINIMUM_NOT_MET" => {
                Ok(TransactionRejectReason::PriceDistanceMinimumNotMet)
            }
            "TIME_IN_FORCE_MISSING" => Ok(TransactionRejectReason::TimeInForceMissing),
            "TIME_IN_FORCE_INVALID" => Ok(TransactionRejectReason::TimeInForceInvalid),
            "TIME_IN_FORCE_GTD_TIMESTAMP_MISSING" => {
                Ok(TransactionRejectReason::TimeInForceGtdTimestampMissing)
            }
            "TIME_IN_FORCE_GTD_TIMESTAMP_IN_PAST" => {
                Ok(TransactionRejectReason::TimeInForceGtdTimestampInPast)
            }
            "PRICE_BOUND_INVALID" => Ok(TransactionRejectReason::PriceBoundInvalid),
            "PRICE_BOUND_PRECISION_EXCEEDED" => {
                Ok(TransactionRejectReason::PriceBoundPrecisionExceeded)
            }
            "ORDERS_ON_FILL_DUPLICATE_CLIENT_ORDER_IDS" => {
                Ok(TransactionRejectReason::OrdersOnFillDuplicateClientOrderIds)
            }
            "TRADE_ON_FILL_CLIENT_EXTENSIONS_NOT_SUPPORTED" => {
                Ok(TransactionRejectReason::TradeOnFillClientExtensionsNotSupported)
            }
            "CLIENT_ORDER_ID_INVALID" => Ok(TransactionRejectReason::ClientOrderIdInvalid),
            "CLIENT_ORDER_ID_ALREADY_EXISTS" => {
                Ok(TransactionRejectReason::ClientOrderIdAlreadyExists)
            }
            "CLIENT_ORDER_TAG_INVALID" => Ok(TransactionRejectReason::ClientOrderTagInvalid),
            "CLIENT_ORDER_COMMENT_INVALID" => {
                Ok(TransactionRejectReason::ClientOrderCommentInvalid)
            }
            "CLIENT_TRADE_ID_INVALID" => Ok(TransactionRejectReason::ClientTradeIdInvalid),
            "CLIENT_TRADE_ID_ALREADY_EXISTS" => {
                Ok(TransactionRejectReason::ClientTradeIdAlreadyExists)
            }
            "CLIENT_TRADE_TAG_INVALID" => Ok(TransactionRejectReason::ClientTradeTagInvalid),
            "CLIENT_TRADE_COMMENT_INVALID" => {
                Ok(TransactionRejectReason::ClientTradeCommentInvalid)
            }
            "ORDER_FILL_POSITION_ACTION_MISSING" => {
                Ok(TransactionRejectReason::OrderFillPositionActionMissing)
            }
            "ORDER_FILL_POSITION_ACTION_INVALID" => {
                Ok(TransactionRejectReason::OrderFillPositionActionInvalid)
            }
            "TRIGGER_CONDITION_MISSING" => Ok(TransactionRejectReason::TriggerConditionMissing),
            "TRIGGER_CONDITION_INVALID" => Ok(TransactionRejectReason::TriggerConditionInvalid),
            "ORDER_PARTIAL_FILL_OPTION_MISSING" => {
                Ok(TransactionRejectReason::OrderPartialFillOptionMissing)
            }
            "ORDER_PARTIAL_FILL_OPTION_INVALID" => {
                Ok(TransactionRejectReason::OrderPartialFillOptionInvalid)
            }
            "INVALID_REISSUE_IMMEDIATE_PARTIAL_FILL" => {
                Ok(TransactionRejectReason::InvalidReissueImmediatePartialFill)
            }
            "TAKE_PROFIT_ORDER_ALREADY_EXISTS" => {
                Ok(TransactionRejectReason::TakeProfitOrderAlreadyExists)
            }
            "TAKE_PROFIT_ON_FILL_PRICE_MISSING" => {
                Ok(TransactionRejectReason::TakeProfitOnFillPriceMissing)
            }
            "TAKE_PROFIT_ON_FILL_PRICE_INVALID" => {
                Ok(TransactionRejectReason::TakeProfitOnFillPriceInvalid)
            }
            "TAKE_PROFIT_ON_FILL_PRICE_PRECISION_EXCEEDED" => {
                Ok(TransactionRejectReason::TakeProfitOnFillPricePrecisionExceeded)
            }
            "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_MISSING" => {
                Ok(TransactionRejectReason::TakeProfitOnFillTimeInForceMissing)
            }
            "TAKE_PROFIT_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Ok(TransactionRejectReason::TakeProfitOnFillTimeInForceInvalid)
            }
            "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_MISSING" => {
                Ok(TransactionRejectReason::TakeProfitOnFillGtdTimestampMissing)
            }
            "TAKE_PROFIT_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Ok(TransactionRejectReason::TakeProfitOnFillGtdTimestampInPast)
            }
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_ID_INVALID" => {
                Ok(TransactionRejectReason::TakeProfitOnFillClientOrderIdInvalid)
            }
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Ok(TransactionRejectReason::TakeProfitOnFillClientOrderTagInvalid)
            }
            "TAKE_PROFIT_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Ok(TransactionRejectReason::TakeProfitOnFillClientOrderCommentInvalid)
            }
            "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Ok(TransactionRejectReason::TakeProfitOnFillTriggerConditionMissing)
            }
            "TAKE_PROFIT_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Ok(TransactionRejectReason::TakeProfitOnFillTriggerConditionInvalid)
            }
            "STOP_LOSS_ORDER_ALREADY_EXISTS" => {
                Ok(TransactionRejectReason::StopLossOrderAlreadyExists)
            }
            "STOP_LOSS_ORDER_GUARANTEED_REQUIRED" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedRequired)
            }
            "STOP_LOSS_ORDER_GUARANTEED_PRICE_WITHIN_SPREAD" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedPriceWithinSpread)
            }
            "STOP_LOSS_ORDER_GUARANTEED_NOT_ALLOWED" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedNotAllowed)
            }
            "STOP_LOSS_ORDER_GUARANTEED_HALTED_CREATE_VIOLATION" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedHaltedCreateViolation)
            }
            "STOP_LOSS_ORDER_GUARANTEED_HALTED_TIGHTEN_VIOLATION" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedHaltedTightenViolation)
            }
            "STOP_LOSS_ORDER_GUARANTEED_HEDGING_NOT_ALLOWED" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedHedgingNotAllowed)
            }
            "STOP_LOSS_ORDER_GUARANTEED_MINIMUM_DISTANCE_NOT_MET" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedMinimumDistanceNotMet)
            }
            "STOP_LOSS_ORDER_NOT_CANCELABLE" => {
                Ok(TransactionRejectReason::StopLossOrderNotCancelable)
            }
            "STOP_LOSS_ORDER_NOT_REPLACEABLE" => {
                Ok(TransactionRejectReason::StopLossOrderNotReplaceable)
            }
            "STOP_LOSS_ORDER_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED" => {
                Ok(TransactionRejectReason::StopLossOrderGuaranteedLevelRestrictionExceeded)
            }
            "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_SPECIFIED" => {
                Ok(TransactionRejectReason::StopLossOrderPriceAndDistanceBothSpecified)
            }
            "STOP_LOSS_ORDER_PRICE_AND_DISTANCE_BOTH_MISSING" => {
                Ok(TransactionRejectReason::StopLossOrderPriceAndDistanceBothMissing)
            }
            "STOP_LOSS_ON_FILL_REQUIRED_FOR_PENDING_ORDER" => {
                Ok(TransactionRejectReason::StopLossOnFillRequiredForPendingOrder)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_NOT_ALLOWED" => {
                Ok(TransactionRejectReason::StopLossOnFillGuaranteedNotAllowed)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_REQUIRED" => {
                Ok(TransactionRejectReason::StopLossOnFillGuaranteedRequired)
            }
            "STOP_LOSS_ON_FILL_PRICE_MISSING" => {
                Ok(TransactionRejectReason::StopLossOnFillPriceMissing)
            }
            "STOP_LOSS_ON_FILL_PRICE_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillPriceInvalid)
            }
            "STOP_LOSS_ON_FILL_PRICE_PRECISION_EXCEEDED" => {
                Ok(TransactionRejectReason::StopLossOnFillPricePrecisionExceeded)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_MINIMUM_DISTANCE_NOT_MET" => {
                Ok(TransactionRejectReason::StopLossOnFillGuaranteedMinimumDistanceNotMet)
            }
            "STOP_LOSS_ON_FILL_GUARANTEED_LEVEL_RESTRICTION_EXCEEDED" => {
                Ok(TransactionRejectReason::StopLossOnFillGuaranteedLevelRestrictionExceeded)
            }
            "STOP_LOSS_ON_FILL_DISTANCE_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillDistanceInvalid)
            }
            "STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Ok(TransactionRejectReason::StopLossOnFillPriceDistanceMaximumExceeded)
            }
            "STOP_LOSS_ON_FILL_DISTANCE_PRECISION_EXCEEDED" => {
                Ok(TransactionRejectReason::StopLossOnFillDistancePrecisionExceeded)
            }
            "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_SPECIFIED" => {
                Ok(TransactionRejectReason::StopLossOnFillPriceAndDistanceBothSpecified)
            }
            "STOP_LOSS_ON_FILL_PRICE_AND_DISTANCE_BOTH_MISSING" => {
                Ok(TransactionRejectReason::StopLossOnFillPriceAndDistanceBothMissing)
            }
            "STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING" => {
                Ok(TransactionRejectReason::StopLossOnFillTimeInForceMissing)
            }
            "STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillTimeInForceInvalid)
            }
            "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING" => {
                Ok(TransactionRejectReason::StopLossOnFillGtdTimestampMissing)
            }
            "STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Ok(TransactionRejectReason::StopLossOnFillGtdTimestampInPast)
            }
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillClientOrderIdInvalid)
            }
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillClientOrderTagInvalid)
            }
            "STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillClientOrderCommentInvalid)
            }
            "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Ok(TransactionRejectReason::StopLossOnFillTriggerConditionMissing)
            }
            "STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Ok(TransactionRejectReason::StopLossOnFillTriggerConditionInvalid)
            }
            "TRAILING_STOP_LOSS_ORDER_ALREADY_EXISTS" => {
                Ok(TransactionRejectReason::TrailingStopLossOrderAlreadyExists)
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MISSING" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillPriceDistanceMissing)
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_INVALID" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillPriceDistanceInvalid)
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_PRECISION_EXCEEDED" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillPriceDistancePrecisionExceeded)
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MAXIMUM_EXCEEDED" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillPriceDistanceMaximumExceeded)
            }
            "TRAILING_STOP_LOSS_ON_FILL_PRICE_DISTANCE_MINIMUM_NOT_MET" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillPriceDistanceMinimumNotMet)
            }
            "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_MISSING" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillTimeInForceMissing)
            }
            "TRAILING_STOP_LOSS_ON_FILL_TIME_IN_FORCE_INVALID" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillTimeInForceInvalid)
            }
            "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_MISSING" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillGtdTimestampMissing)
            }
            "TRAILING_STOP_LOSS_ON_FILL_GTD_TIMESTAMP_IN_PAST" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillGtdTimestampInPast)
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_ID_INVALID" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillClientOrderIdInvalid)
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_TAG_INVALID" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillClientOrderTagInvalid)
            }
            "TRAILING_STOP_LOSS_ON_FILL_CLIENT_ORDER_COMMENT_INVALID" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillClientOrderCommentInvalid)
            }
            "TRAILING_STOP_LOSS_ORDERS_NOT_SUPPORTED" => {
                Ok(TransactionRejectReason::TrailingStopLossOrdersNotSupported)
            }
            "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_MISSING" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillTriggerConditionMissing)
            }
            "TRAILING_STOP_LOSS_ON_FILL_TRIGGER_CONDITION_INVALID" => {
                Ok(TransactionRejectReason::TrailingStopLossOnFillTriggerConditionInvalid)
            }
            "CLOSE_TRADE_TYPE_MISSING" => Ok(TransactionRejectReason::CloseTradeTypeMissing),
            "CLOSE_TRADE_PARTIAL_UNITS_MISSING" => {
                Ok(TransactionRejectReason::CloseTradePartialUnitsMissing)
            }
            "CLOSE_TRADE_UNITS_EXCEED_TRADE_SIZE" => {
                Ok(TransactionRejectReason::CloseTradeUnitsExceedTradeSize)
            }
            "CLOSEOUT_POSITION_DOESNT_EXIST" => {
                Ok(TransactionRejectReason::CloseoutPositionDoesntExist)
            }
            "CLOSEOUT_POSITION_INCOMPLETE_SPECIFICATION" => {
                Ok(TransactionRejectReason::CloseoutPositionIncompleteSpecification)
            }
            "CLOSEOUT_POSITION_UNITS_EXCEED_POSITION_SIZE" => {
                Ok(TransactionRejectReason::CloseoutPositionUnitsExceedPositionSize)
            }
            "CLOSEOUT_POSITION_REJECT" => Ok(TransactionRejectReason::CloseoutPositionReject),
            "CLOSEOUT_POSITION_PARTIAL_UNITS_MISSING" => {
                Ok(TransactionRejectReason::CloseoutPositionPartialUnitsMissing)
            }
            "MARKUP_GROUP_ID_INVALID" => Ok(TransactionRejectReason::MarkupGroupIdInvalid),
            "POSITION_AGGREGATION_MODE_INVALID" => {
                Ok(TransactionRejectReason::PositionAggregationModeInvalid)
            }
            "ADMIN_CONFIGURE_DATA_MISSING" => {
                Ok(TransactionRejectReason::AdminConfigureDataMissing)
            }
            "MARGIN_RATE_INVALID" => Ok(TransactionRejectReason::MarginRateInvalid),
            "MARGIN_RATE_WOULD_TRIGGER_CLOSEOUT" => {
                Ok(TransactionRejectReason::MarginRateWouldTriggerCloseout)
            }
            "ALIAS_INVALID" => Ok(TransactionRejectReason::AliasInvalid),
            "CLIENT_CONFIGURE_DATA_MISSING" => {
                Ok(TransactionRejectReason::ClientConfigureDataMissing)
            }
            "MARGIN_RATE_WOULD_TRIGGER_MARGIN_CALL" => {
                Ok(TransactionRejectReason::MarginRateWouldTriggerMarginCall)
            }
            "AMOUNT_INVALID" => Ok(TransactionRejectReason::AmountInvalid),
            "INSUFFICIENT_FUNDS" => Ok(TransactionRejectReason::InsufficientFunds),
            "AMOUNT_MISSING" => Ok(TransactionRejectReason::AmountMissing),
            "FUNDING_REASON_MISSING" => Ok(TransactionRejectReason::FundingReasonMissing),
            "CLIENT_EXTENSIONS_DATA_MISSING" => {
                Ok(TransactionRejectReason::ClientExtensionsDataMissing)
            }
            "REPLACING_ORDER_INVALID" => Ok(TransactionRejectReason::ReplacingOrderInvalid),
            "REPLACING_TRADE_ID_INVALID" => Ok(TransactionRejectReason::ReplacingTradeIdInvalid),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TransactionRejectReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionFilter {
    #[serde(rename = "ORDER")]
    Order,
    #[serde(rename = "FUNDING")]
    Funding,
    #[serde(rename = "ADMIN")]
    Admin,
    #[serde(rename = "CREATE")]
    Create,
    #[serde(rename = "CLOSE")]
    Close,
    #[serde(rename = "REOPEN")]
    Reopen,
    #[serde(rename = "CLIENT_CONFIGURE")]
    ClientConfigure,
    #[serde(rename = "CLIENT_CONFIGURE_REJECT")]
    ClientConfigureReject,
    #[serde(rename = "TRANSFER_FUNDS")]
    TransferFunds,
    #[serde(rename = "TRANSFER_FUNDS_REJECT")]
    TransferFundsReject,
    #[serde(rename = "MARKET_ORDER")]
    MarketOrder,
    #[serde(rename = "MARKET_ORDER_REJECT")]
    MarketOrderReject,
    #[serde(rename = "LIMIT_ORDER")]
    LimitOrder,
    #[serde(rename = "LIMIT_ORDER_REJECT")]
    LimitOrderReject,
    #[serde(rename = "STOP_ORDER")]
    StopOrder,
    #[serde(rename = "STOP_ORDER_REJECT")]
    StopOrderReject,
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER")]
    MarketIfTouchedOrder,
    #[serde(rename = "MARKET_IF_TOUCHED_ORDER_REJECT")]
    MarketIfTouchedOrderReject,
    #[serde(rename = "TAKE_PROFIT_ORDER")]
    TakeProfitOrder,
    #[serde(rename = "TAKE_PROFIT_ORDER_REJECT")]
    TakeProfitOrderReject,
    #[serde(rename = "STOP_LOSS_ORDER")]
    StopLossOrder,
    #[serde(rename = "STOP_LOSS_ORDER_REJECT")]
    StopLossOrderReject,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER")]
    TrailingStopLossOrder,
    #[serde(rename = "TRAILING_STOP_LOSS_ORDER_REJECT")]
    TrailingStopLossOrderReject,
    #[serde(rename = "ONE_CANCELS_ALL_ORDER")]
    OneCancelsAllOrder,
    #[serde(rename = "ONE_CANCELS_ALL_ORDER_REJECT")]
    OneCancelsAllOrderReject,
    #[serde(rename = "ONE_CANCELS_ALL_ORDER_TRIGGERED")]
    OneCancelsAllOrderTriggered,
    #[serde(rename = "ORDER_FILL")]
    OrderFill,
    #[serde(rename = "ORDER_CANCEL")]
    OrderCancel,
    #[serde(rename = "ORDER_CANCEL_REJECT")]
    OrderCancelReject,
    #[serde(rename = "ORDER_CLIENT_EXTENSIONS_MODIFY")]
    OrderClientExtensionsModify,
    #[serde(rename = "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    OrderClientExtensionsModifyReject,
    #[serde(rename = "TRADE_CLIENT_EXTENSIONS_MODIFY")]
    TradeClientExtensionsModify,
    #[serde(rename = "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT")]
    TradeClientExtensionsModifyReject,
    #[serde(rename = "MARGIN_CALL_ENTER")]
    MarginCallEnter,
    #[serde(rename = "MARGIN_CALL_EXTEND")]
    MarginCallExtend,
    #[serde(rename = "MARGIN_CALL_EXIT")]
    MarginCallExit,
    #[serde(rename = "DELAYED_TRADE_CLOSURE")]
    DelayedTradeClosure,
    #[serde(rename = "DAILY_FINANCING")]
    DailyFinancing,
    #[serde(rename = "RESET_RESETTABLE_PL")]
    ResetResettablePl,
}

impl std::str::FromStr for TransactionFilter {
    type Err = ();
    fn from_str(s: &str) -> Result<TransactionFilter, ()> {
        match s {
            "ORDER" => Ok(TransactionFilter::Order),
            "FUNDING" => Ok(TransactionFilter::Funding),
            "ADMIN" => Ok(TransactionFilter::Admin),
            "CREATE" => Ok(TransactionFilter::Create),
            "CLOSE" => Ok(TransactionFilter::Close),
            "REOPEN" => Ok(TransactionFilter::Reopen),
            "CLIENT_CONFIGURE" => Ok(TransactionFilter::ClientConfigure),
            "CLIENT_CONFIGURE_REJECT" => Ok(TransactionFilter::ClientConfigureReject),
            "TRANSFER_FUNDS" => Ok(TransactionFilter::TransferFunds),
            "TRANSFER_FUNDS_REJECT" => Ok(TransactionFilter::TransferFundsReject),
            "MARKET_ORDER" => Ok(TransactionFilter::MarketOrder),
            "MARKET_ORDER_REJECT" => Ok(TransactionFilter::MarketOrderReject),
            "LIMIT_ORDER" => Ok(TransactionFilter::LimitOrder),
            "LIMIT_ORDER_REJECT" => Ok(TransactionFilter::LimitOrderReject),
            "STOP_ORDER" => Ok(TransactionFilter::StopOrder),
            "STOP_ORDER_REJECT" => Ok(TransactionFilter::StopOrderReject),
            "MARKET_IF_TOUCHED_ORDER" => Ok(TransactionFilter::MarketIfTouchedOrder),
            "MARKET_IF_TOUCHED_ORDER_REJECT" => Ok(TransactionFilter::MarketIfTouchedOrderReject),
            "TAKE_PROFIT_ORDER" => Ok(TransactionFilter::TakeProfitOrder),
            "TAKE_PROFIT_ORDER_REJECT" => Ok(TransactionFilter::TakeProfitOrderReject),
            "STOP_LOSS_ORDER" => Ok(TransactionFilter::StopLossOrder),
            "STOP_LOSS_ORDER_REJECT" => Ok(TransactionFilter::StopLossOrderReject),
            "TRAILING_STOP_LOSS_ORDER" => Ok(TransactionFilter::TrailingStopLossOrder),
            "TRAILING_STOP_LOSS_ORDER_REJECT" => Ok(TransactionFilter::TrailingStopLossOrderReject),
            "ONE_CANCELS_ALL_ORDER" => Ok(TransactionFilter::OneCancelsAllOrder),
            "ONE_CANCELS_ALL_ORDER_REJECT" => Ok(TransactionFilter::OneCancelsAllOrderReject),
            "ONE_CANCELS_ALL_ORDER_TRIGGERED" => Ok(TransactionFilter::OneCancelsAllOrderTriggered),
            "ORDER_FILL" => Ok(TransactionFilter::OrderFill),
            "ORDER_CANCEL" => Ok(TransactionFilter::OrderCancel),
            "ORDER_CANCEL_REJECT" => Ok(TransactionFilter::OrderCancelReject),
            "ORDER_CLIENT_EXTENSIONS_MODIFY" => Ok(TransactionFilter::OrderClientExtensionsModify),
            "ORDER_CLIENT_EXTENSIONS_MODIFY_REJECT" => {
                Ok(TransactionFilter::OrderClientExtensionsModifyReject)
            }
            "TRADE_CLIENT_EXTENSIONS_MODIFY" => Ok(TransactionFilter::TradeClientExtensionsModify),
            "TRADE_CLIENT_EXTENSIONS_MODIFY_REJECT" => {
                Ok(TransactionFilter::TradeClientExtensionsModifyReject)
            }
            "MARGIN_CALL_ENTER" => Ok(TransactionFilter::MarginCallEnter),
            "MARGIN_CALL_EXTEND" => Ok(TransactionFilter::MarginCallExtend),
            "MARGIN_CALL_EXIT" => Ok(TransactionFilter::MarginCallExit),
            "DELAYED_TRADE_CLOSURE" => Ok(TransactionFilter::DelayedTradeClosure),
            "DAILY_FINANCING" => Ok(TransactionFilter::DailyFinancing),
            "RESET_RESETTABLE_PL" => Ok(TransactionFilter::ResetResettablePl),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TransactionFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum PriceStatus {
    #[serde(rename = "tradeable")]
    Tradeable,
    #[serde(rename = "nontradeable")]
    Nontradeable,
    #[serde(rename = "invalid")]
    Invalid,
}

impl std::str::FromStr for PriceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<PriceStatus, ()> {
        match s {
            "tradeable" => Ok(PriceStatus::Tradeable),
            "nontradeable" => Ok(PriceStatus::Nontradeable),
            "invalid" => Ok(PriceStatus::Invalid),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for PriceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderType {
    #[serde(rename = "MARKET")]
    Market,
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "MARKET_IF_TOUCHED")]
    MarketIfTouched,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "STOP_LOSS")]
    StopLoss,
    #[serde(rename = "TRAILING_STOP_LOSS")]
    TrailingStopLoss,
    #[serde(rename = "FIXED_PRICE")]
    FixedPrice,
}

impl std::str::FromStr for OrderType {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderType, ()> {
        match s {
            "MARKET" => Ok(OrderType::Market),
            "LIMIT" => Ok(OrderType::Limit),
            "STOP" => Ok(OrderType::Stop),
            "MARKET_IF_TOUCHED" => Ok(OrderType::MarketIfTouched),
            "TAKE_PROFIT" => Ok(OrderType::TakeProfit),
            "STOP_LOSS" => Ok(OrderType::StopLoss),
            "TRAILING_STOP_LOSS" => Ok(OrderType::TrailingStopLoss),
            "FIXED_PRICE" => Ok(OrderType::FixedPrice),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum CancellableOrderType {
    #[serde(rename = "LIMIT")]
    Limit,
    #[serde(rename = "STOP")]
    Stop,
    #[serde(rename = "MARKET_IF_TOUCHED")]
    MarketIfTouched,
    #[serde(rename = "TAKE_PROFIT")]
    TakeProfit,
    #[serde(rename = "STOP_LOSS")]
    StopLoss,
    #[serde(rename = "TRAILING_STOP_LOSS")]
    TrailingStopLoss,
}

impl std::str::FromStr for CancellableOrderType {
    type Err = ();
    fn from_str(s: &str) -> Result<CancellableOrderType, ()> {
        match s {
            "LIMIT" => Ok(CancellableOrderType::Limit),
            "STOP" => Ok(CancellableOrderType::Stop),
            "MARKET_IF_TOUCHED" => Ok(CancellableOrderType::MarketIfTouched),
            "TAKE_PROFIT" => Ok(CancellableOrderType::TakeProfit),
            "STOP_LOSS" => Ok(CancellableOrderType::StopLoss),
            "TRAILING_STOP_LOSS" => Ok(CancellableOrderType::TrailingStopLoss),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for CancellableOrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderState {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "TRIGGERED")]
    Triggered,
    #[serde(rename = "CANCELLED")]
    Cancelled,
}

impl std::str::FromStr for OrderState {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderState, ()> {
        match s {
            "PENDING" => Ok(OrderState::Pending),
            "FILLED" => Ok(OrderState::Filled),
            "TRIGGERED" => Ok(OrderState::Triggered),
            "CANCELLED" => Ok(OrderState::Cancelled),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStateFilter {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "FILLED")]
    Filled,
    #[serde(rename = "TRIGGERED")]
    Triggered,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "ALL")]
    All,
}

impl std::str::FromStr for OrderStateFilter {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderStateFilter, ()> {
        match s {
            "PENDING" => Ok(OrderStateFilter::Pending),
            "FILLED" => Ok(OrderStateFilter::Filled),
            "TRIGGERED" => Ok(OrderStateFilter::Triggered),
            "CANCELLED" => Ok(OrderStateFilter::Cancelled),
            "ALL" => Ok(OrderStateFilter::All),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderStateFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "GTD")]
    Gtd,
    #[serde(rename = "GFD")]
    Gfd,
    #[serde(rename = "FOK")]
    Fok,
    #[serde(rename = "IOC")]
    Ioc,
}

impl std::str::FromStr for TimeInForce {
    type Err = ();
    fn from_str(s: &str) -> Result<TimeInForce, ()> {
        match s {
            "GTC" => Ok(TimeInForce::Gtc),
            "GTD" => Ok(TimeInForce::Gtd),
            "GFD" => Ok(TimeInForce::Gfd),
            "FOK" => Ok(TimeInForce::Fok),
            "IOC" => Ok(TimeInForce::Ioc),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderPositionFill {
    #[serde(rename = "OPEN_ONLY")]
    OpenOnly,
    #[serde(rename = "REDUCE_FIRST")]
    ReduceFirst,
    #[serde(rename = "REDUCE_ONLY")]
    ReduceOnly,
    #[serde(rename = "DEFAULT")]
    Default,
}

impl std::str::FromStr for OrderPositionFill {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderPositionFill, ()> {
        match s {
            "OPEN_ONLY" => Ok(OrderPositionFill::OpenOnly),
            "REDUCE_FIRST" => Ok(OrderPositionFill::ReduceFirst),
            "REDUCE_ONLY" => Ok(OrderPositionFill::ReduceOnly),
            "DEFAULT" => Ok(OrderPositionFill::Default),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderPositionFill {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[repr(C)]
#[derive(Debug, Serialize, Deserialize)]
pub enum OrderTriggerCondition {
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "INVERSE")]
    Inverse,
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
    #[serde(rename = "MID")]
    Mid,
}

impl std::str::FromStr for OrderTriggerCondition {
    type Err = ();
    fn from_str(s: &str) -> Result<OrderTriggerCondition, ()> {
        match s {
            "DEFAULT" => Ok(OrderTriggerCondition::Default),
            "INVERSE" => Ok(OrderTriggerCondition::Inverse),
            "BID" => Ok(OrderTriggerCondition::Bid),
            "ASK" => Ok(OrderTriggerCondition::Ask),
            "MID" => Ok(OrderTriggerCondition::Mid),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for OrderTriggerCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
