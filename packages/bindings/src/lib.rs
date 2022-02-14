#[allow(clippy::all)]
mod common;
mod msg;
mod querier;
mod query;
mod types;

pub use msg::{
    activate_marker, add_attribute, add_json_attribute, bind_name, burn_marker_supply,
    cancel_marker, create_marker, delete_attributes, destroy_marker, finalize_marker,
    grant_marker_access, mint_marker_supply, revoke_marker_access, transfer_marker_coins,
    unbind_name, withdraw_coins, write_scope, AttributeMsgParams, MarkerMsgParams, NameMsgParams,
    ProvenanceMsg, ProvenanceMsgParams,
};
pub use querier::ProvenanceQuerier;
pub use query::{
    AttributeQueryParams, MarkerQueryParams, MetadataQueryParams, NameQueryParams, ProvenanceQuery,
    ProvenanceQueryParams,
};
pub use types::{
    AccessGrant, Attribute, AttributeValueType, Attributes, Marker, MarkerAccess, MarkerStatus,
    MarkerType, Name, NameBinding, Names, Party, PartyType, Process, ProcessId, ProvenanceRoute,
    Record, RecordInput, RecordInputSource, RecordInputStatus, RecordOutput, Records, ResultStatus,
    Scope, Session, Sessions,
};

// Indicate that smart contracts that use this lib can only be run on the Provenance Blockchain.
#[no_mangle]
extern "C" fn requires_provenance() {}
