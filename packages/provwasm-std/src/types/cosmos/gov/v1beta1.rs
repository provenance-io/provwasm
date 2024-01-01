#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.WeightedVoteOption")]
#[serde(rename_all = "snake_case")]
pub struct WeightedVoteOption {
    #[prost(enumeration = "VoteOption", tag = "1")]
    #[serde(
        serialize_with = "VoteOption::serialize",
        deserialize_with = "VoteOption::deserialize"
    )]
    pub option: i32,
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.Deposit")]
#[serde(rename_all = "snake_case")]
pub struct Deposit {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.Proposal")]
#[serde(rename_all = "snake_case")]
pub struct Proposal {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    #[prost(message, optional, tag = "2")]
    pub content: ::core::option::Option<crate::shim::Any>,
    #[prost(enumeration = "ProposalStatus", tag = "3")]
    #[serde(
        serialize_with = "ProposalStatus::serialize",
        deserialize_with = "ProposalStatus::deserialize"
    )]
    pub status: i32,
    #[prost(message, optional, tag = "4")]
    pub final_tally_result: ::core::option::Option<TallyResult>,
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "6")]
    pub deposit_end_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, repeated, tag = "7")]
    pub total_deposit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "8")]
    pub voting_start_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, optional, tag = "9")]
    pub voting_end_time: ::core::option::Option<crate::shim::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.TallyResult")]
#[serde(rename_all = "snake_case")]
pub struct TallyResult {
    #[prost(string, tag = "1")]
    pub yes: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub abstain: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub no: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub no_with_veto: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.Vote")]
#[serde(rename_all = "snake_case")]
pub struct Vote {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(enumeration = "VoteOption", tag = "3")]
    #[serde(
        serialize_with = "VoteOption::serialize",
        deserialize_with = "VoteOption::deserialize"
    )]
    pub option: i32,
    #[prost(message, repeated, tag = "4")]
    pub options: ::prost::alloc::vec::Vec<WeightedVoteOption>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.DepositParams")]
#[serde(rename_all = "snake_case")]
pub struct DepositParams {
    #[prost(message, repeated, tag = "1")]
    pub min_deposit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "2")]
    pub max_deposit_period: ::core::option::Option<crate::shim::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.VotingParams")]
