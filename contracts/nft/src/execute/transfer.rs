use std::str::FromStr;

use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::metadata::v1::process::ProcessId;
use provwasm_std::types::provenance::metadata::v1::record_input::Source;
use provwasm_std::types::provenance::metadata::v1::{
    MsgWriteRecordRequest, MsgWriteScopeRequest, MsgWriteSessionRequest, Party, PartyType, Process,
    Record, RecordInput, RecordInputStatus, RecordOutput, ResultStatus, Scope, Session,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::util::metadata_address::MetadataAddress;
use crate::{
    core::error::ContractError,
    storage,
    util::action::{Action, ActionType},
};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: Addr,
    scope_uuid: Uuid,
    session_uuid: Uuid,
) -> Result<Response, ContractError> {
    let state = storage::state::get(deps.storage)?;
    let contract_spec_uuid = Uuid::from_str(&state.contract_spec_uuid).unwrap();
    let scope_spec_uuid = Uuid::from_str(&state.scope_spec_uuid).unwrap();

    let scope = Scope {
        scope_id: vec![],
        specification_id: MetadataAddress::scope_specification(scope_spec_uuid)
            .unwrap()
            .bytes,
        owners: vec![Party {
            address: env.contract.address.to_string(),
            role: PartyType::Provenance.into(),
            optional: false,
        }],
        data_access: vec![],
        value_owner_address: recipient.to_string(),
        require_party_rollup: false,
    };

    let write_scope_msg = MsgWriteScopeRequest {
        scope: Some(scope),
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
        scope_uuid: scope_uuid.to_string(),
        spec_uuid: scope_spec_uuid.to_string(),
    };

    let session = Session {
        session_id: MetadataAddress::session(scope_uuid, session_uuid)
            .unwrap()
            .bytes,
        specification_id: MetadataAddress::contract_specification(contract_spec_uuid)
            .unwrap()
            .bytes,
        parties: vec![Party {
            address: env.contract.address.to_string(),
            role: PartyType::Provenance.into(),
            optional: false,
        }],
        name: "nft_session".to_string(),
        context: vec![],
        audit: None,
    };

    let write_session_msg = MsgWriteSessionRequest {
        session: Some(session.clone()),
        signers: vec![env.contract.address.to_string()],
        session_id_components: None,
        spec_uuid: contract_spec_uuid.to_string(),
    };

    let record = Record {
        name: "nft_owner_record_spec".to_string(),
        session_id: session.session_id.clone(),
        process: Some(Process {
            name: "nft_process_name".to_string(),
            method: "set_owner".to_string(),
            process_id: Some(ProcessId::Address(env.contract.address.to_string())),
        }),
        inputs: vec![RecordInput {
            name: "owner".to_string(),
            type_name: "cosmwasm_std::Addr".to_string(),
            status: RecordInputStatus::Proposed.into(),
            source: Some(Source::Hash(format!(
                "{:x}",
                Sha256::digest(info.sender.to_string())
            ))),
        }],
        outputs: vec![RecordOutput {
            hash: MetadataAddress::scope(scope_uuid)?.bech32,
            status: ResultStatus::Pass.into(),
        }],
        specification_id: MetadataAddress::record_specification(
            contract_spec_uuid,
            "nft_owner_record_spec".to_string(),
        )?
        .bytes,
    };

    let write_record_msg = MsgWriteRecordRequest {
        record: Some(record),
        signers: vec![env.contract.address.to_string()],
        session_id_components: None,
        contract_spec_uuid: contract_spec_uuid.to_string(),
        parties: vec![Party {
            address: env.contract.address.to_string(),
            role: PartyType::Provenance.into(),
            optional: false,
        }],
    };

    Ok(Response::default()
        .set_action(ActionType::Execute)
        .add_message(write_scope_msg)
        .add_message(write_session_msg)
        .add_message(write_record_msg))
}
