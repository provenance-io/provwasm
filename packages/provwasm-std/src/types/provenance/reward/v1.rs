use provwasm_std_derive::CosmwasmExt;
/// RewardProgram
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.RewardProgram")]
pub struct RewardProgram {
    /// An integer to uniquely identify the reward program.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    /// Name to help identify the Reward Program.(MaxTitleLength=140)
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Short summary describing the Reward Program.(MaxDescriptionLength=10000)
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// address that provides funds for the total reward pool.
    #[prost(string, tag = "4")]
    pub distribute_from_address: ::prost::alloc::string::String,
    /// The total amount of funding given to the RewardProgram.
    #[prost(message, optional, tag = "5")]
    pub total_reward_pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The remaining funds available to distribute after n claim periods have passed.
    #[prost(message, optional, tag = "6")]
    pub remaining_pool_balance:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The total amount of all funds claimed by participants for all past claim periods.
    #[prost(message, optional, tag = "7")]
    pub claimed_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Maximum reward per claim period per address.
    #[prost(message, optional, tag = "8")]
    pub max_reward_by_address:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Minimum amount of coins for a program to rollover.
    #[prost(message, optional, tag = "9")]
    pub minimum_rollover_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Number of seconds that a claim period lasts.
    #[prost(uint64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_seconds: u64,
    /// Time that a RewardProgram should start and switch to STARTED state.
    #[prost(message, optional, tag = "11")]
    pub program_start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Time that a RewardProgram is expected to end, based on data when it was setup.
    #[prost(message, optional, tag = "12")]
    pub expected_program_end_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Time that a RewardProgram MUST end.
    #[prost(message, optional, tag = "13")]
    pub program_end_time_max: ::core::option::Option<crate::shim::Timestamp>,
    /// Used internally to calculate and track the current claim period's ending time.
    #[prost(message, optional, tag = "14")]
    pub claim_period_end_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Time the RewardProgram switched to FINISHED state. Initially set as empty.
    #[prost(message, optional, tag = "15")]
    pub actual_program_end_time: ::core::option::Option<crate::shim::Timestamp>,
    /// Number of claim periods this program will run for.
    #[prost(uint64, tag = "16")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_periods: u64,
    /// Current claim period of the RewardProgram. Uses 1-based indexing.
    #[prost(uint64, tag = "17")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub current_claim_period: u64,
    /// maximum number of claim periods a reward program can rollover.
    #[prost(uint64, tag = "18")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_rollover_claim_periods: u64,
    /// Current state of the RewardProgram.
    #[prost(enumeration = "reward_program::State", tag = "19")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub state: i32,
    /// Grace period after a RewardProgram FINISHED. It is the number of seconds until a RewardProgram enters the EXPIRED
    /// state.
    #[prost(uint64, tag = "20")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expiration_offset: u64,
    /// Actions that count towards the reward.
    #[prost(message, repeated, tag = "21")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// Nested message and enum types in `RewardProgram`.
pub mod reward_program {
    use provwasm_std_derive::CosmwasmExt;
    /// State is the state of the reward program
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// undefined program state
        Unspecified = 0,
        /// pending state of reward program
        Pending = 1,
        /// started state of reward program
        Started = 2,
        /// finished state of reward program
        Finished = 3,
        /// expired state of reward program
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
/// ClaimPeriodRewardDistribution, this is updated at the end of every claim period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.ClaimPeriodRewardDistribution")]
pub struct ClaimPeriodRewardDistribution {
    /// The claim period id.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
    /// The id of the reward program that this reward belongs to.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// The sum of all the granted rewards for this claim period.
    #[prost(message, optional, tag = "3")]
    pub total_rewards_pool_for_claim_period:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The final allocated rewards for this claim period.
    #[prost(message, optional, tag = "4")]
    pub rewards_pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The total number of granted shares for this claim period.
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_shares: i64,
    /// A flag representing if the claim period for this reward has ended.
    #[prost(bool, tag = "6")]
    pub claim_period_ended: bool,
}
/// RewardAccountState contains state at the claim period level for a specific address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.RewardAccountState")]
pub struct RewardAccountState {
    /// The id of the reward program that this share belongs to.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// The id of the claim period that the share belongs to.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
    /// Owner of the reward account state.
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    /// The number of actions performed by this account, mapped by action type.
    #[prost(message, repeated, tag = "4")]
    pub action_counter: ::prost::alloc::vec::Vec<ActionCounter>,
    /// The amount of granted shares for the address in the reward program's claim period.
    #[prost(uint64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub shares_earned: u64,
    /// The status of the claim.
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_status: i32,
}
/// Nested message and enum types in `RewardAccountState`.
pub mod reward_account_state {
    use provwasm_std_derive::CosmwasmExt;
    /// ClaimStatus is the state a claim is in
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClaimStatus {
        /// undefined state
        Unspecified = 0,
        /// unclaimable status
        Unclaimable = 1,
        /// unclaimable claimable
        Claimable = 2,
        /// unclaimable claimed
        Claimed = 3,
        /// unclaimable expired
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
/// QualifyingAction can be one of many action types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QualifyingAction")]
pub struct QualifyingAction {
    /// type of action to process
    #[prost(oneof = "qualifying_action::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<qualifying_action::Type>,
}
/// Nested message and enum types in `QualifyingAction`.
pub mod qualifying_action {
    use provwasm_std_derive::CosmwasmExt;
    /// type of action to process
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Delegate(super::ActionDelegate),
        #[prost(message, tag = "2")]
        Transfer(super::ActionTransfer),
        #[prost(message, tag = "3")]
        Vote(super::ActionVote),
    }
}
/// QualifyingActions contains a list of QualifyingActions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QualifyingActions")]
pub struct QualifyingActions {
    /// The actions that count towards the reward.
    #[prost(message, repeated, tag = "1")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// ActionDelegate represents the delegate action and its required eligibility criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.ActionDelegate")]
pub struct ActionDelegate {
    /// Minimum number of successful delegates.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_actions: u64,
    /// Maximum number of successful delegates.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_actions: u64,
    /// Minimum amount that the user must have currently delegated on the validator.
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Maximum amount that the user must have currently delegated on the validator.
    #[prost(message, optional, tag = "4")]
    pub maximum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Minimum percentile that can be below the validator's power ranking.
    #[prost(string, tag = "5")]
    pub minimum_active_stake_percentile: ::prost::alloc::string::String,
    /// Maximum percentile that can be below the validator's power ranking.
    #[prost(string, tag = "6")]
    pub maximum_active_stake_percentile: ::prost::alloc::string::String,
}
/// ActionTransfer represents the transfer action and its required eligibility criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.ActionTransfer")]
pub struct ActionTransfer {
    /// Minimum number of successful transfers.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_actions: u64,
    /// Maximum number of successful transfers.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_actions: u64,
    /// Minimum delegation amount the account must have across all validators, for the transfer action to be counted.
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// ActionVote represents the voting action and its required eligibility criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.ActionVote")]
pub struct ActionVote {
    /// Minimum number of successful votes.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub minimum_actions: u64,
    /// Maximum number of successful votes.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub maximum_actions: u64,
    /// Minimum delegation amount the account must have across all validators, for the vote action to be counted.
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Positive multiplier that is applied to the shares awarded by the vote action when conditions
    /// are met(for now the only condition is the current vote is a validator vote). A value of zero will behave the same as one
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub validator_multiplier: u64,
}
/// ActionCounter is a key-value pair that maps action type to the number of times it was performed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.ActionCounter")]
pub struct ActionCounter {
    /// The type of action performed.
    #[prost(string, tag = "1")]
    pub action_type: ::prost::alloc::string::String,
    /// The number of times this action has been performed
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub number_of_actions: u64,
}
/// GenesisState defines the reward module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.GenesisState")]
pub struct GenesisState {
    /// Reward program id is the next auto incremented id to be assigned to the next created reward program
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// Reward programs to initially start with.
    #[prost(message, repeated, tag = "2")]
    pub reward_programs: ::prost::alloc::vec::Vec<RewardProgram>,
    /// Claim period reward distributions to initially start with.
    #[prost(message, repeated, tag = "3")]
    pub claim_period_reward_distributions: ::prost::alloc::vec::Vec<ClaimPeriodRewardDistribution>,
    /// Reward account states to initially start with.
    #[prost(message, repeated, tag = "4")]
    pub reward_account_states: ::prost::alloc::vec::Vec<RewardAccountState>,
}
/// QueryRewardProgramByIDRequest queries for the Reward Program with an identifier of id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramByIDRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/RewardProgramByID",
    response_type = QueryRewardProgramByIdResponse
)]
pub struct QueryRewardProgramByIdRequest {
    /// The id of the reward program to query.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// QueryRewardProgramByIDResponse contains the requested RewardProgram
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramByIDResponse")]
pub struct QueryRewardProgramByIdResponse {
    /// The reward program object that was queried for.
    #[prost(message, optional, tag = "1")]
    pub reward_program: ::core::option::Option<RewardProgram>,
}
/// QueryRewardProgramsRequest queries for all reward programs matching the query_type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramsRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/RewardPrograms",
    response_type = QueryRewardProgramsResponse
)]
pub struct QueryRewardProgramsRequest {
    /// A filter on the types of reward programs.
    #[prost(enumeration = "query_reward_programs_request::QueryType", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub query_type: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// Nested message and enum types in `QueryRewardProgramsRequest`.
pub mod query_reward_programs_request {
    use provwasm_std_derive::CosmwasmExt;
    /// QueryType is the state of reward program to query
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryType {
        /// unspecified type
        Unspecified = 0,
        /// all reward programs states
        All = 1,
        /// pending reward program state=
        Pending = 2,
        /// active reward program state
        Active = 3,
        /// pending and active reward program states
        Outstanding = 4,
        /// finished reward program state
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
/// QueryRewardProgramsResponse contains the list of RewardPrograms matching the query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardProgramsResponse")]
pub struct QueryRewardProgramsResponse {
    /// List of RewardProgram objects matching the query_type.
    #[prost(message, repeated, tag = "1")]
    pub reward_programs: ::prost::alloc::vec::Vec<RewardProgram>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClaimPeriodRewardDistributionsRequest queries for all the ClaimPeriodRewardDistributions with pagination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/ClaimPeriodRewardDistributions",
    response_type = QueryClaimPeriodRewardDistributionsResponse
)]
pub struct QueryClaimPeriodRewardDistributionsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryClaimPeriodRewardDistributionsResponse returns the list of paginated ClaimPeriodRewardDistributions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsResponse")]
pub struct QueryClaimPeriodRewardDistributionsResponse {
    /// List of all ClaimPeriodRewardDistribution objects queried for.
    #[prost(message, repeated, tag = "1")]
    pub claim_period_reward_distributions: ::prost::alloc::vec::Vec<ClaimPeriodRewardDistribution>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClaimPeriodRewardDistributionsByIDRequest queries for a single ClaimPeriodRewardDistribution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsByIDRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/ClaimPeriodRewardDistributionsByID",
    response_type = QueryClaimPeriodRewardDistributionsByIdResponse
)]
pub struct QueryClaimPeriodRewardDistributionsByIdRequest {
    /// The reward program that the claim period reward distribution belongs to.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_id: u64,
    /// The claim period that the claim period reward distribution was created for.
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
}
/// QueryClaimPeriodRewardDistributionsByIDResponse returns the requested ClaimPeriodRewardDistribution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryClaimPeriodRewardDistributionsByIDResponse")]
pub struct QueryClaimPeriodRewardDistributionsByIdResponse {
    /// The ClaimPeriodRewardDistribution object that was queried for.
    #[prost(message, optional, tag = "1")]
    pub claim_period_reward_distribution: ::core::option::Option<ClaimPeriodRewardDistribution>,
}
/// QueryRewardDistributionsByAddressRequest queries for reward claims by address that match the claim_status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardDistributionsByAddressRequest")]
#[proto_query(
    path = "/provenance.reward.v1.Query/RewardDistributionsByAddress",
    response_type = QueryRewardDistributionsByAddressResponse
)]
pub struct QueryRewardDistributionsByAddressRequest {
    /// The address that the claim belongs to.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// The status that the reward account must have.
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_status: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryRewardDistributionsByAddressResponse returns the reward claims for an address that match the claim_status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.QueryRewardDistributionsByAddressResponse")]
pub struct QueryRewardDistributionsByAddressResponse {
    /// The address that the reward account belongs to.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// List of RewardAccounts queried for.
    #[prost(message, repeated, tag = "2")]
    pub reward_account_state: ::prost::alloc::vec::Vec<RewardAccountResponse>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// RewardAccountResponse is an address' reward claim for a reward program's claim period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.RewardAccountResponse")]
pub struct RewardAccountResponse {
    /// The id of the reward program that this claim belongs to.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// total rewards claimed for all eligible claim periods in program.
    #[prost(message, optional, tag = "2")]
    pub total_reward_claim:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The status of the claim.
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_status: i32,
    /// The claim period that the claim belongs to.
    #[prost(uint64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_id: u64,
}
/// MsgCreateRewardProgramRequest is the request type for creating a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgCreateRewardProgramRequest")]
pub struct MsgCreateRewardProgramRequest {
    /// title for the reward program.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description for the reward program.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// provider address for the reward program funds and signer of message.
    #[prost(string, tag = "3")]
    pub distribute_from_address: ::prost::alloc::string::String,
    /// total reward pool for the reward program.
    #[prost(message, optional, tag = "4")]
    pub total_reward_pool: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// maximum amount of funds an address can be rewarded per claim period.
    #[prost(message, optional, tag = "5")]
    pub max_reward_per_claim_address:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// start time of the reward program.
    #[prost(message, optional, tag = "6")]
    pub program_start_time: ::core::option::Option<crate::shim::Timestamp>,
    /// number of claim periods the reward program runs for.
    #[prost(uint64, tag = "7")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_periods: u64,
    /// number of days a claim period will exist.
    #[prost(uint64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_days: u64,
    /// maximum number of claim periods a reward program can rollover.
    #[prost(uint64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_rollover_claim_periods: u64,
    /// number of days before a reward program will expire after it has ended.
    #[prost(uint64, tag = "10")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub expire_days: u64,
    /// actions that count towards the reward.
    #[prost(message, repeated, tag = "11")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// MsgCreateRewardProgramResponse is the response type for creating a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgCreateRewardProgramResponse")]
pub struct MsgCreateRewardProgramResponse {
    /// reward program id that is generated on creation.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// MsgEndRewardProgramRequest is the request type for ending a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgEndRewardProgramRequest")]
pub struct MsgEndRewardProgramRequest {
    /// reward program id to end.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// owner of the reward program that funds were distributed from.
    #[prost(string, tag = "2")]
    pub program_owner_address: ::prost::alloc::string::String,
}
/// MsgEndRewardProgramResponse is the response type for ending a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgEndRewardProgramResponse")]
pub struct MsgEndRewardProgramResponse {}
/// MsgClaimRewardsRequest is the request type for claiming reward from reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimRewardsRequest")]
pub struct MsgClaimRewardsRequest {
    /// reward program id to claim rewards.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// reward address and signer of msg to send claimed rewards to.
    #[prost(string, tag = "2")]
    pub reward_address: ::prost::alloc::string::String,
}
/// MsgClaimRewardsResponse is the response type for claiming reward from reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimRewardsResponse")]
pub struct MsgClaimRewardsResponse {
    /// details about acquired rewards from reward program.
    #[prost(message, optional, tag = "1")]
    pub claim_details: ::core::option::Option<RewardProgramClaimDetail>,
}
/// MsgClaimRewardsResponse is the request type for claiming rewards from all reward programs RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimAllRewardsRequest")]
pub struct MsgClaimAllRewardsRequest {
    /// reward address and signer of msg to send claimed rewards to.
    #[prost(string, tag = "1")]
    pub reward_address: ::prost::alloc::string::String,
}
/// MsgClaimRewardsResponse is the response type for claiming rewards from all reward programs RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.MsgClaimAllRewardsResponse")]
pub struct MsgClaimAllRewardsResponse {
    /// total rewards claimed for all eligible claim periods in all programs.
    #[prost(message, repeated, tag = "1")]
    pub total_reward_claim:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// details about acquired rewards from a reward program.
    #[prost(message, repeated, tag = "2")]
    pub claim_details: ::prost::alloc::vec::Vec<RewardProgramClaimDetail>,
}
/// ClaimedRewardPeriodDetail is information regarding an addresses' shares and reward for a claim period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.ClaimedRewardPeriodDetail")]
pub struct ClaimedRewardPeriodDetail {
    /// claim period id
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub claim_period_id: u64,
    /// total shares accumulated for claim period
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub total_shares: u64,
    /// total rewards for claim period
    #[prost(message, optional, tag = "3")]
    pub claim_period_reward:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// RewardProgramClaimDetail is the response object regarding an address's shares and reward for a reward program.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.reward.v1.RewardProgramClaimDetail")]
pub struct RewardProgramClaimDetail {
    /// reward program id.
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub reward_program_id: u64,
    /// total rewards claimed for all eligible claim periods in program.
    #[prost(message, optional, tag = "2")]
    pub total_reward_claim:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// claim period details.
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
