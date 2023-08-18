use std::str::FromStr;

use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::types::provenance::metadata::v1::process::ProcessId;
use provwasm_std::types::provenance::metadata::v1::record_input::Source;
use provwasm_std::types::provenance::metadata::v1::{
    input_specification, DefinitionType, InputSpecification, MsgWriteRecordRequest,
    MsgWriteRecordSpecificationRequest, MsgWriteScopeRequest, MsgWriteSessionRequest, Party,
    PartyType, Process, Record, RecordInput, RecordInputStatus, RecordOutput, RecordSpecification,
    ResultStatus, Scope, Session,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::core::msg::ExecuteMsg;
use crate::util::metadata_address::MetadataAddress;
use crate::{
    core::error::ContractError,
    storage,
    util::action::{Action, ActionType},
};

pub fn handle(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    scope_uuid: Uuid,
    session_uuid: Uuid,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let state = storage::state::get(deps.storage)?;
    let contract_spec_uuid = Uuid::from_str(&state.contract_spec_uuid).unwrap();
    let scope_spec_uuid = Uuid::from_str(&state.scope_spec_uuid).unwrap();

    let scope = Scope {
        scope_id: vec![],
        specification_id: MetadataAddress::for_scope_specification(scope_spec_uuid)
            .unwrap()
            .bytes,
        owners: vec![Party {
            address: env.contract.address.to_string(),
            role: PartyType::Provenance.into(),
            optional: false,
        }],
        data_access: vec![],
        value_owner_address: info.sender.to_string(),
        require_party_rollup: false,
    };

    let write_scope_msg = MsgWriteScopeRequest {
        scope: Some(scope),
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
        scope_uuid: scope_uuid.to_string(),
        spec_uuid: scope_spec_uuid.to_string(),
    };

    let session = Session {
        session_id: MetadataAddress::for_session(scope_uuid, session_uuid)
            .unwrap()
            .bytes,
        specification_id: MetadataAddress::for_contract_specification(contract_spec_uuid)
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

    let record_specification = RecordSpecification {
        specification_id: vec![],
        name: "nft_record_spec_name".to_string(),
        inputs: vec![InputSpecification {
            name: "mint_msg".to_string(),
            type_name: "nft::core::msg::Execute::Mint".to_string(),
            source: Some(input_specification::Source::Hash(
                "hash_of_nft::core::msg::Execute::Mint".to_string(),
            )),
        }],
        type_name: "nft_record_spec_type_name".to_string(),
        result_type: DefinitionType::Record.into(),
        responsible_parties: vec![PartyType::Provenance.into()],
    };

    let write_record_spec_msg = MsgWriteRecordSpecificationRequest {
        specification: Some(record_specification.clone()),
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
        contract_spec_uuid: contract_spec_uuid.to_string(),
    };

    let record = Record {
        name: "nft_record_spec_name".to_string(),
        session_id: session.session_id.clone(),
        process: Some(Process {
            name: "nft_process_name".to_string(),
            method: "mint".to_string(),
            process_id: Some(ProcessId::Address(env.contract.address.to_string())),
        }),
        inputs: vec![RecordInput {
            name: "mint_msg".to_string(),
            type_name: "nft::core::msg::Execute::Mint".to_string(),
            status: RecordInputStatus::Proposed.into(),
            source: Some(Source::Hash(format!(
                "{:x}",
                Sha256::digest(serde_json::to_string(&msg).unwrap())
            ))),
        }],
        outputs: vec![RecordOutput {
            hash: "nft_record_output_hash".to_string(),
            status: ResultStatus::Pass.into(),
        }],
        specification_id: vec![],
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

    let record2 = Record {
        name: "nft_record_spec_name".to_string(),
        session_id: session.session_id,
        process: Some(Process {
            name: "nft_process_name".to_string(),
            method: "mint".to_string(),
            process_id: Some(ProcessId::Address(env.contract.address.to_string())),
        }),
        inputs: vec![RecordInput {
            name: "mint_msg".to_string(),
            type_name: "nft::core::msg::Execute::Mint".to_string(),
            status: RecordInputStatus::Proposed.into(),
            source: Some(Source::Hash(format!(
                "{:x}",
                Sha256::digest(serde_json::to_string(&msg).unwrap())
            ))),
        }],
        outputs: vec![RecordOutput {
            hash: "nft_record_output_hash".to_string(),
            status: ResultStatus::Pass.into(),
        }],
        specification_id: vec![],
    };

    let write_record_msg2 = MsgWriteRecordRequest {
        record: Some(record2),
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
        .add_message(write_record_spec_msg)
        .add_message(write_record_msg)
        .add_message(write_record_msg2))
}

#[cfg(test)]
pub mod test {
    use crate::core::msg::ExecuteMsg;
    use sha2::{Digest, Sha256};

    #[test]
    pub fn sha_test() {
        let msg = ExecuteMsg::Mint {
            scope_uuid: "uuid1".to_string(),
            session_uuid: "uuid2".to_string(),
        };
        let x = format!("{:x}", Sha256::digest(serde_json::to_string(&msg).unwrap()));

        println!("sha: {}", x);
    }
}
