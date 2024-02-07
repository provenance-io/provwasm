use cosmwasm_std::{Binary, CosmosMsg, Response};

use super::error::ContractError;

pub type ProvResponse = Response;
pub type ProvTxResponse = Result<ProvResponse, ContractError>;
pub type ProvQueryResponse = Result<Binary, ContractError>;
pub type ProvMsg = CosmosMsg;
pub type AssetTag = String;
