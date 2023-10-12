use cosmwasm_std::{Addr, Binary, DepsMut, Env, MessageInfo, Response};
use cw721::Cw721ReceiveMsg;
use provwasm_std::types::provenance::metadata::v1::process::ProcessId;
use provwasm_std::types::provenance::metadata::v1::record_input::Source;
use provwasm_std::types::provenance::metadata::v1::{
    MsgUpdateValueOwnersRequest, MsgWriteRecordRequest, MsgWriteSessionRequest, Party, PartyType,
    Process, Record, RecordInput, RecordInputStatus, RecordOutput, ResultStatus, Session,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::core::error::ContractError;
use crate::events::send::EventSend;
use crate::storage;
use crate::storage::nft::TOKENS;
use crate::util::action::{Action, ActionType};
use crate::util::metadata_address::MetadataAddress;
use crate::util::{parse_uuid, permission};

pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_id: Uuid,
    session_uuid: Uuid,
    contract: Addr,
    msg: Binary,
) -> Result<Response, ContractError> {
    // check if sender can transfer token
    let mut nft = TOKENS.load(deps.storage, &token_id.to_string())?;
    permission::can_send(deps.as_ref(), &env, &info, &nft)?;

    let state = storage::state::get(deps.storage)?;
    let contract_spec_uuid = parse_uuid(&state.contract_spec_uuid)?;

    let update_scope_value_owner_msg = MsgUpdateValueOwnersRequest {
        scope_ids: vec![MetadataAddress::scope(token_id).unwrap().bytes],
        value_owner_address: contract.to_string(),
        signers: vec![env.contract.address.to_string(), info.sender.to_string()],
    };

    let session = Session {
        session_id: MetadataAddress::session(token_id, session_uuid)
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
            hash: MetadataAddress::scope(token_id)?.bech32,
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

    let contract_msg = Cw721ReceiveMsg {
        sender: info.sender.to_string(),
        token_id: token_id.to_string(),
        msg,
    }
    .into_cosmos_msg(contract.to_string())?;

    // set new owner and clear existing approvals
    nft.owner = contract.clone();
    nft.approvals = vec![];
    TOKENS.save(deps.storage, &token_id.to_string(), &nft)?;

    Ok(Response::default()
        .set_action(ActionType::Send)
        .add_event(EventSend { contract, token_id }.into())
        .add_message(update_scope_value_owner_msg)
        .add_message(write_session_msg)
        .add_message(write_record_msg)
        .add_message(contract_msg))
}
