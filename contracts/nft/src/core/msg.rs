use cosmwasm_schema::cw_serde;
use cosmwasm_std::Binary;
use cw_ownable::cw_ownable_execute;
use cw_utils::Expiration;

#[cw_serde]
pub enum InstantiateMsg {
    Default {},
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    TransferNft {
        recipient: String,
        token_id: String,
    },
    SendNft {
        contract: String,
        token_id: String,
        msg: Binary,
    },
    Approve {
        spender: String,
        token_id: String,
        expires: Option<Expiration>,
    },
    Revoke {
        spender: String,
        token_id: String,
    },
    ApproveAll {
        operator: String,
        expires: Option<Expiration>,
    },
    RevokeAll {
        operator: String,
    },
    Mint {
        token_id: String,
        owner: String,
        token_uri: Option<String>,
    },
    Burn {
        token_id: String,
    },
}
