use crate::storage::nft::Approval;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_utils::Expiration;
use provwasm_std::types::provenance::metadata::v1::ScopeResponse;

#[cw_serde]
pub enum InstantiateMsg {
    Default {
        contract_spec_uuid: String,
        scope_spec_uuid: String,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    TransferNft {
        // this is a Scope UUID
        token_id: String,
        recipient: Addr,
        session_uuid: String,
    },
    // SendNft {
    //     contract: String,
    //     token_id: String,
    //     msg: Binary,
    // },
    Approve {
        spender: Addr,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: Addr,
        token_id: String,
    },
    ApproveAll {
        operator: Addr,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: Addr,
    },
    Mint {
        scope_uuid: String,
        session_uuid: String,
        recipient: Addr,
    },
    Burn {
        // this is a Scope UUID
        token_id: String,
    },
}

// #[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Return the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Return operator that can access all of the owner's tokens.
    #[returns(cw721::ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    /// Return approvals that a token has
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Return approval of a given operator for all tokens of an owner, error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        owner: String,
        operator: String,
        include_expired: Option<bool>,
    },
    /// List all operators that can access all of the owner's tokens
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        owner: String,
        /// unset or false will filter out expired items, you must set to true to see them
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    /// With MetaData Extension.
    /// Returns top-level metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},
    /// With MetaData Extension.
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse<ScopeResponse>)]
    NftInfo { token_id: String },
    /// With MetaData Extension.
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse<ScopeResponse>)]
    AllNftInfo {
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// With Enumerable extension.
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    /// With Enumerable extension.
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Return the minter
    #[returns(MinterResponse)]
    Minter {},
}

#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

#[cw_serde]
pub struct OwnerOfResponse {
    /// Owner of the token
    pub owner: String,
    /// If set this address is approved to transfer/send the token as well
    pub approvals: Vec<Approval>,
}

#[cw_serde]
pub struct NftData {
    pub id: String,
    pub owner: Addr,
    pub data: ScopeResponse,
}
