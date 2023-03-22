use osmosis_std_derive::CosmwasmExt;
/// GenesisState defines the concentrated liquidity module's genesis state.
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params are all the parameters of the module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
}
/// IncentiveRecordBody represents an active perpetual incentive gauge for a pool
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.IncentiveRecordBody")]
pub struct IncentiveRecordBody {
    /// remaining_amount is the total amount of incentives to be distributed
    #[prost(string, tag = "1")]
    pub remaining_amount: ::prost::alloc::string::String,
    /// emission_rate is the incentive emission rate per second
    #[prost(string, tag = "2")]
    pub emission_rate: ::prost::alloc::string::String,
    /// start_time is the time when the incentive starts distributing
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// Position contains position's pool id, lower tick, upper tick
/// liquidity, join time and freeze duration.
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.Position")]
pub struct Position {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
    #[prost(message, optional, tag = "5")]
    pub join_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub freeze_duration: ::core::option::Option<crate::shim::Duration>,
    #[prost(string, tag = "7")]
    pub liquidity: ::prost::alloc::string::String,
}
/// =============================== Positions
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryUserPositionsRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/UserPositions",
    response_type = QueryUserPositionsResponse
)]
pub struct QueryUserPositionsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryUserPositionsResponse")]
pub struct QueryUserPositionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub positions: ::prost::alloc::vec::Vec<Position>,
}
/// =============================== Pool
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/Pool",
    response_type = QueryPoolResponse
)]
pub struct QueryPoolRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolResponse")]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<crate::shim::Any>,
}
/// =============================== Pools
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolsRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/Pools",
    response_type = QueryPoolsResponse
)]
pub struct QueryPoolsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryPoolsResponse")]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<crate::shim::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// =============================== ModuleParams
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<super::Params>,
}
/// =============================== LiquidityDepthsForRange
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
#[proto_message(
    type_url = "/osmosis.concentratedliquidity.v1beta1.QueryLiquidityDepthsForRangeRequest"
)]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/LiquidityDepthsForRange",
    response_type = QueryLiquidityDepthsForRangeResponse
)]
pub struct QueryLiquidityDepthsForRangeRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub lower_tick: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub upper_tick: ::prost::alloc::string::String,
}
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
#[proto_message(
    type_url = "/osmosis.concentratedliquidity.v1beta1.QueryLiquidityDepthsForRangeResponse"
)]
pub struct QueryLiquidityDepthsForRangeResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidity_depths: ::prost::alloc::vec::Vec<LiquidityDepth>,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.LiquidityDepth")]
pub struct LiquidityDepth {
    #[prost(string, tag = "1")]
    pub liquidity_net: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tick_index: ::prost::alloc::string::String,
}
/// ===================== MsgQueryClaimableFees
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryClaimableFeesRequest")]
#[proto_query(
    path = "/osmosis.concentratedliquidity.v1beta1.Query/ClaimableFees",
    response_type = QueryClaimableFeesResponse
)]
pub struct QueryClaimableFeesRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.QueryClaimableFeesResponse")]
pub struct QueryClaimableFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub claimable_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ===================== MsgCreateConcentratedPool
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreateConcentratedPool")]
pub struct MsgCreateConcentratedPool {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom0: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom1: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_spacing: u64,
    #[prost(string, tag = "5")]
    pub precision_factor_at_price_one: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub swap_fee: ::prost::alloc::string::String,
}
/// Returns a unique poolID to identify the pool with.
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
#[proto_message(
    type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreateConcentratedPoolResponse"
)]
pub struct MsgCreateConcentratedPoolResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.Pool")]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Amount of total liquidity
    #[prost(string, tag = "3")]
    pub current_tick_liquidity: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub token0: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub token1: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub current_sqrt_price: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub current_tick: ::prost::alloc::string::String,
    /// tick_spacing must be one of the authorized_tick_spacing values set in the
    /// concentrated-liquidity parameters
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tick_spacing: u64,
    #[prost(string, tag = "9")]
    pub precision_factor_at_price_one: ::prost::alloc::string::String,
    /// swap_fee is the ratio that is charged on the amount of token in.
    #[prost(string, tag = "10")]
    pub swap_fee: ::prost::alloc::string::String,
    /// last_liquidity_update is the last time either the pool liquidity or the
    /// active tick changed
    #[prost(message, optional, tag = "11")]
    pub last_liquidity_update: ::core::option::Option<crate::shim::Timestamp>,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.TickInfo")]
