use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;
use provwasm_std::types::provenance::metadata::v1::contract_specification::Source;
use provwasm_std::types::provenance::metadata::v1::{
    ContractSpecification, Description, MsgWriteContractSpecificationRequest,
    MsgWriteScopeSpecificationRequest, PartyType, ScopeSpecification,
};
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::core::msg::ContractInfoResponse;
use crate::storage::contract_info::CONTRACT_INFO;
use crate::storage::state::State;
use crate::util::metadata_address::MetadataAddress;
use crate::{
    core::constants::{CONTRACT_NAME, CONTRACT_VERSION},
    storage,
    util::action::{Action, ActionType},
};

pub fn handle(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    minter: Addr,
    name: String,
    symbol: String,
    contract_spec_uuid: Uuid,
    scope_spec_uuid: Uuid,
) -> Result<Response, ContractError> {
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(minter.as_str()))?;
    CONTRACT_INFO.save(
        deps.storage,
        &ContractInfoResponse {
            minter,
            name,
            symbol,
            contract_spec_uuid: contract_spec_uuid.to_string(),
            scope_spec_uuid: scope_spec_uuid.to_string(),
        },
    )?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let contract_spec = ContractSpecification {
        specification_id: vec![], // ignore since provenance generates using uuid
        description: Some(Description {
            name: "NFT Manager Contract".to_string(),
            description: "A contract specification that manages an NFT".to_string(),
            website_url: "https://github.com/provenance-io/provwasm/contracts/nft".to_string(),
            icon_url: "https://github.com/provenance-io/provwasm/contracts/nft/public/icon.png"
                .to_string(),
        }),
        owner_addresses: vec![env.contract.address.to_string()],
        parties_involved: vec![PartyType::Provenance.into()],
        class_name: CONTRACT_NAME.to_string(),
        source: Some(Source::Hash("wasmbinaryhash".to_string())),
    };

    let write_contract_spec_msg = MsgWriteContractSpecificationRequest {
        specification: Some(contract_spec),
        signers: vec![env.contract.address.to_string()],
        spec_uuid: contract_spec_uuid.to_string(),
    };

    let scope_spec = ScopeSpecification {
        specification_id: vec![],
        description: Some(Description {
            name: "NFT Specification".to_string(),
            description: "A specification of an NFT".to_string(),
            website_url: "https://provenance.io/nft".to_string(),
            icon_url: "https://provenance.io/nft/public/icon.png".to_string(),
        }),
        owner_addresses: vec![env.contract.address.to_string()],
        parties_involved: vec![PartyType::Provenance.into()],
        contract_spec_ids: vec![
            MetadataAddress::contract_specification(contract_spec_uuid)
                .unwrap()
                .bytes,
        ],
    };

    let write_scope_spec_msg = MsgWriteScopeSpecificationRequest {
        specification: Some(scope_spec),
        signers: vec![env.contract.address.to_string()],
        spec_uuid: scope_spec_uuid.to_string(),
    };

    let state = State::new(contract_spec_uuid.to_string(), scope_spec_uuid.to_string());
    storage::state::set(deps.storage, &state)?;

    Ok(Response::default()
        .set_action(ActionType::Initialize {})
        .add_message(write_contract_spec_msg)
        .add_message(write_scope_spec_msg))
}
