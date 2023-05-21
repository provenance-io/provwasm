/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const PROVENANCE_VERSION: &str = include_str!("types/PROVENANCE_COMMIT");

mod common;
#[allow(deprecated)]
mod msg;
#[allow(deprecated)]
mod querier;
#[allow(deprecated)]
mod query;
mod serde;
pub mod shim;
#[allow(deprecated)]
pub mod types;

#[allow(deprecated)]
pub use msg::{
    activate_marker, add_attribute, add_json_attribute, assess_custom_fee, bind_name,
    burn_marker_supply, cancel_marker, create_forced_transfer_marker, create_marker,
    delete_attributes, delete_distinct_attribute, destroy_marker, finalize_marker,
    grant_marker_access, mint_marker_supply, revoke_marker_access, transfer_marker_coins,
    unbind_name, update_attribute, withdraw_coins, write_scope, AttributeMsgParams,
    MarkerMsgParams, MetadataMsgParams, MsgFeesMsgParams, NameMsgParams, ProvenanceMsg,
    ProvenanceMsgParams,
};
#[allow(deprecated)]
pub use querier::ProvenanceQuerier;
#[allow(deprecated)]
pub use query::{
    AttributeQueryParams, MarkerQueryParams, MetadataQueryParams, NameQueryParams, ProvenanceQuery,
    ProvenanceQueryParams,
};
#[allow(deprecated)]
pub use types::{
    AccessGrant, Attribute, AttributeValueType, Attributes, Marker, MarkerAccess, MarkerStatus,
    MarkerType, Name, NameBinding, Names, Party, PartyType, Process, ProcessId, ProvenanceRoute,
    Record, RecordInput, RecordInputSource, RecordInputStatus, RecordOutput, Records, ResultStatus,
    Scope, Session, Sessions,
};

// Indicate that smart contracts that use this lib can only be run on the Provenance Blockchain.
#[no_mangle]
extern "C" fn requires_provenance() {}
