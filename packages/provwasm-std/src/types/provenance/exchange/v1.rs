use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
/// Commitment contains information on committed funds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Commitment")]
pub struct Commitment {
    /// account is the bech32 address string with the committed funds.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// market_id is the numeric identifier of the market the funds are committed to.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// amount is the funds that have been committed by the account to the market.
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// AccountAmount associates an account with a coins amount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.AccountAmount")]
pub struct AccountAmount {
    /// account is the bech32 address string of the account associated with the amount.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// amount is the funds associated with the address.
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MarketAmount associates a market with a coins amount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MarketAmount")]
pub struct MarketAmount {
    /// market_id is the numeric identifier the amount has been committed to.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// amount is the funds associated with the address.
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// NetAssetPrice is an association of assets and price used to record the value of things.
/// It is related to the NetAssetValue message from the x/marker module, and is therefore often referred to as "a NAV".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.NetAssetPrice")]
pub struct NetAssetPrice {
    /// assets is the volume and denom that has been bought or sold.
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// price is what was paid for the assets.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// EventOrderCreated is an event emitted when an order is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderCreated")]
pub struct EventOrderCreated {
    /// order_id is the numerical identifier of the order created.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// order_type is the type of order, e.g. "ask" or "bid".
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "3")]
    pub market_id: u32,
    /// external_id is the order's external id.
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventOrderCancelled is an event emitted when an order is cancelled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderCancelled")]
pub struct EventOrderCancelled {
    /// order_id is the numerical identifier of the order cancelled.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// cancelled_by is the account that triggered the cancellation of the order.
    #[prost(string, tag = "2")]
    pub cancelled_by: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "3")]
    pub market_id: u32,
    /// external_id is the order's external id.
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventOrderFilled is an event emitted when an order has been filled in full.
/// This event is also used for orders that were previously partially filled, but have now been filled in full.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderFilled")]
pub struct EventOrderFilled {
    /// order_id is the numerical identifier of the order filled.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// assets is the coins amount string of assets bought/sold for this order.
    #[prost(string, tag = "2")]
    pub assets: ::prost::alloc::string::String,
    /// price is the coins amount string of the price payed/received for this order.
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// fees is the coins amount string of settlement fees paid with this order.
    #[prost(string, tag = "4")]
    pub fees: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "5")]
    pub market_id: u32,
    /// external_id is the order's external id.
    #[prost(string, tag = "6")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventOrderPartiallyFilled is an event emitted when an order filled in part and still has more left to fill.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderPartiallyFilled")]
pub struct EventOrderPartiallyFilled {
    /// order_id is the numerical identifier of the order partially filled.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// assets is the coins amount string of assets that were filled and removed from the order.
    #[prost(string, tag = "2")]
    pub assets: ::prost::alloc::string::String,
    /// price is the coins amount string of the price payed/received for this order.
    /// For ask orders, this might be more than the amount that was removed from the order's price.
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// fees is the coins amount string of settlement fees paid with this partial order.
    /// For ask orders, this might be more than the amount that was removed from the order's settlement fees.
    #[prost(string, tag = "4")]
    pub fees: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "5")]
    pub market_id: u32,
    /// external_id is the order's external id.
    #[prost(string, tag = "6")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventOrderExternalIDUpdated is an event emitted when an order's external id is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventOrderExternalIDUpdated")]
pub struct EventOrderExternalIdUpdated {
    /// order_id is the numerical identifier of the order partially filled.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// external_id is the order's new external id.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventFundsCommitted is an event emitted when funds are committed to a market.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventFundsCommitted")]
pub struct EventFundsCommitted {
    /// account is the bech32 address string of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// amount is the coins string of the newly committed funds.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    /// tag is the string provided in the message causing this event.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
}
/// EventCommitmentReleased is an event emitted when funds are released from their commitment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventCommitmentReleased")]
pub struct EventCommitmentReleased {
    /// account is the bech32 address string of the account.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// amount is the coins string of the funds that were released from commitment.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
    /// tag is the string provided in the message causing this event.
    #[prost(string, tag = "4")]
    pub tag: ::prost::alloc::string::String,
}
/// EventMarketWithdraw is an event emitted when a withdrawal of a market's collected fees is made.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketWithdraw")]
pub struct EventMarketWithdraw {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// amount is the coins amount string of funds withdrawn from the market account.
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// destination is the account that received the funds.
    #[prost(string, tag = "3")]
    pub destination: ::prost::alloc::string::String,
    /// withdrawn_by is the account that requested the withdrawal.
    #[prost(string, tag = "4")]
    pub withdrawn_by: ::prost::alloc::string::String,
}
/// EventMarketDetailsUpdated is an event emitted when a market's details are updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketDetailsUpdated")]
pub struct EventMarketDetailsUpdated {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the details.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketEnabled is an event emitted when a market is enabled.
/// Deprecated: This event is no longer used. It is replaced with EventMarketOrdersEnabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketEnabled")]
#[deprecated]
pub struct EventMarketEnabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that enabled the market.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketDisabled is an event emitted when a market is disabled.
/// Deprecated: This event is no longer used. It is replaced with EventMarketOrdersDisabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketDisabled")]
#[deprecated]
pub struct EventMarketDisabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that disabled the market.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketOrdersEnabled is an event emitted when a market enables order creation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketOrdersEnabled")]
pub struct EventMarketOrdersEnabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the accepting_orders option.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketOrdersEnabled is an event emitted when a market disables order creation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketOrdersDisabled")]
pub struct EventMarketOrdersDisabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the accepting_orders option.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketUserSettleEnabled is an event emitted when a market's user_settle option is enabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketUserSettleEnabled")]
pub struct EventMarketUserSettleEnabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the user_settle option.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketUserSettleDisabled is an event emitted when a market's user_settle option is disabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketUserSettleDisabled")]
pub struct EventMarketUserSettleDisabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the user_settle option.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketCommitmentsEnabled is an event emitted when a market's accepting_commitments option is enabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketCommitmentsEnabled")]
pub struct EventMarketCommitmentsEnabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the accepting_commitments option.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketCommitmentsDisabled is an event emitted when a market's accepting_commitments option is disabled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketCommitmentsDisabled")]
pub struct EventMarketCommitmentsDisabled {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the accepting_commitments option.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketIntermediaryDenomUpdated is an event emitted when a market updates its
/// commitment_settlement_intermediary_denom field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketIntermediaryDenomUpdated")]
pub struct EventMarketIntermediaryDenomUpdated {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the intermediary denom.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketPermissionsUpdated is an event emitted when a market's permissions are updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketPermissionsUpdated")]
pub struct EventMarketPermissionsUpdated {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the permissions.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketReqAttrUpdated is an event emitted when a market's required attributes are updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketReqAttrUpdated")]
pub struct EventMarketReqAttrUpdated {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// updated_by is the account that updated the required attributes.
    #[prost(string, tag = "2")]
    pub updated_by: ::prost::alloc::string::String,
}
/// EventMarketCreated is an event emitted when a market has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketCreated")]
pub struct EventMarketCreated {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
}
/// EventMarketFeesUpdated is an event emitted when a market's fees have been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventMarketFeesUpdated")]
pub struct EventMarketFeesUpdated {
    /// market_id is the numerical identifier of the market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
}
/// EventParamsUpdated is an event emitted when the exchange module's params have been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventParamsUpdated")]
pub struct EventParamsUpdated {}
/// EventPaymentCreated is an event emitted when a payment is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventPaymentCreated")]
pub struct EventPaymentCreated {
    /// source is the account that created the Payment.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// source_amount is the coins amount string of the funds that the source will pay (to the target).
    #[prost(string, tag = "2")]
    pub source_amount: ::prost::alloc::string::String,
    /// target is the account that can accept the Payment.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// target_amount is the coins amount string of the funds that the target will pay (to the source).
    #[prost(string, tag = "4")]
    pub target_amount: ::prost::alloc::string::String,
    /// external_id is used along with the source to uniquely identify this Payment.
    #[prost(string, tag = "5")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventPaymentUpdated is an event emitted when a payment is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventPaymentUpdated")]
pub struct EventPaymentUpdated {
    /// source is the account that updated (and previously created) the Payment.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// source_amount is the coins amount string of the funds that the source will pay (to the target).
    #[prost(string, tag = "2")]
    pub source_amount: ::prost::alloc::string::String,
    /// old_target is the account that used to be able to accept the Payment (but not any more).
    #[prost(string, tag = "3")]
    pub old_target: ::prost::alloc::string::String,
    /// new_target is the account that is now able to accept the Payment.
    #[prost(string, tag = "4")]
    pub new_target: ::prost::alloc::string::String,
    /// target_amount is the coins amount string of the funds that the target will pay (to the source).
    #[prost(string, tag = "5")]
    pub target_amount: ::prost::alloc::string::String,
    /// external_id is used along with the source to uniquely identify this Payment.
    #[prost(string, tag = "6")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventPaymentAccepted is an event emitted when a payment is accepted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventPaymentAccepted")]
pub struct EventPaymentAccepted {
    /// source is the account that created the Payment.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// source_amount is the coins amount string of the funds that the source will pay (to the target).
    #[prost(string, tag = "2")]
    pub source_amount: ::prost::alloc::string::String,
    /// target is the account that accepted the Payment.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// target_amount is the coins amount string of the funds that the target will pay (to the source).
    #[prost(string, tag = "4")]
    pub target_amount: ::prost::alloc::string::String,
    /// external_id is used along with the source to uniquely identify this Payment.
    #[prost(string, tag = "5")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventPaymentRejected is an event emitted when a payment is rejected (by the target).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventPaymentRejected")]
pub struct EventPaymentRejected {
    /// source is the account that created the Payment.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// target is the account that rejected the Payment.
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// external_id is used along with the source to uniquely identify this Payment.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// EventPaymentCancelled is an event emitted when a payment is cancelled (by the source).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.EventPaymentCancelled")]
pub struct EventPaymentCancelled {
    /// source is the account that cancelled (and created) the Payment.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// target is the account that could have accepted the Payment.
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// external_id is used along with the source to uniquely identify this Payment.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// MarketAccount is an account type for use with the accounts module to hold some basic information about a market.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MarketAccount")]
pub struct MarketAccount {
    /// base_account is the base cosmos account information.
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    /// market_id is the numerical identifier for this market.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// market_details is some human-consumable information about this market.
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
}
/// MarketDetails contains information about a market.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MarketDetails")]
pub struct MarketDetails {
    /// name is a moniker that people can use to refer to this market.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// description extra information about this market. The field is meant to be human-readable.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// website_url is a url people can use to get to this market, or at least get more information about this market.
    #[prost(string, tag = "3")]
    pub website_url: ::prost::alloc::string::String,
    /// icon_uri is a uri for an icon to associate with this market.
    #[prost(string, tag = "4")]
    pub icon_uri: ::prost::alloc::string::String,
}
/// MarketBrief is a message containing brief, superficial information about a market.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MarketBrief")]
pub struct MarketBrief {
    /// market_id is the numerical identifier for this market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// market_address is the bech32 address string of this market's account.
    #[prost(string, tag = "2")]
    pub market_address: ::prost::alloc::string::String,
    /// market_details is some information about this market.
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
}
/// Market contains all information about a market.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Market")]
pub struct Market {
    /// market_id is the numerical identifier for this market.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// market_details is some information about this market.
    #[prost(message, optional, tag = "2")]
    pub market_details: ::core::option::Option<MarketDetails>,
    /// fee_create_ask_flat is the flat fee charged for creating an ask order.
    /// Each coin entry is a separate option. When an ask is created, one of these must be paid.
    /// If empty, no fee is required to create an ask order.
    #[prost(message, repeated, tag = "3")]
    pub fee_create_ask_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_create_bid_flat is the flat fee charged for creating a bid order.
    /// Each coin entry is a separate option. When a bid is created, one of these must be paid.
    /// If empty, no fee is required to create a bid order.
    #[prost(message, repeated, tag = "4")]
    pub fee_create_bid_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_seller_settlement_flat is the flat fee charged to the seller during settlement.
    /// Each coin entry is a separate option.
    /// When an ask is settled, the seller will pay the amount in the denom that matches the price they received.
    #[prost(message, repeated, tag = "5")]
    pub fee_seller_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_seller_settlement_ratios is the fee to charge a seller during settlement based on the price they are receiving.
    /// The price and fee denoms must be equal for each entry, and only one entry for any given denom is allowed.
    #[prost(message, repeated, tag = "6")]
    pub fee_seller_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    /// fee_buyer_settlement_flat is the flat fee charged to the buyer during settlement.
    /// Each coin entry is a separate option.
    /// When a bid is created, the settlement fees provided must contain one of these.
    #[prost(message, repeated, tag = "7")]
    pub fee_buyer_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_buyer_settlement_ratios is the fee to charge a buyer during settlement based on the price they are spending.
    /// The price and fee denoms do not have to equal. Multiple entries for any given price or fee denom are allowed, but
    /// each price denom to fee denom pair can only have one entry.
    #[prost(message, repeated, tag = "8")]
    pub fee_buyer_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    /// accepting_orders is whether this market is allowing orders to be created for it.
    #[prost(bool, tag = "9")]
    pub accepting_orders: bool,
    /// allow_user_settlement is whether this market allows users to initiate their own settlements.
    /// For example, the FillBids and FillAsks endpoints are available if and only if this is true.
    /// The MarketSettle endpoint is only available to market actors regardless of the value of this field.
    #[prost(bool, tag = "10")]
    pub allow_user_settlement: bool,
    /// access_grants is the list of addresses and permissions granted for this market.
    #[prost(message, repeated, tag = "11")]
    pub access_grants: ::prost::alloc::vec::Vec<AccessGrant>,
    /// req_attr_create_ask is a list of attributes required on an account for it to be allowed to create an ask order.
    /// An account must have all of these attributes in order to create an ask order in this market.
    /// If the list is empty, any account can create ask orders in this market.
    ///
    /// An entry that starts with "*." will match any attributes that end with the rest of it.
    /// E.g. "*.b.a" will match all of "c.b.a", "x.b.a", and "e.d.c.b.a"; but not "b.a", "xb.a", "b.x.a", or "c.b.a.x".
    #[prost(string, repeated, tag = "12")]
    pub req_attr_create_ask: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// req_attr_create_ask is a list of attributes required on an account for it to be allowed to create a bid order.
    /// An account must have all of these attributes in order to create a bid order in this market.
    /// If the list is empty, any account can create bid orders in this market.
    ///
    /// An entry that starts with "*." will match any attributes that end with the rest of it.
    /// E.g. "*.b.a" will match all of "c.b.a", "x.b.a", and "e.d.c.b.a"; but not "b.a", "xb.a", "c.b.x.a", or "c.b.a.x".
    #[prost(string, repeated, tag = "13")]
    pub req_attr_create_bid: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// accepting_commitments is whether the market is allowing users to commit funds to it.
    #[prost(bool, tag = "14")]
    pub accepting_commitments: bool,
    /// fee_create_commitment_flat is the flat fee charged for creating a commitment.
    /// Each coin entry is a separate option. When a commitment is created, one of these must be paid.
    /// If empty, no fee is required to create a commitment.
    #[prost(message, repeated, tag = "15")]
    pub fee_create_commitment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// commitment_settlement_bips is the fraction of a commitment settlement that will be paid to the exchange.
    /// It is represented in basis points (1/100th of 1%, e.g. 0.0001) and is limited to 0 to 10,000 inclusive.
    /// During a commitment settlement, the inputs are summed and NAVs are used to convert that total to the
    /// intermediary denom, then to the fee denom. That is then multiplied by this value to get the fee amount
    /// that will be transferred out of the market's account into the exchange for that settlement.
    ///
    /// Summing the inputs effectively doubles the value of the settlement from what what is usually thought of
    /// as the value of a trade. That should be taken into account when setting this value.
    /// E.g. if two accounts are trading 10apples for 100grapes, the inputs total will be 10apples,100grapes
    /// (which might then be converted to USD then nhash before applying this ratio); Usually, though, the value
    /// of that trade would be viewed as either just 10apples or just 100grapes.
    #[prost(uint32, tag = "16")]
    pub commitment_settlement_bips: u32,
    /// intermediary_denom is the denom that funds get converted to (before being converted to the chain's fee denom)
    /// when calculating the fees that are paid to the exchange. NAVs are used for this conversion and actions will fail
    /// if a NAV is needed but not available.
    #[prost(string, tag = "17")]
    pub intermediary_denom: ::prost::alloc::string::String,
    /// req_attr_create_commitment is a list of attributes required on an account for it to be allowed to create a
    /// commitment. An account must have all of these attributes in order to create a commitment in this market.
    /// If the list is empty, any account can create commitments in this market.
    ///
    /// An entry that starts with "*." will match any attributes that end with the rest of it.
    /// E.g. "*.b.a" will match all of "c.b.a", "x.b.a", and "e.d.c.b.a"; but not "b.a", "xb.a", "c.b.x.a", or "c.b.a.x".
    #[prost(string, repeated, tag = "18")]
    pub req_attr_create_commitment: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// FeeRatio defines a ratio of price amount to fee amount.
/// For an order to be valid, its price must be evenly divisible by a FeeRatio's price.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.FeeRatio")]
pub struct FeeRatio {
    /// price is the unit the order price is divided by to get how much of the fee should apply.
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee is the amount to charge per price unit.
    #[prost(message, optional, tag = "2")]
    pub fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// AddrPermissions associates an address with a list of permissions available for that address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.AccessGrant")]
pub struct AccessGrant {
    /// address is the address that these permissions apply to.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// allowed is the list of permissions available for the address.
    #[prost(enumeration = "Permission", repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
/// Permission defines the different types of permission that can be given to an account for a market.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, SerdeEnumAsInt)]
pub enum Permission {
    /// PERMISSION_UNSPECIFIED is the zero-value Permission; it is an error to use it.
    Unspecified = 0,
    /// PERMISSION_SETTLE is the ability to use the Settle Tx endpoint on behalf of a market.
    Settle = 1,
    /// PERMISSION_SET_IDS is the ability to use the SetOrderExternalID Tx endpoint on behalf of a market.
    SetIds = 2,
    /// PERMISSION_CANCEL is the ability to use the Cancel Tx endpoint on behalf of a market.
    Cancel = 3,
    /// PERMISSION_WITHDRAW is the ability to use the MarketWithdraw Tx endpoint.
    Withdraw = 4,
    /// PERMISSION_UPDATE is the ability to use the MarketUpdate* Tx endpoints.
    Update = 5,
    /// PERMISSION_PERMISSIONS is the ability to use the MarketManagePermissions Tx endpoint.
    Permissions = 6,
    /// PERMISSION_ATTRIBUTES is the ability to use the MarketManageReqAttrs Tx endpoint.
    Attributes = 7,
}
impl Permission {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Permission::Unspecified => "PERMISSION_UNSPECIFIED",
            Permission::Settle => "PERMISSION_SETTLE",
            Permission::SetIds => "PERMISSION_SET_IDS",
            Permission::Cancel => "PERMISSION_CANCEL",
            Permission::Withdraw => "PERMISSION_WITHDRAW",
            Permission::Update => "PERMISSION_UPDATE",
            Permission::Permissions => "PERMISSION_PERMISSIONS",
            Permission::Attributes => "PERMISSION_ATTRIBUTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PERMISSION_UNSPECIFIED" => Some(Self::Unspecified),
            "PERMISSION_SETTLE" => Some(Self::Settle),
            "PERMISSION_SET_IDS" => Some(Self::SetIds),
            "PERMISSION_CANCEL" => Some(Self::Cancel),
            "PERMISSION_WITHDRAW" => Some(Self::Withdraw),
            "PERMISSION_UPDATE" => Some(Self::Update),
            "PERMISSION_PERMISSIONS" => Some(Self::Permissions),
            "PERMISSION_ATTRIBUTES" => Some(Self::Attributes),
            _ => None,
        }
    }
}
/// Order associates an order id with one of the order types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Order")]
pub struct Order {
    /// order_id is the numerical identifier for this order.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// order is the specifics of this order.
    #[prost(oneof = "order::Order", tags = "2, 3")]
    pub order: ::core::option::Option<order::Order>,
}
/// Nested message and enum types in `Order`.
pub mod order {
    use provwasm_proc_macro::{CosmwasmExt, SerdeEnumAsInt};
    /// order is the specifics of this order.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::schemars::JsonSchema,
    )]
    pub enum Order {
        /// ask_order is the information about this order if it represents an ask order.
        #[prost(message, tag = "2")]
        AskOrder(super::AskOrder),
        /// bid_order is the information about this order if it represents a bid order.
        #[prost(message, tag = "3")]
        BidOrder(super::BidOrder),
    }
}
/// AskOrder represents someone's desire to sell something at a minimum price.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.AskOrder")]
pub struct AskOrder {
    /// market_id identifies the market that this order belongs to.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// seller is the address of the account that owns this order and has the assets to sell.
    #[prost(string, tag = "2")]
    pub seller: ::prost::alloc::string::String,
    /// assets are the things that the seller wishes to sell.
    /// A hold is placed on this until the order is filled or cancelled.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// price is the minimum amount that the seller is willing to accept for the assets. The seller's settlement
    /// proportional fee (and possibly the settlement flat fee) is taken out of the amount the seller receives,
    /// so it's possible that the seller will still receive less than this price.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// seller_settlement_flat_fee is the flat fee for sellers that will be charged during settlement. If this denom is the
    /// same denom as the price, it will come out of the actual price received. If this denom is different, the amount must
    /// be in the seller's account and a hold is placed on it until the order is filled or cancelled.
    #[prost(message, optional, tag = "5")]
    pub seller_settlement_flat_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// allow_partial should be true if partial fulfillment of this order should be allowed, and should be false if the
    /// order must be either filled in full or not filled at all.
    #[prost(bool, tag = "6")]
    pub allow_partial: bool,
    /// external_id is an optional string used to externally identify this order. Max length is 100 characters.
    /// If an order in this market with this external id already exists, this order will be rejected.
    #[prost(string, tag = "7")]
    pub external_id: ::prost::alloc::string::String,
}
/// BidOrder represents someone's desire to buy something at a specific price.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.BidOrder")]
pub struct BidOrder {
    /// market_id identifies the market that this order belongs to.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// buyer is the address of the account that owns this order and has the price to spend.
    #[prost(string, tag = "2")]
    pub buyer: ::prost::alloc::string::String,
    /// assets are the things that the buyer wishes to buy.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// price is the amount that the buyer will pay for the assets.
    /// A hold is placed on this until the order is filled or cancelled.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// buyer_settlement_fees are the fees (both flat and proportional) that the buyer will pay (in addition to the price)
    /// when the order is settled. A hold is placed on this until the order is filled or cancelled.
    #[prost(message, repeated, tag = "5")]
    pub buyer_settlement_fees:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// allow_partial should be true if partial fulfillment of this order should be allowed, and should be false if the
    /// order must be either filled in full or not filled at all.
    #[prost(bool, tag = "6")]
    pub allow_partial: bool,
    /// external_id is an optional string used to externally identify this order. Max length is 100 characters.
    /// If an order in this market with this external id already exists, this order will be rejected.
    #[prost(string, tag = "7")]
    pub external_id: ::prost::alloc::string::String,
}
/// Params is a representation of the exchange module parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Params")]
pub struct Params {
    /// default_split is the default proportion of fees the exchange receives in basis points.
    /// It is used if there isn't an applicable denom-specific split defined.
    /// E.g. 100 = 1%. Min = 0, Max = 10000.
    #[prost(uint32, tag = "1")]
    pub default_split: u32,
    /// denom_splits are the denom-specific amounts the exchange receives.
    #[prost(message, repeated, tag = "2")]
    pub denom_splits: ::prost::alloc::vec::Vec<DenomSplit>,
    /// fee_create_payment_flat is the flat fee options for creating a payment.
    /// If the source amount is not zero then one of these fee entries is required to create the payment.
    /// This field is currently limited to zero or one entries.
    #[prost(message, repeated, tag = "3")]
    pub fee_create_payment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_accept_payment_flat is the flat fee options for accepting a payment.
    /// If the target amount is not zero then one of these fee entries is required to accept the payment.
    /// This field is currently limited to zero or one entries.
    #[prost(message, repeated, tag = "4")]
    pub fee_accept_payment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// DenomSplit associates a coin denomination with an amount the exchange receives for that denom.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.DenomSplit")]
pub struct DenomSplit {
    /// denom is the coin denomination this split applies to.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// split is the proportion of fees the exchange receives for this denom in basis points.
    /// E.g. 100 = 1%. Min = 0, Max = 10000.
    #[prost(uint32, tag = "2")]
    pub split: u32,
}
/// Payment represents one account's desire to trade funds with another account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.Payment")]
pub struct Payment {
    /// source is the account that created this Payment. It is considered the owner of the payment.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// source_amount is the funds that the source is will pay the target in exchange for the target_amount.
    /// A hold will be placed on this amount in the source account until this Payment is accepted, rejected or cancelled.
    /// If the source_amount is zero, this Payment can be considered a "payment request."
    #[prost(message, repeated, tag = "2")]
    pub source_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// target is the account that can accept this Payment.
    /// The target is the only thing allowed to change in a payment.
    /// I.e. it can be empty initially and updated later as needed.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// target_amount is the funds that the target will pay the source in exchange for the source_amount.
    /// If the target_amount is zero, this Payment can be considered a "peer-to-peer (P2P) payment."
    #[prost(message, repeated, tag = "4")]
    pub target_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// external_id is used along with the source to uniquely identify this Payment.
    ///
    /// A source can only have one Payment with any given external id.
    /// A source can have two payments with two different external ids.
    /// Two different sources can each have a payment with the same external id.
    /// But a source cannot have two different payments each with the same external id.
    ///
    /// An external id can be reused by a source once the payment is accepted, rejected, or cancelled.
    ///
    /// The external id is limited to 100 bytes. An empty string is a valid external id.
    #[prost(string, tag = "5")]
    pub external_id: ::prost::alloc::string::String,
}
/// GenesisState is the data that should be loaded into the exchange module during genesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the exchange module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// markets are all of the markets to create at genesis.
    #[prost(message, repeated, tag = "2")]
    pub markets: ::prost::alloc::vec::Vec<Market>,
    /// orders are all the orders to create at genesis.
    #[prost(message, repeated, tag = "3")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// last_market_id is the value of the last auto-selected market id.
    #[prost(uint32, tag = "4")]
    pub last_market_id: u32,
    /// last_order_id is the value of the last order id created.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_order_id: u64,
    /// commitments are all of the commitments to create at genesis.
    #[prost(message, repeated, tag = "6")]
    pub commitments: ::prost::alloc::vec::Vec<Commitment>,
    /// payments are all the payments to create at genesis.
    #[prost(message, repeated, tag = "7")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
}
/// MsgCreateAskRequest is a request message for the CreateAsk endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateAskRequest")]
pub struct MsgCreateAskRequest {
    /// ask_order is the details of the order being created.
    #[prost(message, optional, tag = "1")]
    pub ask_order: ::core::option::Option<AskOrder>,
    /// order_creation_fee is the fee that is being paid to create this order.
    #[prost(message, optional, tag = "2")]
    pub order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgCreateAskResponse is a response message for the CreateAsk endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateAskResponse")]
pub struct MsgCreateAskResponse {
    /// order_id is the id of the order created.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
}
/// MsgCreateBidRequest is a request message for the CreateBid endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateBidRequest")]
pub struct MsgCreateBidRequest {
    /// bid_order is the details of the order being created.
    #[prost(message, optional, tag = "1")]
    pub bid_order: ::core::option::Option<BidOrder>,
    /// order_creation_fee is the fee that is being paid to create this order.
    #[prost(message, optional, tag = "2")]
    pub order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgCreateBidResponse is a response message for the CreateBid endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreateBidResponse")]
pub struct MsgCreateBidResponse {
    /// order_id is the id of the order created.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
}
/// MsgCommitFundsRequest is a request message for the CommitFunds endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCommitFundsRequest")]
pub struct MsgCommitFundsRequest {
    /// account is the address of the account with the funds being committed.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market the funds will be committed to.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// amount is the funds being committed to the market.
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// creation_fee is the fee that is being paid to create this commitment.
    #[prost(message, optional, tag = "4")]
    pub creation_fee: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// event_tag is a string that is included in the funds-committed event. Max length is 100 characters.
    #[prost(string, tag = "5")]
    pub event_tag: ::prost::alloc::string::String,
}
/// MsgCommitFundsResponse is a response message for the CommitFunds endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCommitFundsResponse")]
pub struct MsgCommitFundsResponse {}
/// MsgCancelOrderRequest is a request message for the CancelOrder endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelOrderRequest")]
pub struct MsgCancelOrderRequest {
    /// signer is the account requesting the order cancellation.
    /// It must be either the order owner (e.g. the buyer or seller), the governance module account address, or an account
    /// with cancel permission with the market that the order is in.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// order_id is the id of the order to cancel.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
}
/// MsgCancelOrderResponse is a response message for the CancelOrder endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelOrderResponse")]
pub struct MsgCancelOrderResponse {}
/// MsgFillBidsRequest is a request message for the FillBids endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillBidsRequest")]
pub struct MsgFillBidsRequest {
    /// seller is the address of the account with the assets to sell.
    #[prost(string, tag = "1")]
    pub seller: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market with the bids to fill.
    /// All bid orders being filled must be in this market.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// total_assets are the things that the seller wishes to sell.
    /// It must be the sum of all bid order assets.
    #[prost(message, repeated, tag = "3")]
    pub total_assets: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// bid_order_ids are the ids of the bid orders that you are trying to fill.
    /// All ids must be for bid orders, and must be in the same market as the market_id.
    #[prost(uint64, repeated, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub bid_order_ids: ::prost::alloc::vec::Vec<u64>,
    /// seller_settlement_flat_fee is the flat fee for sellers that will be charged for this settlement.
    #[prost(message, optional, tag = "5")]
    pub seller_settlement_flat_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// ask_order_creation_fee is the fee that is being paid to create this order (which is immediately then settled).
    #[prost(message, optional, tag = "6")]
    pub ask_order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgFillBidsResponse is a response message for the FillBids endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillBidsResponse")]
pub struct MsgFillBidsResponse {}
/// MsgFillAsksRequest is a request message for the FillAsks endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillAsksRequest")]
pub struct MsgFillAsksRequest {
    /// buyer is the address of the account attempting to buy some assets.
    #[prost(string, tag = "1")]
    pub buyer: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market with the asks to fill.
    /// All ask orders being filled must be in this market.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// total_price is the total amount being spent on some assets.
    /// It must be the sum of all ask order prices.
    #[prost(message, optional, tag = "3")]
    pub total_price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// ask_order_ids are the ids of the ask orders that you are trying to fill.
    /// All ids must be for ask orders, and must be in the same market as the market_id.
    #[prost(uint64, repeated, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub ask_order_ids: ::prost::alloc::vec::Vec<u64>,
    /// buyer_settlement_fees are the fees (both flat and proportional) that the buyer will pay (in addition to the price)
    /// for this settlement.
    #[prost(message, repeated, tag = "5")]
    pub buyer_settlement_fees:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// bid_order_creation_fee is the fee that is being paid to create this order (which is immediately then settled).
    #[prost(message, optional, tag = "6")]
    pub bid_order_creation_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgFillAsksResponse is a response message for the FillAsks endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgFillAsksResponse")]
pub struct MsgFillAsksResponse {}
/// MsgMarketSettleRequest is a request message for the MarketSettle endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSettleRequest")]
pub struct MsgMarketSettleRequest {
    /// admin is the account with "settle" permission requesting this settlement.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market requesting this settlement.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// ask_order_ids are the ask orders being filled.
    #[prost(uint64, repeated, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub ask_order_ids: ::prost::alloc::vec::Vec<u64>,
    /// bid_order_ids are the bid orders being filled.
    #[prost(uint64, repeated, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str_vec::serialize",
        deserialize_with = "crate::serde::as_str_vec::deserialize"
    )]
    pub bid_order_ids: ::prost::alloc::vec::Vec<u64>,
    /// expect_partial is whether to expect an order to only be partially filled. Set to true to indicate that either
    /// the last ask order, or last bid order will be partially filled by this settlement. Set to false to indicate
    /// that all provided orders will be filled in full during this settlement.
    #[prost(bool, tag = "5")]
    pub expect_partial: bool,
}
/// MsgMarketSettleResponse is a response message for the MarketSettle endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSettleResponse")]
pub struct MsgMarketSettleResponse {}
/// MsgMarketCommitmentSettleRequest is a request message for the MarketCommitmentSettle endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketCommitmentSettleRequest")]
pub struct MsgMarketCommitmentSettleRequest {
    /// admin is the account with "settle" permission requesting this settlement.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market requesting this settlement.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// inputs defines where the funds are coming from. All of these funds must be already committed to the market.
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<AccountAmount>,
    /// outputs defines how the funds are to be distributed. These funds will be re-committed in the destination accounts.
    #[prost(message, repeated, tag = "4")]
    pub outputs: ::prost::alloc::vec::Vec<AccountAmount>,
    /// fees is the funds that the market is collecting as part of this settlement.
    /// All of these funds must be already committed to the market.
    #[prost(message, repeated, tag = "5")]
    pub fees: ::prost::alloc::vec::Vec<AccountAmount>,
    /// navs are any NAV info that should be updated at the beginning of this settlement.
    #[prost(message, repeated, tag = "6")]
    pub navs: ::prost::alloc::vec::Vec<NetAssetPrice>,
    /// event_tag is a string that is included in the funds-committed/released events. Max length is 100 characters.
    #[prost(string, tag = "7")]
    pub event_tag: ::prost::alloc::string::String,
}
/// MsgMarketCommitmentSettleResponse is a response message for the MarketCommitmentSettle endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketCommitmentSettleResponse")]
pub struct MsgMarketCommitmentSettleResponse {}
/// MsgMarketReleaseCommitmentsRequest is a request message for the MarketReleaseCommitments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketReleaseCommitmentsRequest")]
pub struct MsgMarketReleaseCommitmentsRequest {
    /// admin is the account with "cancel" permission requesting this release.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market releasing these funds.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// to_release is the funds that are to be released.
    /// An entry with a zero amount indicates that all committed funds for that account should be released.
    #[prost(message, repeated, tag = "3")]
    pub to_release: ::prost::alloc::vec::Vec<AccountAmount>,
    /// event_tag is a string that is included in the funds-released events. Max length is 100 characters.
    #[prost(string, tag = "4")]
    pub event_tag: ::prost::alloc::string::String,
}
/// MsgMarketReleaseCommitmentsResponse is a response message for the MarketReleaseCommitments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketReleaseCommitmentsResponse")]
pub struct MsgMarketReleaseCommitmentsResponse {}
/// MsgMarketSetOrderExternalIDRequest is a request message for the MarketSetOrderExternalID endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSetOrderExternalIDRequest")]
pub struct MsgMarketSetOrderExternalIdRequest {
    /// admin is the account with "set_ids" permission requesting this settlement.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market with the orders to update.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// order_id is the numerical identifier of the order to update.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
    /// external_id is the new external id to associate with the order. Max length is 100 characters.
    /// If the external id is already associated with another order in this market, this update will fail.
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
}
/// MsgMarketSetOrderExternalIDResponse is a response message for the MarketSetOrderExternalID endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketSetOrderExternalIDResponse")]
pub struct MsgMarketSetOrderExternalIdResponse {}
/// MsgMarketWithdrawRequest is a request message for the MarketWithdraw endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketWithdrawRequest")]
pub struct MsgMarketWithdrawRequest {
    /// admin is the account with withdraw permission requesting the withdrawal.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to withdraw from.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// to_address is the address that will receive the funds.
    #[prost(string, tag = "3")]
    pub to_address: ::prost::alloc::string::String,
    /// amount is the funds to withdraw.
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgMarketWithdrawResponse is a response message for the MarketWithdraw endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketWithdrawResponse")]
pub struct MsgMarketWithdrawResponse {}
/// MsgMarketUpdateDetailsRequest is a request message for the MarketUpdateDetails endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateDetailsRequest")]
pub struct MsgMarketUpdateDetailsRequest {
    /// admin is the account with "update" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to update details for.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// market_details is some information about this market.
    #[prost(message, optional, tag = "3")]
    pub market_details: ::core::option::Option<MarketDetails>,
}
/// MsgMarketUpdateDetailsResponse is a response message for the MarketUpdateDetails endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateDetailsResponse")]
pub struct MsgMarketUpdateDetailsResponse {}
/// MsgMarketUpdateEnabledRequest is a request message for the MarketUpdateEnabled endpoint.
/// Deprecated: This endpoint is no longer usable. It is replaced by MarketUpdateAcceptingOrders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateEnabledRequest")]
#[deprecated]
pub struct MsgMarketUpdateEnabledRequest {
    /// admin is the account with "update" permission requesting this change.
    /// Deprecated: This endpoint is no longer usable. It is replaced by MarketUpdateAcceptingOrders.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to enable or disable.
    /// Deprecated: This endpoint is no longer usable. It is replaced by MarketUpdateAcceptingOrders.
    #[deprecated]
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// accepting_orders is whether this market is allowing orders to be created for it.
    /// Deprecated: This endpoint is no longer usable. It is replaced by MarketUpdateAcceptingOrders.
    #[deprecated]
    #[prost(bool, tag = "3")]
    pub accepting_orders: bool,
}
/// MsgMarketUpdateEnabledResponse is a response message for the MarketUpdateEnabled endpoint.
/// Deprecated: This endpoint is no longer usable. It is replaced by MarketUpdateAcceptingOrders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateEnabledResponse")]
#[deprecated]
pub struct MsgMarketUpdateEnabledResponse {}
/// MsgMarketUpdateAcceptingOrdersRequest is a request message for the MarketUpdateAcceptingOrders endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingOrdersRequest")]
pub struct MsgMarketUpdateAcceptingOrdersRequest {
    /// admin is the account with "update" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to enable or disable.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// accepting_orders is whether this market is allowing orders to be created for it.
    #[prost(bool, tag = "3")]
    pub accepting_orders: bool,
}
/// MsgMarketUpdateAcceptingOrdersResponse is a response message for the MarketUpdateAcceptingOrders endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingOrdersResponse")]
pub struct MsgMarketUpdateAcceptingOrdersResponse {}
/// MsgMarketUpdateUserSettleRequest is a request message for the MarketUpdateUserSettle endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateUserSettleRequest")]
pub struct MsgMarketUpdateUserSettleRequest {
    /// admin is the account with "update" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to enable or disable user-settlement for.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// allow_user_settlement is whether this market allows users to initiate their own settlements.
    /// For example, the FillBids and FillAsks endpoints are available if and only if this is true.
    /// The MarketSettle endpoint is available (only to market actors) regardless of the value of this field.
    #[prost(bool, tag = "3")]
    pub allow_user_settlement: bool,
}
/// MsgMarketUpdateUserSettleResponse is a response message for the MarketUpdateUserSettle endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateUserSettleResponse")]
pub struct MsgMarketUpdateUserSettleResponse {}
/// MsgMarketUpdateAcceptingCommitmentsRequest is a request message for the MarketUpdateAcceptingCommitments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingCommitmentsRequest")]
pub struct MsgMarketUpdateAcceptingCommitmentsRequest {
    /// admin is the account with "update" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to enable or disable commitments for.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// accepting_commitments is whether this market allows users to commit funds to it.
    /// For example, the CommitFunds endpoint is available if and only if this is true.
    /// The MarketCommitmentSettle endpoint is available (only to market actors) regardless of the value of this field.
    #[prost(bool, tag = "3")]
    pub accepting_commitments: bool,
}
/// MsgMarketUpdateAcceptingCommitmentsResponse is a response message for the MarketUpdateAcceptingCommitments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateAcceptingCommitmentsResponse")]
pub struct MsgMarketUpdateAcceptingCommitmentsResponse {}
/// MsgMarketUpdateIntermediaryDenomRequest is a request message for the MarketUpdateIntermediaryDenom endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateIntermediaryDenomRequest")]
pub struct MsgMarketUpdateIntermediaryDenomRequest {
    /// admin is the account with "update" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market changing the intermediary denom.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// intermediary_denom is the new intermediary denom for this market to use.
    #[prost(string, tag = "3")]
    pub intermediary_denom: ::prost::alloc::string::String,
}
/// MsgMarketUpdateIntermediaryDenomResponse is a response message for the MarketUpdateIntermediaryDenom endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketUpdateIntermediaryDenomResponse")]
pub struct MsgMarketUpdateIntermediaryDenomResponse {}
/// MsgMarketManagePermissionsRequest is a request message for the MarketManagePermissions endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManagePermissionsRequest")]
pub struct MsgMarketManagePermissionsRequest {
    /// admin is the account with "permissions" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to manage permissions for.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// revoke_all are addresses that should have all their permissions revoked.
    #[prost(string, repeated, tag = "3")]
    pub revoke_all: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// to_revoke are the specific permissions to remove for addresses.
    #[prost(message, repeated, tag = "4")]
    pub to_revoke: ::prost::alloc::vec::Vec<AccessGrant>,
    /// to_grant are the permissions to grant to addresses.
    #[prost(message, repeated, tag = "5")]
    pub to_grant: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// MsgMarketManagePermissionsResponse is a response message for the MarketManagePermissions endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManagePermissionsResponse")]
pub struct MsgMarketManagePermissionsResponse {}
/// MsgMarketManageReqAttrsRequest is a request message for the MarketManageReqAttrs endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManageReqAttrsRequest")]
pub struct MsgMarketManageReqAttrsRequest {
    /// admin is the account with "attributes" permission requesting this change.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to update required attributes for.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// create_ask_to_add are the attributes that should now also be required to create an ask order.
    #[prost(string, repeated, tag = "3")]
    pub create_ask_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// create_ask_to_remove are the attributes that should no longer be required to create an ask order.
    #[prost(string, repeated, tag = "4")]
    pub create_ask_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// create_bid_to_add are the attributes that should now also be required to create a bid order.
    #[prost(string, repeated, tag = "5")]
    pub create_bid_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// create_bid_to_remove are the attributes that should no longer be required to create a bid order.
    #[prost(string, repeated, tag = "6")]
    pub create_bid_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// create_commitment_to_add are the attributes that should now also be required to create a commitment.
    #[prost(string, repeated, tag = "7")]
    pub create_commitment_to_add: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// create_commitment_to_remove are the attributes that should no longer be required to create a commitment.
    #[prost(string, repeated, tag = "8")]
    pub create_commitment_to_remove: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgMarketManageReqAttrsResponse is a response message for the MarketManageReqAttrs endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgMarketManageReqAttrsResponse")]
pub struct MsgMarketManageReqAttrsResponse {}
/// MsgCreatePaymentRequest is a request message for the CreatePayment endpoint.
///
/// The signer is the payment.source, but we can't define that using the cosmos.msg.v1.signer option.
/// So signers for this msg are defined in code using a custom get-signers function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreatePaymentRequest")]
pub struct MsgCreatePaymentRequest {
    /// payment is the details of the payment to create.
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
}
/// MsgCreatePaymentResponse is a response message for the CreatePayment endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCreatePaymentResponse")]
pub struct MsgCreatePaymentResponse {}
/// MsgAcceptPaymentRequest is a request message for the AcceptPayment endpoint.
///
/// The signer is the payment.target, but we can't define that using the cosmos.msg.v1.signer option.
/// So signers for this msg are defined in code using a custom get-signers function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgAcceptPaymentRequest")]
pub struct MsgAcceptPaymentRequest {
    /// payment is the details of the payment to accept.
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
}
/// MsgAcceptPaymentResponse is a response message for the AcceptPayment endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgAcceptPaymentResponse")]
pub struct MsgAcceptPaymentResponse {}
/// MsgRejectPaymentRequest is a request message for the RejectPayment endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentRequest")]
pub struct MsgRejectPaymentRequest {
    /// target is the target account of the payment to reject.
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    /// source is the source account of the payment to reject.
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// external_id is the external id of the payment to reject.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// MsgRejectPaymentResponse is a response message for the RejectPayment endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentResponse")]
pub struct MsgRejectPaymentResponse {}
/// MsgRejectPaymentsRequest is a request message for the RejectPayments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentsRequest")]
pub struct MsgRejectPaymentsRequest {
    /// target is the account that wishes to reject some payments.
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    /// sources is the source accounts of the payments to reject.
    #[prost(string, repeated, tag = "2")]
    pub sources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgRejectPaymentsResponse is a response message for the RejectPayments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgRejectPaymentsResponse")]
pub struct MsgRejectPaymentsResponse {}
/// MsgCancelPaymentsRequest is a request message for the CancelPayments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelPaymentsRequest")]
pub struct MsgCancelPaymentsRequest {
    /// source is the account that wishes to cancel some of their payments.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// external_ids is all of the external ids of the payments to cancel.
    #[prost(string, repeated, tag = "2")]
    pub external_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgCancelPaymentsResponse is a response message for the CancelPayments endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgCancelPaymentsResponse")]
pub struct MsgCancelPaymentsResponse {}
/// MsgChangePaymentTargetRequest is a request message for the ChangePaymentTarget endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgChangePaymentTargetRequest")]
pub struct MsgChangePaymentTargetRequest {
    /// source is the account that wishes to update the target of one of their payments.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// external_id is the external id of the payment to update.
    #[prost(string, tag = "2")]
    pub external_id: ::prost::alloc::string::String,
    /// new_target is the new target account of the payment.
    #[prost(string, tag = "3")]
    pub new_target: ::prost::alloc::string::String,
}
/// MsgChangePaymentTargetResponse is a response message for the ChangePaymentTarget endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgChangePaymentTargetResponse")]
pub struct MsgChangePaymentTargetResponse {}
/// MsgGovCreateMarketRequest is a request message for the GovCreateMarket endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCreateMarketRequest")]
pub struct MsgGovCreateMarketRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// market is the initial market configuration.
    /// If the market_id is 0, the next available market_id will be used (once voting ends).
    /// If it is not zero, it must not yet be in use when the voting period ends.
    #[prost(message, optional, tag = "2")]
    pub market: ::core::option::Option<Market>,
}
/// MsgGovCreateMarketResponse is a response message for the GovCreateMarket endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCreateMarketResponse")]
pub struct MsgGovCreateMarketResponse {}
/// MsgGovManageFeesRequest is a request message for the GovManageFees endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovManageFeesRequest")]
pub struct MsgGovManageFeesRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// market_id is the market id that will get these fee updates.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
    /// add_fee_create_ask_flat are the create-ask flat fee options to add.
    #[prost(message, repeated, tag = "3")]
    pub add_fee_create_ask_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// remove_fee_create_ask_flat are the create-ask flat fee options to remove.
    #[prost(message, repeated, tag = "4")]
    pub remove_fee_create_ask_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// add_fee_create_bid_flat are the create-bid flat fee options to add.
    #[prost(message, repeated, tag = "5")]
    pub add_fee_create_bid_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// remove_fee_create_bid_flat are the create-bid flat fee options to remove.
    #[prost(message, repeated, tag = "6")]
    pub remove_fee_create_bid_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// add_fee_seller_settlement_flat are the seller settlement flat fee options to add.
    #[prost(message, repeated, tag = "7")]
    pub add_fee_seller_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// remove_fee_seller_settlement_flat are the seller settlement flat fee options to remove.
    #[prost(message, repeated, tag = "8")]
    pub remove_fee_seller_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// add_fee_seller_settlement_ratios are the seller settlement fee ratios to add.
    #[prost(message, repeated, tag = "9")]
    pub add_fee_seller_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    /// remove_fee_seller_settlement_ratios are the seller settlement fee ratios to remove.
    #[prost(message, repeated, tag = "10")]
    pub remove_fee_seller_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    /// add_fee_buyer_settlement_flat are the buyer settlement flat fee options to add.
    #[prost(message, repeated, tag = "11")]
    pub add_fee_buyer_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// remove_fee_buyer_settlement_flat are the buyer settlement flat fee options to remove.
    #[prost(message, repeated, tag = "12")]
    pub remove_fee_buyer_settlement_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// add_fee_buyer_settlement_ratios are the buyer settlement fee ratios to add.
    #[prost(message, repeated, tag = "13")]
    pub add_fee_buyer_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    /// remove_fee_buyer_settlement_ratios are the buyer settlement fee ratios to remove.
    #[prost(message, repeated, tag = "14")]
    pub remove_fee_buyer_settlement_ratios: ::prost::alloc::vec::Vec<FeeRatio>,
    /// add_fee_create_commitment_flat are the create-commitment flat fee options to add.
    #[prost(message, repeated, tag = "15")]
    pub add_fee_create_commitment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// remove_fee_create_commitment_flat are the create-commitment flat fee options to remove.
    #[prost(message, repeated, tag = "16")]
    pub remove_fee_create_commitment_flat:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// set_fee_commitment_settlement_bips is the new fee_commitment_settlement_bips for the market.
    /// It is ignored if it is zero. To set it to zero set unset_fee_commitment_settlement_bips to true.
    #[prost(uint32, tag = "17")]
    pub set_fee_commitment_settlement_bips: u32,
    /// unset_fee_commitment_settlement_bips, if true, sets the fee_commitment_settlement_bips to zero.
    /// If false, it is ignored.
    #[prost(bool, tag = "18")]
    pub unset_fee_commitment_settlement_bips: bool,
}
/// MsgGovManageFeesResponse is a response message for the GovManageFees endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovManageFeesResponse")]
pub struct MsgGovManageFeesResponse {}
/// MsgGovCloseMarketRequest is a request message for the GovCloseMarket endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCloseMarketRequest")]
pub struct MsgGovCloseMarketRequest {
    /// authority must be the governance module account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// market_id is the numerical identifier of the market to close.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
}
/// MsgGovCloseMarketResponse is a response message for the GovCloseMarket endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovCloseMarketResponse")]
pub struct MsgGovCloseMarketResponse {}
/// MsgGovUpdateParamsRequest is a request message for the GovUpdateParams endpoint.
/// Deprecated: Use MsgUpdateParamsRequest instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovUpdateParamsRequest")]
#[deprecated]
pub struct MsgGovUpdateParamsRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params are the new param values to set
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgGovUpdateParamsResponse is a response message for the GovUpdateParams endpoint.
/// Deprecated: Use MsgUpdateParamsResponse instead.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgGovUpdateParamsResponse")]
#[deprecated]
pub struct MsgGovUpdateParamsResponse {}
/// MsgGovUpdateParamsRequest is a request message for the GovUpdateParams endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgUpdateParamsRequest")]
pub struct MsgUpdateParamsRequest {
    /// authority should be the governance module account address.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params are the new param values to set
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse is a response message for the GovUpdateParams endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// QueryOrderFeeCalcRequest is a request message for the OrderFeeCalc query.
/// Exactly one of ask_order or bid_order must be provided.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryOrderFeeCalcRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/OrderFeeCalc",
    response_type = QueryOrderFeeCalcResponse
)]
pub struct QueryOrderFeeCalcRequest {
    /// ask_order is the ask order to calculate the fees for.
    #[prost(message, optional, tag = "2")]
    pub ask_order: ::core::option::Option<AskOrder>,
    /// bid_order is the bid order to calculate the fees for.
    #[prost(message, optional, tag = "3")]
    pub bid_order: ::core::option::Option<BidOrder>,
}
/// QueryOrderFeeCalcResponse is a response message for the OrderFeeCalc query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryOrderFeeCalcResponse")]
pub struct QueryOrderFeeCalcResponse {
    /// creation_fee_options are the order creation flat fee options available for creating the provided order.
    /// If it's empty, no order creation fee is required.
    /// When creating the order, you should include exactly one of these.
    #[prost(message, repeated, tag = "1")]
    pub creation_fee_options:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// settlement_flat_fee_options are the settlement flat fee options available for the provided order.
    /// If it's empty, no settlement flat fee is required.
    /// When creating an order, you should include exactly one of these in the settlement fees field.
    #[prost(message, repeated, tag = "2")]
    pub settlement_flat_fee_options:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// settlement_ratio_fee_options are the settlement ratio fee options available for the provided order.
    /// If it's empty, no settlement ratio fee is required.
    ///
    /// If the provided order was a bid order, you should include exactly one of these in the settlement fees field.
    /// If the flat and ratio options you've chose have the same denom, a single entry should be included with their sum.
    ///
    /// If the provided order was an ask order, these are purely informational and represent how much will be removed
    /// from your price if it settles at that price. If it settles for more, the actual amount will probably be larger.
    #[prost(message, repeated, tag = "3")]
    pub settlement_ratio_fee_options:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryGetOrderRequest is a request message for the GetOrder query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOrder",
    response_type = QueryGetOrderResponse
)]
pub struct QueryGetOrderRequest {
    /// order_id is the id of the order to look up.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub order_id: u64,
}
/// QueryGetOrderResponse is a response message for the GetOrder query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderResponse")]
pub struct QueryGetOrderResponse {
    /// order is the requested order.
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// QueryGetOrderByExternalIDRequest is a request message for the GetOrderByExternalID query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderByExternalIDRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOrderByExternalID",
    response_type = QueryGetOrderByExternalIdResponse
)]
pub struct QueryGetOrderByExternalIdRequest {
    /// market_id is the id of the market that's expected to have the order.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// external_id the external id to look up.
    #[prost(string, tag = "2")]
    pub external_id: ::prost::alloc::string::String,
}
/// QueryGetOrderByExternalIDResponse is a response message for the GetOrderByExternalID query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOrderByExternalIDResponse")]
pub struct QueryGetOrderByExternalIdResponse {
    /// order is the requested order.
    #[prost(message, optional, tag = "1")]
    pub order: ::core::option::Option<Order>,
}
/// QueryGetMarketOrdersRequest is a request message for the GetMarketOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketOrdersRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarketOrders",
    response_type = QueryGetMarketOrdersResponse
)]
pub struct QueryGetMarketOrdersRequest {
    /// market_id is the id of the market to get all the orders for.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// order_type is optional and can limit orders to only "ask" or "bid" orders.
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    /// after_order_id is a minimum (exclusive) order id. All results will be strictly greater than this.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub after_order_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetMarketOrdersResponse is a response message for the GetMarketOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketOrdersResponse")]
pub struct QueryGetMarketOrdersResponse {
    /// orders are a page of the orders in the provided market.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetOwnerOrdersRequest is a request message for the GetOwnerOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOwnerOrdersRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetOwnerOrders",
    response_type = QueryGetOwnerOrdersResponse
)]
pub struct QueryGetOwnerOrdersRequest {
    /// owner is the bech32 address string of the owner to get the orders for.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// order_type is optional and can limit orders to only "ask" or "bid" orders.
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    /// after_order_id is a minimum (exclusive) order id. All results will be strictly greater than this.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub after_order_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetOwnerOrdersResponse is a response message for the GetOwnerOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetOwnerOrdersResponse")]
pub struct QueryGetOwnerOrdersResponse {
    /// orders are a page of the orders for the provided address.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetAssetOrdersRequest is a request message for the GetAssetOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAssetOrdersRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAssetOrders",
    response_type = QueryGetAssetOrdersResponse
)]
pub struct QueryGetAssetOrdersRequest {
    /// asset is the denom of assets to get orders for.
    #[prost(string, tag = "1")]
    pub asset: ::prost::alloc::string::String,
    /// order_type is optional and can limit orders to only "ask" or "bid" orders.
    #[prost(string, tag = "2")]
    pub order_type: ::prost::alloc::string::String,
    /// after_order_id is a minimum (exclusive) order id. All results will be strictly greater than this.
    #[prost(uint64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub after_order_id: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetAssetOrdersResponse is a response message for the GetAssetOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAssetOrdersResponse")]
pub struct QueryGetAssetOrdersResponse {
    /// orders are a page of the orders for the provided asset.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetAllOrdersRequest is a request message for the GetAllOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllOrdersRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAllOrders",
    response_type = QueryGetAllOrdersResponse
)]
pub struct QueryGetAllOrdersRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetAllOrdersResponse is a response message for the GetAllOrders query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllOrdersResponse")]
pub struct QueryGetAllOrdersResponse {
    /// orders are a page of the all orders.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<Order>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetCommitmentRequest is a request message for the GetCommitment query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetCommitmentRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetCommitment",
    response_type = QueryGetCommitmentResponse
)]
pub struct QueryGetCommitmentRequest {
    /// account is the bech32 address string of the account in the commitment.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
    /// market_id is the numeric identifier of the market in the commitment.
    #[prost(uint32, tag = "2")]
    pub market_id: u32,
}
/// QueryGetCommitmentResponse is a response message for the GetCommitment query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetCommitmentResponse")]
pub struct QueryGetCommitmentResponse {
    /// amount is the total funds committed to the market by the account.
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryGetAccountCommitmentsRequest is a request message for the GetAccountCommitments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAccountCommitmentsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAccountCommitments",
    response_type = QueryGetAccountCommitmentsResponse
)]
pub struct QueryGetAccountCommitmentsRequest {
    /// account is the bech32 address string of the account with the commitments.
    #[prost(string, tag = "1")]
    pub account: ::prost::alloc::string::String,
}
/// QueryGetAccountCommitmentsResponse is a response message for the GetAccountCommitments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAccountCommitmentsResponse")]
pub struct QueryGetAccountCommitmentsResponse {
    /// commitments is the amounts committed from the account to the any market.
    #[prost(message, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<MarketAmount>,
}
/// QueryGetMarketCommitmentsRequest is a request message for the GetMarketCommitments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketCommitmentsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarketCommitments",
    response_type = QueryGetMarketCommitmentsResponse
)]
pub struct QueryGetMarketCommitmentsRequest {
    /// market_id is the numeric identifier of the market with the commitment.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetMarketCommitmentsResponse is a response message for the GetMarketCommitments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketCommitmentsResponse")]
pub struct QueryGetMarketCommitmentsResponse {
    /// commitments is the amounts committed to the market from any account.
    #[prost(message, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<AccountAmount>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetAllCommitmentsRequest is a request message for the GetAllCommitments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllCommitmentsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAllCommitments",
    response_type = QueryGetAllCommitmentsResponse
)]
pub struct QueryGetAllCommitmentsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetAllCommitmentsResponse is a response message for the GetAllCommitments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllCommitmentsResponse")]
pub struct QueryGetAllCommitmentsResponse {
    /// commitments is the requested commitment information.
    #[prost(message, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<Commitment>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetMarketRequest is a request message for the GetMarket query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetMarket",
    response_type = QueryGetMarketResponse
)]
pub struct QueryGetMarketRequest {
    /// market_id is the id of the market to look up.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
}
/// QueryGetMarketResponse is a response message for the GetMarket query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetMarketResponse")]
pub struct QueryGetMarketResponse {
    /// address is the bech32 address string of this market's account.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// market is all information and details of the market.
    #[prost(message, optional, tag = "2")]
    pub market: ::core::option::Option<Market>,
}
/// QueryGetAllMarketsRequest is a request message for the GetAllMarkets query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllMarketsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAllMarkets",
    response_type = QueryGetAllMarketsResponse
)]
pub struct QueryGetAllMarketsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetAllMarketsResponse is a response message for the GetAllMarkets query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllMarketsResponse")]
pub struct QueryGetAllMarketsResponse {
    /// markets are a page of the briefs for all markets.
    #[prost(message, repeated, tag = "1")]
    pub markets: ::prost::alloc::vec::Vec<MarketBrief>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryParamsRequest is a request message for the Params query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is a response message for the Params query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params are the exchange module parameter values.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryCommitmentSettlementFeeCalcRequest is a request message for the CommitmentSettlementFeeCalc query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryCommitmentSettlementFeeCalcRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/CommitmentSettlementFeeCalc",
    response_type = QueryCommitmentSettlementFeeCalcResponse
)]
pub struct QueryCommitmentSettlementFeeCalcRequest {
    /// settlement is a market's commitment settlement request message.
    /// If no inputs are provided, only the to_fee_nav field will be populated in the response.
    #[prost(message, optional, tag = "1")]
    pub settlement: ::core::option::Option<MsgMarketCommitmentSettleRequest>,
    /// include_breakdown_fields controls the fields that are populated in the response.
    /// If false, only the exchange_fees field is populated.
    /// If true, all of the fields are populated as possible.
    /// If the settlement does not have any inputs, this field defaults to true.
    #[prost(bool, tag = "2")]
    pub include_breakdown_fields: bool,
}
/// QueryCommitmentSettlementFeeCalcResponse is a response message for the CommitmentSettlementFeeCalc query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryCommitmentSettlementFeeCalcResponse")]
pub struct QueryCommitmentSettlementFeeCalcResponse {
    /// exchange_fees is the total that the exchange would currently pay for the provided settlement.
    #[prost(message, repeated, tag = "1")]
    pub exchange_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// input_total is the sum of all the inputs in the provided settlement.
    #[prost(message, repeated, tag = "2")]
    pub input_total: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// converted_total is the input_total converted to a single intermediary denom or left as the fee denom.
    #[prost(message, repeated, tag = "3")]
    pub converted_total: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// conversion_navs are the NAVs used to convert the input_total to the converted_total.
    #[prost(message, repeated, tag = "4")]
    pub conversion_navs: ::prost::alloc::vec::Vec<NetAssetPrice>,
    /// to_fee_nav is the NAV used to convert the converted_total into the fee denom.
    #[prost(message, optional, tag = "5")]
    pub to_fee_nav: ::core::option::Option<NetAssetPrice>,
}
/// QueryValidateCreateMarketRequest is a request message for the ValidateCreateMarket query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateCreateMarketRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateCreateMarket",
    response_type = QueryValidateCreateMarketResponse
)]
pub struct QueryValidateCreateMarketRequest {
    /// create_market_request is the request to run validation on.
    #[prost(message, optional, tag = "1")]
    pub create_market_request: ::core::option::Option<MsgGovCreateMarketRequest>,
}
/// QueryValidateCreateMarketResponse is a response message for the ValidateCreateMarket query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateCreateMarketResponse")]
pub struct QueryValidateCreateMarketResponse {
    /// error is any problems or inconsistencies in the provided gov prop msg.
    /// This goes above and beyond the validation done when actually processing the governance proposal.
    /// If an error is returned, and gov_prop_will_pass is true, it means the error is more of an
    /// inconsistency that might cause certain aspects of the market to behave unexpectedly.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    /// gov_prop_will_pass will be true if the the provided msg will be successfully processed at the end of it's voting
    /// period (assuming it passes).
    #[prost(bool, tag = "2")]
    pub gov_prop_will_pass: bool,
}
/// QueryValidateMarketRequest is a request message for the ValidateMarket query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateMarketRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateMarket",
    response_type = QueryValidateMarketResponse
)]
pub struct QueryValidateMarketRequest {
    /// market_id is the id of the market to check.
    #[prost(uint32, tag = "1")]
    pub market_id: u32,
}
/// QueryValidateMarketResponse is a response message for the ValidateMarket query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateMarketResponse")]
pub struct QueryValidateMarketResponse {
    /// error is any problems or inconsistencies in the provided market.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
}
/// QueryValidateManageFeesRequest is a request message for the ValidateManageFees query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateManageFeesRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/ValidateManageFees",
    response_type = QueryValidateManageFeesResponse
)]
pub struct QueryValidateManageFeesRequest {
    /// manage_fees_request is the request to run validation on.
    #[prost(message, optional, tag = "1")]
    pub manage_fees_request: ::core::option::Option<MsgGovManageFeesRequest>,
}
/// QueryValidateManageFeesResponse is a response message for the ValidateManageFees query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryValidateManageFeesResponse")]
pub struct QueryValidateManageFeesResponse {
    /// error is any problems or inconsistencies in the provided gov prop msg.
    /// This goes above and beyond the validation done when actually processing the governance proposal.
    /// If an error is returned, and gov_prop_will_pass is true, it means the error is more of an
    /// inconsistency that might cause certain aspects of the market to behave unexpectedly.
    #[prost(string, tag = "1")]
    pub error: ::prost::alloc::string::String,
    /// gov_prop_will_pass will be true if the the provided msg will be successfully processed at the end of it's voting
    /// period (assuming it passes).
    #[prost(bool, tag = "2")]
    pub gov_prop_will_pass: bool,
}
/// QueryGetPaymentRequest is a request message for the GetPayment query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetPaymentRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetPayment",
    response_type = QueryGetPaymentResponse
)]
pub struct QueryGetPaymentRequest {
    /// source is the source account of the payment to get.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// external_id is the external id of the payment to get.
    #[prost(string, tag = "2")]
    pub external_id: ::prost::alloc::string::String,
}
/// QueryGetPaymentResponse is a response message for the GetPayment query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetPaymentResponse")]
pub struct QueryGetPaymentResponse {
    /// payment is the info on the requested payment.
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
}
/// QueryGetPaymentsWithSourceRequest is a request message for the GetPaymentsWithSource query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetPaymentsWithSourceRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetPaymentsWithSource",
    response_type = QueryGetPaymentsWithSourceResponse
)]
pub struct QueryGetPaymentsWithSourceRequest {
    /// source is the source account of the payments to get.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetPaymentsWithSourceResponse is a response message for the GetPaymentsWithSource query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetPaymentsWithSourceResponse")]
pub struct QueryGetPaymentsWithSourceResponse {
    /// payments is all the payments with the requested source.
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetPaymentsWithTargetRequest is a request message for the GetPaymentsWithTarget query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetPaymentsWithTargetRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetPaymentsWithTarget",
    response_type = QueryGetPaymentsWithTargetResponse
)]
pub struct QueryGetPaymentsWithTargetRequest {
    /// target is the target account of the payments to get.
    #[prost(string, tag = "1")]
    pub target: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetPaymentsWithTargetResponse is a response message for the GetPaymentsWithTarget query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetPaymentsWithTargetResponse")]
pub struct QueryGetPaymentsWithTargetResponse {
    /// payments is all the payments with the requested target.
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryGetAllPaymentsRequest is a request message for the GetAllPayments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllPaymentsRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/GetAllPayments",
    response_type = QueryGetAllPaymentsResponse
)]
pub struct QueryGetAllPaymentsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetAllPaymentsResponse is a response message for the GetAllPayments query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryGetAllPaymentsResponse")]
pub struct QueryGetAllPaymentsResponse {
    /// payments is all the payments on this page of results.
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
    /// pagination is the resulting pagination parameters.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPaymentFeeCalcRequest is a request message for the PaymentFeeCalc query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryPaymentFeeCalcRequest")]
#[proto_query(
    path = "/provenance.exchange.v1.Query/PaymentFeeCalc",
    response_type = QueryPaymentFeeCalcResponse
)]
pub struct QueryPaymentFeeCalcRequest {
    /// payment is the details of the payment to create or accept.
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
}
/// QueryPaymentFeeCalcResponse is a response message for the PaymentFeeCalc query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.exchange.v1.QueryPaymentFeeCalcResponse")]
pub struct QueryPaymentFeeCalcResponse {
    /// fee_create is the fee required to create the provided payment.
    #[prost(message, repeated, tag = "1")]
    pub fee_create: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// fee_accept is the fee required to accept the provided payment.
    #[prost(message, repeated, tag = "2")]
    pub fee_accept: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
pub struct ExchangeQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ExchangeQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn order_fee_calc(
        &self,
        ask_order: ::core::option::Option<AskOrder>,
        bid_order: ::core::option::Option<BidOrder>,
    ) -> Result<QueryOrderFeeCalcResponse, cosmwasm_std::StdError> {
        QueryOrderFeeCalcRequest {
            ask_order,
            bid_order,
        }
        .query(self.querier)
    }
    pub fn get_order(
        &self,
        order_id: u64,
    ) -> Result<QueryGetOrderResponse, cosmwasm_std::StdError> {
        QueryGetOrderRequest { order_id }.query(self.querier)
    }
    pub fn get_order_by_external_id(
        &self,
        market_id: u32,
        external_id: ::prost::alloc::string::String,
    ) -> Result<QueryGetOrderByExternalIdResponse, cosmwasm_std::StdError> {
        QueryGetOrderByExternalIdRequest {
            market_id,
            external_id,
        }
        .query(self.querier)
    }
    pub fn get_market_orders(
        &self,
        market_id: u32,
        order_type: ::prost::alloc::string::String,
        after_order_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetMarketOrdersResponse, cosmwasm_std::StdError> {
        QueryGetMarketOrdersRequest {
            market_id,
            order_type,
            after_order_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_owner_orders(
        &self,
        owner: ::prost::alloc::string::String,
        order_type: ::prost::alloc::string::String,
        after_order_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetOwnerOrdersResponse, cosmwasm_std::StdError> {
        QueryGetOwnerOrdersRequest {
            owner,
            order_type,
            after_order_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_asset_orders(
        &self,
        asset: ::prost::alloc::string::String,
        order_type: ::prost::alloc::string::String,
        after_order_id: u64,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetAssetOrdersResponse, cosmwasm_std::StdError> {
        QueryGetAssetOrdersRequest {
            asset,
            order_type,
            after_order_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_all_orders(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetAllOrdersResponse, cosmwasm_std::StdError> {
        QueryGetAllOrdersRequest { pagination }.query(self.querier)
    }
    pub fn get_commitment(
        &self,
        account: ::prost::alloc::string::String,
        market_id: u32,
    ) -> Result<QueryGetCommitmentResponse, cosmwasm_std::StdError> {
        QueryGetCommitmentRequest { account, market_id }.query(self.querier)
    }
    pub fn get_account_commitments(
        &self,
        account: ::prost::alloc::string::String,
    ) -> Result<QueryGetAccountCommitmentsResponse, cosmwasm_std::StdError> {
        QueryGetAccountCommitmentsRequest { account }.query(self.querier)
    }
    pub fn get_market_commitments(
        &self,
        market_id: u32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetMarketCommitmentsResponse, cosmwasm_std::StdError> {
        QueryGetMarketCommitmentsRequest {
            market_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn get_all_commitments(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetAllCommitmentsResponse, cosmwasm_std::StdError> {
        QueryGetAllCommitmentsRequest { pagination }.query(self.querier)
    }
    pub fn get_market(
        &self,
        market_id: u32,
    ) -> Result<QueryGetMarketResponse, cosmwasm_std::StdError> {
        QueryGetMarketRequest { market_id }.query(self.querier)
    }
    pub fn get_all_markets(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetAllMarketsResponse, cosmwasm_std::StdError> {
        QueryGetAllMarketsRequest { pagination }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn commitment_settlement_fee_calc(
        &self,
        settlement: ::core::option::Option<MsgMarketCommitmentSettleRequest>,
        include_breakdown_fields: bool,
    ) -> Result<QueryCommitmentSettlementFeeCalcResponse, cosmwasm_std::StdError> {
        QueryCommitmentSettlementFeeCalcRequest {
            settlement,
            include_breakdown_fields,
        }
        .query(self.querier)
    }
    pub fn validate_create_market(
        &self,
        create_market_request: ::core::option::Option<MsgGovCreateMarketRequest>,
    ) -> Result<QueryValidateCreateMarketResponse, cosmwasm_std::StdError> {
        QueryValidateCreateMarketRequest {
            create_market_request,
        }
        .query(self.querier)
    }
    pub fn validate_market(
        &self,
        market_id: u32,
    ) -> Result<QueryValidateMarketResponse, cosmwasm_std::StdError> {
        QueryValidateMarketRequest { market_id }.query(self.querier)
    }
    pub fn validate_manage_fees(
        &self,
        manage_fees_request: ::core::option::Option<MsgGovManageFeesRequest>,
    ) -> Result<QueryValidateManageFeesResponse, cosmwasm_std::StdError> {
        QueryValidateManageFeesRequest {
            manage_fees_request,
        }
        .query(self.querier)
    }
    pub fn get_payment(
        &self,
        source: ::prost::alloc::string::String,
        external_id: ::prost::alloc::string::String,
    ) -> Result<QueryGetPaymentResponse, cosmwasm_std::StdError> {
        QueryGetPaymentRequest {
            source,
            external_id,
        }
        .query(self.querier)
    }
    pub fn get_payments_with_source(
        &self,
        source: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetPaymentsWithSourceResponse, cosmwasm_std::StdError> {
        QueryGetPaymentsWithSourceRequest { source, pagination }.query(self.querier)
    }
    pub fn get_payments_with_target(
        &self,
        target: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetPaymentsWithTargetResponse, cosmwasm_std::StdError> {
        QueryGetPaymentsWithTargetRequest { target, pagination }.query(self.querier)
    }
    pub fn get_all_payments(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetAllPaymentsResponse, cosmwasm_std::StdError> {
        QueryGetAllPaymentsRequest { pagination }.query(self.querier)
    }
    pub fn payment_fee_calc(
        &self,
        payment: ::core::option::Option<Payment>,
    ) -> Result<QueryPaymentFeeCalcResponse, cosmwasm_std::StdError> {
        QueryPaymentFeeCalcRequest { payment }.query(self.querier)
    }
}
