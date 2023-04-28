use provwasm_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/provenance.reward.v1.RewardProgram")]
pub struct RewardProgram {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub distribute_from_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub total_reward_pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub remaining_pool_balance:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub claimed_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "8")]
    pub max_reward_by_address:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "9")]
    pub minimum_rollover_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_seconds: u64,
    #[prost(message, optional, tag = "11")]
    pub program_start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "12")]
    pub expected_program_end_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "13")]
    pub program_end_time_max: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "14")]
    pub claim_period_end_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "15")]
    pub actual_program_end_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(uint64, tag = "16")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_periods: u64,
    #[prost(uint64, tag = "17")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_claim_period: u64,
    #[prost(uint64, tag = "18")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_rollover_claim_periods: u64,
    #[prost(enumeration = "reward_program::State", tag = "19")]
    pub state: i32,
    #[prost(uint64, tag = "20")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expiration_offset: u64,
    #[prost(message, repeated, tag = "21")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// Nested message and enum types in `RewardProgram`.
pub mod reward_program {
    use provwasm_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum State {
        Unspecified = 0,
        Pending = 1,
        Started = 2,
        Finished = 3,
        Expired = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "STATE_PENDING",
                State::Started => "STATE_STARTED",
                State::Finished => "STATE_FINISHED",
                State::Expired => "STATE_EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_PENDING" => Some(Self::Pending),
                "STATE_STARTED" => Some(Self::Started),
                "STATE_FINISHED" => Some(Self::Finished),
                "STATE_EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/provenance.reward.v1.ClaimPeriodRewardDistribution")]
