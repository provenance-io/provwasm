use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Timestamp};

use crate::state::AccountData;

/// This needs no info. Owner of the contract is whoever signed the InstantiateMsg.
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Changes the admin
    UpdateAdmin {
        admin: String,
    },
    WhoAmI {
        channel_id: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Returns current admin
    #[returns(AdminResponse)]
    Admin {},
    // Shows all open accounts (incl. remote info)
    #[returns(ListAccountsResponse)]
    ListAccounts {},
    // Get account for one channel
    #[returns(AccountInfo)]
    Account { channel_id: String },
}

#[cw_serde]
pub struct AdminResponse {
    pub admin: String,
}

#[cw_serde]
pub struct ListAccountsResponse {
    pub accounts: Vec<AccountInfo>,
}

#[cw_serde]
pub struct AccountInfo {
    pub channel_id: String,
    /// last block balance was updated (0 is never)
    pub last_update_time: Timestamp,
    /// in normal cases, it should be set, but there is a delay between binding
    /// the channel and making a query and in that time it is empty
    pub remote_addr: Option<String>,
    pub remote_balance: Vec<Coin>,
    pub height: u64,
    pub time: Timestamp,
    pub chain_id: String,
}

impl AccountInfo {
    pub fn convert(channel_id: String, input: AccountData) -> Self {
        AccountInfo {
            channel_id,
            last_update_time: input.last_update_time,
            remote_addr: input.remote_addr,
            remote_balance: input.remote_balance,
            height: input.height,
            time: input.time,
            chain_id: input.chain_id,
        }
    }
}

#[cw_serde]
pub struct AccountResponse {
    /// last block balance was updated (0 is never)
    pub last_update_time: Timestamp,
    /// in normal cases, it should be set, but there is a delay between binding
    /// the channel and making a query and in that time it is empty
    pub remote_addr: Option<String>,
    pub remote_balance: Vec<Coin>,
    pub height: u64,
    pub time: Timestamp,
    pub chain_id: String,
}

impl From<AccountData> for AccountResponse {
    fn from(input: AccountData) -> Self {
        AccountResponse {
            last_update_time: input.last_update_time,
            remote_addr: input.remote_addr,
            remote_balance: input.remote_balance,
            height: input.height,
            time: input.time,
            chain_id: input.chain_id,
        }
    }
}
