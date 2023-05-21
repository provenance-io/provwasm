use cosmwasm_std::{Binary, CosmosMsg, Deps, DepsMut, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

use super::error::ContractError;

pub type ProvDeps<'a> = Deps<'a, ProvenanceQuery>;
pub type ProvDepsMut<'a> = DepsMut<'a, ProvenanceQuery>;
pub type ProvResponse = Response<ProvenanceMsg>;
pub type ProvTxResponse = Result<ProvResponse, ContractError>;
pub type ProvQueryResponse = Result<Binary, ContractError>;
pub type ProvMsg = CosmosMsg<ProvenanceMsg>;