pub struct ClaimPeriodRewardDistribution {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    #[prost(message, optional, tag = "3")]
    pub total_rewards_pool_for_claim_period:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub rewards_pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_shares: i64,
    #[prost(bool, tag = "6")]
    pub claim_period_ended: bool,
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
#[proto_message(type_url = "/provenance.reward.v1.RewardAccountState")]
pub struct RewardAccountState {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub action_counter: ::prost::alloc::vec::Vec<ActionCounter>,
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub shares_earned: u64,
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "6")]
    pub claim_status: i32,
}
/// Nested message and enum types in `RewardAccountState`.
pub mod reward_account_state {
    use provwasm_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum ClaimStatus {
        Unspecified = 0,
        Unclaimable = 1,
        Claimable = 2,
        Claimed = 3,
        Expired = 4,
    }
    impl ClaimStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClaimStatus::Unspecified => "CLAIM_STATUS_UNSPECIFIED",
                ClaimStatus::Unclaimable => "CLAIM_STATUS_UNCLAIMABLE",
                ClaimStatus::Claimable => "CLAIM_STATUS_CLAIMABLE",
                ClaimStatus::Claimed => "CLAIM_STATUS_CLAIMED",
                ClaimStatus::Expired => "CLAIM_STATUS_EXPIRED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CLAIM_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "CLAIM_STATUS_UNCLAIMABLE" => Some(Self::Unclaimable),
                "CLAIM_STATUS_CLAIMABLE" => Some(Self::Claimable),
                "CLAIM_STATUS_CLAIMED" => Some(Self::Claimed),
                "CLAIM_STATUS_EXPIRED" => Some(Self::Expired),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/provenance.reward.v1.QualifyingAction")]
pub struct QualifyingAction {
    #[prost(oneof = "qualifying_action::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<qualifying_action::Type>,
}
/// Nested message and enum types in `QualifyingAction`.
pub mod qualifying_action {
    use provwasm_std_derive::CosmwasmExt;
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
    pub enum Type {
        #[prost(message, tag = "1")]
        Delegate(super::ActionDelegate),
        #[prost(message, tag = "2")]
        Transfer(super::ActionTransfer),
        #[prost(message, tag = "3")]
        Vote(super::ActionVote),
    }
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
#[proto_message(type_url = "/provenance.reward.v1.QualifyingActions")]
pub struct QualifyingActions {
    #[prost(message, repeated, tag = "1")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
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
#[proto_message(type_url = "/provenance.reward.v1.ActionDelegate")]
pub struct ActionDelegate {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_actions: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_actions: u64,
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub maximum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub minimum_active_stake_percentile: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub maximum_active_stake_percentile: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.reward.v1.ActionTransfer")]
pub struct ActionTransfer {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_actions: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_actions: u64,
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.reward.v1.ActionVote")]
pub struct ActionVote {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_actions: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_actions: u64,
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_multiplier: u64,
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
#[proto_message(type_url = "/provenance.reward.v1.ActionCounter")]
pub struct ActionCounter {
    #[prost(string, tag = "1")]
    pub action_type: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub number_of_actions: u64,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramByIDRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/RewardProgramByID",
    response_type = QueryRewardProgramByIdResponse
)]
pub struct QueryRewardProgramByIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramByIDResponse")]
pub struct QueryRewardProgramByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub reward_program: ::core::option::Option<RewardProgram>,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramsRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/RewardPrograms",
    response_type = QueryRewardProgramsResponse
)]
pub struct QueryRewardProgramsRequest {
    #[prost(enumeration = "query_reward_programs_request::QueryType", tag = "1")]
    pub query_type: i32,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// Nested message and enum types in `QueryRewardProgramsRequest`.
pub mod query_reward_programs_request {
    use provwasm_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema)]
    pub enum QueryType {
        Unspecified = 0,
        All = 1,
        Pending = 2,
        Active = 3,
        Outstanding = 4,
        Finished = 5,
    }
    impl QueryType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                QueryType::Unspecified => "QUERY_TYPE_UNSPECIFIED",
                QueryType::All => "QUERY_TYPE_ALL",
                QueryType::Pending => "QUERY_TYPE_PENDING",
                QueryType::Active => "QUERY_TYPE_ACTIVE",
                QueryType::Outstanding => "QUERY_TYPE_OUTSTANDING",
                QueryType::Finished => "QUERY_TYPE_FINISHED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "QUERY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "QUERY_TYPE_ALL" => Some(Self::All),
                "QUERY_TYPE_PENDING" => Some(Self::Pending),
                "QUERY_TYPE_ACTIVE" => Some(Self::Active),
                "QUERY_TYPE_OUTSTANDING" => Some(Self::Outstanding),
                "QUERY_TYPE_FINISHED" => Some(Self::Finished),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramsResponse")]
pub struct QueryRewardProgramsResponse {
    #[prost(message, repeated, tag = "1")]
    pub reward_programs: ::prost::alloc::vec::Vec<RewardProgram>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/ClaimPeriodRewardDistributions",
    response_type = QueryClaimPeriodRewardDistributionsResponse
)]
pub struct QueryClaimPeriodRewardDistributionsRequest {
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsResponse")]
pub struct QueryClaimPeriodRewardDistributionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub claim_period_reward_distributions: ::prost::alloc::vec::Vec<ClaimPeriodRewardDistribution>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsByIDRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/ClaimPeriodRewardDistributionsByID",
    response_type = QueryClaimPeriodRewardDistributionsByIdResponse
)]
pub struct QueryClaimPeriodRewardDistributionsByIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsByIDResponse")]
pub struct QueryClaimPeriodRewardDistributionsByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub claim_period_reward_distribution: ::core::option::Option<ClaimPeriodRewardDistribution>,
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
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardDistributionsByAddressRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/RewardDistributionsByAddress",
    response_type = QueryRewardDistributionsByAddressResponse
)]
pub struct QueryRewardDistributionsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "2")]
    pub claim_status: i32,
    #[prost(message, optional, tag = "99")]
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
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardDistributionsByAddressResponse")]
pub struct QueryRewardDistributionsByAddressResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub reward_account_state: ::prost::alloc::vec::Vec<RewardAccountResponse>,
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/provenance.reward.v1.RewardAccountResponse")]
pub struct RewardAccountResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    #[prost(message, optional, tag = "2")]
    pub total_reward_claim:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "3")]
    pub claim_status: i32,
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_id: u64,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgCreateRewardProgramRequest")]
pub struct MsgCreateRewardProgramRequest {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub distribute_from_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub total_reward_pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub max_reward_per_claim_address:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub program_start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_periods: u64,
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_days: u64,
    #[prost(uint64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_rollover_claim_periods: u64,
    #[prost(uint64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expire_days: u64,
    #[prost(message, repeated, tag = "11")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgCreateRewardProgramResponse")]
pub struct MsgCreateRewardProgramResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgEndRewardProgramRequest")]
pub struct MsgEndRewardProgramRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    #[prost(string, tag = "2")]
    pub program_owner_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgEndRewardProgramResponse")]
pub struct MsgEndRewardProgramResponse {}
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
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimRewardsRequest")]
pub struct MsgClaimRewardsRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    #[prost(string, tag = "2")]
    pub reward_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimRewardsResponse")]
pub struct MsgClaimRewardsResponse {
    #[prost(message, optional, tag = "1")]
    pub claim_details: ::core::option::Option<RewardProgramClaimDetail>,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimAllRewardsRequest")]
pub struct MsgClaimAllRewardsRequest {
    #[prost(string, tag = "1")]
    pub reward_address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimAllRewardsResponse")]
pub struct MsgClaimAllRewardsResponse {
    #[prost(message, repeated, tag = "1")]
    pub total_reward_claim:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "2")]
    pub claim_details: ::prost::alloc::vec::Vec<RewardProgramClaimDetail>,
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
#[proto_message(type_url = "/provenance.reward.v1.ClaimedRewardPeriodDetail")]
pub struct ClaimedRewardPeriodDetail {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_shares: u64,
    #[prost(message, optional, tag = "3")]
    pub claim_period_reward:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/provenance.reward.v1.RewardProgramClaimDetail")]
pub struct RewardProgramClaimDetail {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    #[prost(message, optional, tag = "2")]
    pub total_reward_claim:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub claimed_reward_period_details: ::prost::alloc::vec::Vec<ClaimedRewardPeriodDetail>,
}
pub struct RewardQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> RewardQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn reward_program_by_id(
        &self,
        id: u64,
    ) -> Result<QueryRewardProgramByIdResponse, cosmwasm_std::StdError> {
        QueryRewardProgramByIdRequest { id }.query(self.querier)
    }
    pub fn reward_programs(
        &self,
        query_type: i32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryRewardProgramsResponse, cosmwasm_std::StdError> {
        QueryRewardProgramsRequest {
            query_type,
            pagination,
        }
        .query(self.querier)
    }
    pub fn claim_period_reward_distributions(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryClaimPeriodRewardDistributionsResponse, cosmwasm_std::StdError> {
        QueryClaimPeriodRewardDistributionsRequest { pagination }.query(self.querier)
    }
    pub fn claim_period_reward_distributions_by_id(
        &self,
        reward_id: u64,
        claim_period_id: u64,
    ) -> Result<QueryClaimPeriodRewardDistributionsByIdResponse, cosmwasm_std::StdError> {
        QueryClaimPeriodRewardDistributionsByIdRequest {
            reward_id,
            claim_period_id,
        }
        .query(self.querier)
    }
    pub fn reward_distributions_by_address(
        &self,
        address: ::prost::alloc::string::String,
        claim_status: i32,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryRewardDistributionsByAddressResponse, cosmwasm_std::StdError> {
        QueryRewardDistributionsByAddressRequest {
            address,
            claim_status,
            pagination,
        }
        .query(self.querier)
    }
}