#[serde(rename_all = "snake_case")]
pub struct VotingParams {
    #[prost(message, optional, tag = "1")]
    pub voting_period: ::core::option::Option<crate::shim::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.TallyParams")]
#[serde(rename_all = "snake_case")]
pub struct TallyParams {
    #[prost(bytes = "vec", tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub quorum: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub threshold: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_base64::serialize",
        deserialize_with = "crate::serde::as_base64::deserialize"
    )]
    pub veto_threshold: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum VoteOption {
    Unspecified = 0,
    Yes = 1,
    Abstain = 2,
    No = 3,
    NoWithVeto = 4,
}
impl VoteOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VoteOption::Unspecified => "VOTE_OPTION_UNSPECIFIED",
            VoteOption::Yes => "VOTE_OPTION_YES",
            VoteOption::Abstain => "VOTE_OPTION_ABSTAIN",
            VoteOption::No => "VOTE_OPTION_NO",
            VoteOption::NoWithVeto => "VOTE_OPTION_NO_WITH_VETO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VOTE_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "VOTE_OPTION_YES" => Some(Self::Yes),
            "VOTE_OPTION_ABSTAIN" => Some(Self::Abstain),
            "VOTE_OPTION_NO" => Some(Self::No),
            "VOTE_OPTION_NO_WITH_VETO" => Some(Self::NoWithVeto),
            _ => None,
        }
    }
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(strum_macros::FromRepr, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Unspecified = 0,
    DepositPeriod = 1,
    VotingPeriod = 2,
    Passed = 3,
    Rejected = 4,
    Failed = 5,
}
impl ProposalStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ProposalStatus::Unspecified => "PROPOSAL_STATUS_UNSPECIFIED",
            ProposalStatus::DepositPeriod => "PROPOSAL_STATUS_DEPOSIT_PERIOD",
            ProposalStatus::VotingPeriod => "PROPOSAL_STATUS_VOTING_PERIOD",
            ProposalStatus::Passed => "PROPOSAL_STATUS_PASSED",
            ProposalStatus::Rejected => "PROPOSAL_STATUS_REJECTED",
            ProposalStatus::Failed => "PROPOSAL_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PROPOSAL_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PROPOSAL_STATUS_DEPOSIT_PERIOD" => Some(Self::DepositPeriod),
            "PROPOSAL_STATUS_VOTING_PERIOD" => Some(Self::VotingPeriod),
            "PROPOSAL_STATUS_PASSED" => Some(Self::Passed),
            "PROPOSAL_STATUS_REJECTED" => Some(Self::Rejected),
            "PROPOSAL_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
    pub fn serialize<S>(v: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let enum_value = Self::from_repr(*v);
        match enum_value {
            Some(v) => serializer.serialize_str(v.as_str_name()),
            None => Err(serde::ser::Error::custom("unknown value")),
        }
    }
    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<i32, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Deserialize;
        let s = String::deserialize(deserializer)?;
        match Self::from_str_name(&s) {
            Some(v) => Ok(v.into()),
            None => Err(serde::de::Error::custom("unknown value")),
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryProposalRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.gov.v1beta1.Query/Proposal",
    response_type = QueryProposalResponse
)]
pub struct QueryProposalRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryProposalResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryProposalResponse {
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryVoteRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.gov.v1beta1.Query/Vote",
    response_type = QueryVoteResponse
)]
pub struct QueryVoteRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryVoteResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryVoteResponse {
    #[prost(message, optional, tag = "1")]
    pub vote: ::core::option::Option<Vote>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryParamsRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.gov.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {
    #[prost(string, tag = "1")]
    pub params_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryParamsResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub voting_params: ::core::option::Option<VotingParams>,
    #[prost(message, optional, tag = "2")]
    pub deposit_params: ::core::option::Option<DepositParams>,
    #[prost(message, optional, tag = "3")]
    pub tally_params: ::core::option::Option<TallyParams>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryDepositRequest")]
#[serde(rename_all = "snake_case")]
#[proto_query(
    path = "/cosmos.gov.v1beta1.Query/Deposit",
    response_type = QueryDepositResponse
)]
pub struct QueryDepositRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.QueryDepositResponse")]
#[serde(rename_all = "snake_case")]
pub struct QueryDepositResponse {
    #[prost(message, optional, tag = "1")]
    pub deposit: ::core::option::Option<Deposit>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.MsgSubmitProposal")]
#[serde(rename_all = "snake_case")]
pub struct MsgSubmitProposal {
    #[prost(message, optional, tag = "1")]
    pub content: ::core::option::Option<crate::shim::Any>,
    #[prost(message, repeated, tag = "2")]
    pub initial_deposit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub proposer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.MsgSubmitProposalResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgSubmitProposalResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.MsgVote")]
#[serde(rename_all = "snake_case")]
pub struct MsgVote {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub proposal_id: u64,
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    #[prost(enumeration = "VoteOption", tag = "3")]
    #[serde(
        serialize_with = "VoteOption::serialize",
        deserialize_with = "VoteOption::deserialize"
    )]
    pub option: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    ::prost::Message,
    schemars::JsonSchema,
    serde::Serialize,
    serde::Deserialize,
    provwasm_proc_macro::CosmwasmExt,
)]
#[proto_message(type_url = "/cosmos.gov.v1beta1.MsgVoteResponse")]
#[serde(rename_all = "snake_case")]
pub struct MsgVoteResponse {}
pub struct GovQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> GovQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn proposal(
        &self,
        proposal_id: u64,
    ) -> std::result::Result<QueryProposalResponse, cosmwasm_std::StdError> {
        QueryProposalRequest { proposal_id }.query(self.querier)
    }
    pub fn vote(
        &self,
        proposal_id: u64,
        voter: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryVoteResponse, cosmwasm_std::StdError> {
        QueryVoteRequest { proposal_id, voter }.query(self.querier)
    }
    pub fn params(
        &self,
        params_type: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest { params_type }.query(self.querier)
    }
    pub fn deposit(
        &self,
        proposal_id: u64,
        depositor: ::prost::alloc::string::String,
    ) -> std::result::Result<QueryDepositResponse, cosmwasm_std::StdError> {
        QueryDepositRequest {
            proposal_id,
            depositor,
        }
        .query(self.querier)
    }
}
