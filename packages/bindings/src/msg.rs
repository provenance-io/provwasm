use cosmwasm_std::{to_binary, Binary, Coin, CosmosMsg, HumanAddr, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{AttributeValueType, MarkerPermission, ProvenanceRoute};

// The data format version to pass into provenance for message encoding
static MSG_DATAFMT_VERSION: &str = "2.0.0";

/// Represents a request to encode custom provenance messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ProvenanceMsg {
    pub route: ProvenanceRoute,      // The module router key
    pub params: ProvenanceMsgParams, // The module-specific encoder params
    pub version: String,             // The data format version
}

/// A helper method to simplify creating messages.
impl Into<CosmosMsg<ProvenanceMsg>> for ProvenanceMsg {
    fn into(self) -> CosmosMsg<ProvenanceMsg> {
        CosmosMsg::Custom(self)
    }
}

/// Input params for custom provenance message encoders.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProvenanceMsgParams {
    Name(NameMsgParams),
    Metadata(MetadataMsgParams),
    Marker(MarkerMsgParams),
}

/// Input params for creating name module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NameMsgParams {
    BindName {
        name: String,
        address: HumanAddr,
        restrict: bool,
    },
    DeleteName {
        name: String,
    },
}

/// A helper method to simplify creating messages.
impl Into<ProvenanceMsgParams> for NameMsgParams {
    fn into(self) -> ProvenanceMsgParams {
        ProvenanceMsgParams::Name(self)
    }
}

// Create a custom cosmos message using name module params.
fn create_name_msg(params: NameMsgParams) -> CosmosMsg<ProvenanceMsg> {
    ProvenanceMsg {
        route: ProvenanceRoute::Name,
        params: params.into(),
        version: String::from(MSG_DATAFMT_VERSION),
    }
    .into()
}

/// Create a message that will bind a restricted name to an address.
pub fn bind_name(name: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::BindName {
        name,
        address,
        restrict: true,
    })
}

/// Create a message that will bind a unrestricted name to an address.
pub fn bind_name_unrestricted(name: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::BindName {
        name,
        address,
        restrict: false,
    })
}

/// Create a message that will un-bind a name from an address.
/// NOTE: This functionality is new in provwasm v0.12.1.
pub fn unbind_name(name: String) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::DeleteName { name })
}

/// Input params for creating metadata module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MetadataMsgParams {
    AddAttribute {
        address: HumanAddr,
        name: String,
        value: Binary,
        value_type: AttributeValueType,
    },
    DeleteAttribute {
        name: String,
        address: HumanAddr,
    },
}

/// A helper method to simplify creating messages.
impl Into<ProvenanceMsgParams> for MetadataMsgParams {
    fn into(self) -> ProvenanceMsgParams {
        ProvenanceMsgParams::Metadata(self)
    }
}

// Create a custom cosmos message using metadata module params.
fn create_metadata_msg(params: MetadataMsgParams) -> CosmosMsg<ProvenanceMsg> {
    ProvenanceMsg {
        route: ProvenanceRoute::Metadata,
        params: params.into(),
        version: String::from(MSG_DATAFMT_VERSION),
    }
    .into()
}

/// Create a message that will add a metadata attribute to an account.
pub fn add_attribute(
    address: HumanAddr,
    name: String,
    value: Binary,
    value_type: AttributeValueType,
) -> CosmosMsg<ProvenanceMsg> {
    create_metadata_msg(MetadataMsgParams::AddAttribute {
        address,
        name,
        value,
        value_type,
    })
}

/// Create a message that will add a UUID attribute to an account.
pub fn add_uuid_attribute(
    address: HumanAddr,
    name: String,
    uuid: String,
) -> CosmosMsg<ProvenanceMsg> {
    add_attribute(
        address,
        name,
        Binary::from(uuid.as_bytes()),
        AttributeValueType::Uuid,
    )
}

/// Create a message that will add a binary attribute to an account.
pub fn add_binary_attribute(
    address: HumanAddr,
    name: String,
    value: Binary,
) -> CosmosMsg<ProvenanceMsg> {
    add_attribute(address, name, value, AttributeValueType::Bytes)
}

/// Create a message that will add a string attribute to an account.
pub fn add_string_attribute(
    address: HumanAddr,
    name: String,
    value: String,
) -> CosmosMsg<ProvenanceMsg> {
    add_attribute(
        address,
        name,
        Binary::from(value.as_bytes()),
        AttributeValueType::String,
    )
}

