use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

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
        id: String,
        recipient: Addr,
        session_uuid: String,
    },
    // SendNft {
    //     contract: String,
    //     token_id: String,
    //     msg: Binary,
    // },
    // Approve {
    //     spender: String,
    //     token_id: String,
    //     expires: Option<Expiration>,
    // },
    // Revoke {
    //     spender: String,
    //     token_id: String,
    // },
    // ApproveAll {
    //     operator: String,
    //     expires: Option<Expiration>,
    // },
    // RevokeAll {
    //     operator: String,
    // },
    Mint {
        scope_uuid: String,
        session_uuid: String,
        recipient: Addr,
    },
    Burn {
        // this is a Scope UUID
        id: String,
    },
}

// #[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(NftInfoResponse)]
    NftInfo { id: String },
}
#[cw_serde]
pub struct NftInfoResponse {
    pub id: String,
    pub owner: Addr,
}

#[cw_serde]
pub struct MinterResponse {
    pub minter: Option<String>,
}