pub struct TickInfo {
    #[prost(string, tag = "1")]
    pub liquidity_gross: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub liquidity_net: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub fee_growth_outside:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
    #[prost(message, repeated, tag = "4")]
    pub uptime_trackers: ::prost::alloc::vec::Vec<UptimeTracker>,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.UptimeTracker")]
pub struct UptimeTracker {
    #[prost(message, repeated, tag = "1")]
    pub uptime_growth_outside:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// ===================== MsgCreatePosition
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreatePosition")]
pub struct MsgCreatePosition {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
    #[prost(message, optional, tag = "5")]
    pub token_desired0: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub token_desired1: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "7")]
    pub token_min_amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub token_min_amount1: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub freeze_duration: ::core::option::Option<crate::shim::Duration>,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCreatePositionResponse")]
pub struct MsgCreatePositionResponse {
    #[prost(string, tag = "1")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount1: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub liquidity_created: ::prost::alloc::string::String,
}
/// ===================== MsgWithdrawPosition
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgWithdrawPosition")]
pub struct MsgWithdrawPosition {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
    #[prost(string, tag = "5")]
    pub liquidity_amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub join_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "7")]
    pub freeze_duration: ::core::option::Option<crate::shim::Duration>,
}

// what's freeze_duration?
// Why would anyone do this?
// Can we liquidate this position?

// key: sender + pool id + lower tick + upper tick + join time + freeze duration
// ---> If two messages in the same block, are they combined? or does it throw?
// ---> If combined: bad!

// How do we get join_time when opening a position?
// ---> needs to be added MsgCreatePositionResponse

// Collect fees, uses: sender + pool id + lower tick + upper tick
// ---> it aggr



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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgWithdrawPositionResponse")]
pub struct MsgWithdrawPositionResponse {
    #[prost(string, tag = "1")]
    pub amount0: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount1: ::prost::alloc::string::String,
}
/// ===================== MsgCollectFees
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCollectFees")]
pub struct MsgCollectFees {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub pool_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub lower_tick: i64,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub upper_tick: i64,
}
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
#[proto_message(type_url = "/osmosis.concentratedliquidity.v1beta1.MsgCollectFeesResponse")]
pub struct MsgCollectFeesResponse {
    #[prost(message, repeated, tag = "1")]
    pub collected_fees: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
pub struct ConcentratedliquidityQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ConcentratedliquidityQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn pools(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPoolsResponse, cosmwasm_std::StdError> {
        QueryPoolsRequest { pagination }.query(self.querier)
    }
    pub fn pool(&self, pool_id: u64) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        QueryPoolRequest { pool_id }.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn liquidity_depths_for_range(
        &self,
        pool_id: u64,
        lower_tick: ::prost::alloc::string::String,
        upper_tick: ::prost::alloc::string::String,
    ) -> Result<QueryLiquidityDepthsForRangeResponse, cosmwasm_std::StdError> {
        QueryLiquidityDepthsForRangeRequest {
            pool_id,
            lower_tick,
            upper_tick,
        }
        .query(self.querier)
    }
    pub fn user_positions(
        &self,
        address: ::prost::alloc::string::String,
        pool_id: u64,
    ) -> Result<QueryUserPositionsResponse, cosmwasm_std::StdError> {
        QueryUserPositionsRequest { address, pool_id }.query(self.querier)
    }
    pub fn claimable_fees(
        &self,
        pool_id: u64,
        sender: ::prost::alloc::string::String,
        lower_tick: i64,
        upper_tick: i64,
    ) -> Result<QueryClaimableFeesResponse, cosmwasm_std::StdError> {
        QueryClaimableFeesRequest {
            pool_id,
            sender,
            lower_tick,
            upper_tick,
        }
        .query(self.querier)
    }
}