/// Create a message that will add a JSON attribute to an account. Serializable types can be passed
/// into this function, but it's up to the user to handle StdResult error case.
pub fn add_json_attribute<T: Serialize + ?Sized>(
    address: HumanAddr,
    name: String,
    data: &T,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    // Serialize the value, bailing on error
    let value = to_binary(data)?;
    // Create and return json typed message
    Ok(add_attribute(
        address,
        name,
        value,
        AttributeValueType::Json,
    ))
}

/// Create a message that will remove all attributes with the given name from an account.
pub fn delete_attributes(name: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    create_metadata_msg(MetadataMsgParams::DeleteAttribute { name, address })
}

/// Input params for creating marker module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerMsgParams {
    CreateMarker {
        coin: Coin,
        marker_type: String,
    },
    GrantMarkerAccess {
        denom: String,
        address: HumanAddr,
        permissions: Vec<MarkerPermission>,
    },
    RevokeMarkerAccess {
        denom: String,
        address: HumanAddr,
    },
    FinalizeMarker {
        denom: String,
    },
    ActivateMarker {
        denom: String,
    },
    CancelMarker {
        denom: String,
    },
    DestroyMarker {
        denom: String,
    },
    MintMarkerSupply {
        coin: Coin,
    },
    BurnMarkerSupply {
        coin: Coin,
    },
    WithdrawMarkerCoins {
        coin: Coin,
        recipient: HumanAddr,
    },
}

/// A helper method to simplify creating messages.
impl Into<ProvenanceMsgParams> for MarkerMsgParams {
    fn into(self) -> ProvenanceMsgParams {
        ProvenanceMsgParams::Marker(self)
    }
}

// Create a custom cosmos message using marker module params.
fn create_marker_msg(params: MarkerMsgParams) -> CosmosMsg<ProvenanceMsg> {
    ProvenanceMsg {
        route: ProvenanceRoute::Marker,
        params: params.into(),
        version: String::from(MSG_DATAFMT_VERSION),
    }
    .into()
}

/// Create a message that will propose a new marker with the default type.
pub fn create_marker(coin: Coin) -> CosmosMsg<ProvenanceMsg> {
    create_marker_with_type(coin, String::default()) // Type will be set by encoder
}

/// Create a message that will propose a new marker with a given type.
pub fn create_marker_with_type(coin: Coin, marker_type: String) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::CreateMarker { coin, marker_type })
}

/// Create a message that will grant permissions to a marker.
pub fn grant_marker_access(
    denom: String,
    address: HumanAddr,
    permissions: Vec<MarkerPermission>,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::GrantMarkerAccess {
        denom,
        address,
        permissions,
    })
}

/// Create a message that will grant all available permissions to a marker.
pub fn grant_marker_access_all(denom: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    grant_marker_access(
        denom,
        address,
        vec![
            MarkerPermission::Burn,
            MarkerPermission::Delete,
            MarkerPermission::Deposit,
            MarkerPermission::Grant,
            MarkerPermission::Mint,
            MarkerPermission::Withdraw,
        ],
    )
}

/// Create a message that will grant supply permissions to a marker.
pub fn grant_marker_access_supply(denom: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    grant_marker_access(
        denom,
        address,
        vec![MarkerPermission::Burn, MarkerPermission::Mint],
    )
}

/// Create a message that will grant asset permissions to a marker.
pub fn grant_marker_access_asset(denom: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    grant_marker_access(
        denom,
        address,
        vec![MarkerPermission::Deposit, MarkerPermission::Withdraw],
    )
}

/// Create a message that will revoke marker permissions.
pub fn revoke_marker_access(denom: String, address: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::RevokeMarkerAccess { denom, address })
}

/// Create a message that will finalize a proposed marker.
pub fn finalize_marker(denom: String) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::FinalizeMarker { denom })
}

/// Create a message that will activate a finalized marker.
pub fn activate_marker(denom: String) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::ActivateMarker { denom })
}

/// Create a message that will cancel a marker.
pub fn cancel_marker(denom: String) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::CancelMarker { denom })
}

/// Create a message that will destroy a marker.
pub fn destroy_marker(denom: String) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::DestroyMarker { denom })
}

/// Create a message that will mint marker coins.
pub fn mint_marker_supply(coin: Coin) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::MintMarkerSupply { coin })
}

/// Create a message that will burn marker coins.
pub fn burn_marker_supply(coin: Coin) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::BurnMarkerSupply { coin })
}

/// Create a message that will transfer marker coins to a recipient account.
pub fn withdraw_marker_coins(coin: Coin, recipient: HumanAddr) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::WithdrawMarkerCoins { coin, recipient })
}
