use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary};
use cw2::ContractVersion;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};
use cw_utils::Expiration;
use provwasm_std::types::provenance::metadata::v1::ScopeResponse;

#[cw_serde]
pub enum InstantiateMsg {
    Default {
        /// The only address which can create new NFTs
        minter: Addr,
        /// Name of the NFT contract
        name: String,
        /// Symbol of the NFT contract
        symbol: String,
        /// Generated UUID to use for creating the Contract Specification
        contract_spec_uuid: String,
        /// Generated UUID to use for creating the Scope Specification
        scope_spec_uuid: String,
    },
}

/// CW721 execute methods in addition to `Mint`, `Burn`, and `Change Ownership`
#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    /// Allow operator to transfer / send the token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    Approve {
        /// The operator to be allowed transfer / send of the token
        spender: Addr,
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
        /// An optional expiration of the approval
        expires: Option<Expiration>,
    },

    /// Allows operator to transfer / send any token from the owner's account.
    /// If expiration is set, then this allowance has a time/height limit
    ApproveAll {
        /// The operator to be allowed transfer / send of the token
        operator: Addr,
        /// An optional expiration of the approval
        expires: Option<Expiration>,
    },

    /// Burn an NFT the sender has access to
    Burn {
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
    },

    /// Mint a new NFT, can only be called by the contract minter
    Mint {
        /// Generated UUID to use for creating the NFT (Scope)
        token_id: String,
        /// Generated UUID to use for creating the Session which encapsulates an owner data record
        session_uuid: String,
        /// Recipient of the NFT
        recipient: Addr,
    },

    /// Remove previously granted Approval
    Revoke {
        /// The operator to be removed from previous granted permission to transfer / send the token
        spender: Addr,
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
    },

    /// Remove previously granted ApproveAll permission
    RevokeAll {
        /// The operator to be removed from previous granted permission to transfer / send the token
        operator: Addr,
    },

    /// Transfer a token to a contract and trigger an action on the receiving contract.
    SendNft {
        /// Generated `UUID` to use for creating the `NFT`/`Scope`
        token_id: String,
        /// Generated `UUID` to use for creating the `Session` to write the new owner record
        session_uuid: String,
        /// Receiving Contract Address
        contract: Addr,
        /// Message to send to receiving contract
        msg: Binary,
    },

    /// Transfer a token to another account without triggering actions
    TransferNft {
        /// Generated `UUID` to use for creating the `NFT`/`Scope`
        token_id: String,
        /// Recipient of the NFT
        recipient: Addr,
        /// Generated `UUID` to use for creating the `Session` to write the new owner record
        session_uuid: String,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the operator approval details for a given token
    #[returns(cw721::ApprovalResponse)]
    Approval {
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
        /// The operator allowed transfer / send of the token
        spender: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Returns all operator approvals details for a given token
    #[returns(cw721::ApprovalsResponse)]
    Approvals {
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// MetaData Extension
    /// Returns metadata about the contract
    #[returns(cw721::ContractInfoResponse)]
    ContractInfo {},

    /// MetaData Extension
    /// Returns the contract version information
    #[returns(ContractVersionResponse)]
    ContractVersion {},

    /// Returns the `minter`
    #[returns(MinterResponse)]
    Minter {},

    /// Returns the owner of the given token, error if token does not exist
    #[returns(cw721::OwnerOfResponse)]
    OwnerOf {
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// Returns the operator approval details for all tokens of a given owner or an error if not set
    #[returns(cw721::OperatorResponse)]
    Operator {
        /// Owner of the NFT
        owner: String,
        /// The operator allowed transfer / send of the token
        operator: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },
    /// Returns all operator approvals details for all tokens of a given owner
    #[returns(cw721::OperatorsResponse)]
    AllOperators {
        /// Owner of the NFT
        owner: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
        /// The operator id to index from
        start_after: Option<String>,
        /// The total number of operator details to return
        limit: Option<u32>,
    },
    /// Total number of tokens issued
    #[returns(cw721::NumTokensResponse)]
    NumTokens {},

    /// MetaData Extension
    /// Returns metadata about one particular token, based on *ERC721 Metadata JSON Schema*
    /// but directly from the contract
    #[returns(cw721::NftInfoResponse<ScopeResponse>)]
    NftInfo { token_id: String },
    /// MetaData Extension
    /// Returns the result of both `NftInfo` and `OwnerOf` as one query as an optimization
    /// for clients
    #[returns(cw721::AllNftInfoResponse<ScopeResponse>)]
    AllNftInfo {
        /// Token ID in UUID format (Scope UUID)
        token_id: String,
        /// unset or false will filter out expired approvals, you must set to true to see them
        include_expired: Option<bool>,
    },

    /// Enumerable extension
    /// Returns all tokens owned by the given address, [] if unset.
    #[returns(cw721::TokensResponse)]
    Tokens {
        /// Owner of the NFT
        owner: String,
        /// The token id to index from
        start_after: Option<String>,
        /// The total number of token details to return
        limit: Option<u32>,
    },
    /// Enumerable extension
    /// Requires pagination. Lists all token_ids controlled by the contract.
    #[returns(cw721::TokensResponse)]
    AllTokens {
        /// The token id to index from
        start_after: Option<String>,
        /// The total number of token details to return
        limit: Option<u32>,
    },
}

#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}

#[cw_serde]
pub struct NftData {
    pub id: String,
    pub owner: Addr,
    pub data: ScopeResponse,
}

#[cw_serde]
pub struct ContractInfoResponse {
    pub minter: Addr,
    pub name: String,
    pub symbol: String,
    pub contract_spec_uuid: String,
    pub scope_spec_uuid: String,
}
#[cw_serde]
pub struct ContractVersionResponse {
    pub contract_version: ContractVersion,
}
