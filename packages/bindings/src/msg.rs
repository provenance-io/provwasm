#![allow(clippy::field_reassign_with_default)]
use cosmwasm_std::{coin, to_binary, Binary, Coin, CosmosMsg, HumanAddr, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{AttributeValueType, MarkerAccess, MarkerType, ProvenanceRoute};

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

/// Input params for custom provenance message encoders.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProvenanceMsgParams {
    Name(NameMsgParams),
    Attribute(AttributeMsgParams),
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

// Create a custom cosmos message using name module params.
fn create_name_msg(params: NameMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Name,
        params: ProvenanceMsgParams::Name(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will bind a restricted name to an address.
///
/// ### Example
///
/// ```rust
/// # use cosmwasm_std::{HandleResponse, HumanAddr};
/// # use provwasm_std::{bind_name, ProvenanceMsg};
///
/// fn handle_bind_name(
///     name: String,
///     address: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///    let msg = bind_name(&name, &address);
///     HandleResponse {
///         messages: vec![msg], // Dispatches to the provenance name module.
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn bind_name<S: Into<String>, H: Into<HumanAddr>>(
    name: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::BindName {
        name: name.into(),
        address: address.into(),
        restrict: true,
    })
}

/// Create a message that will bind an unrestricted name to an address.
///
/// ### Example
///
/// ```rust
/// # use cosmwasm_std::{HandleResponse, HumanAddr};
/// # use provwasm_std::{bind_name_unrestricted, ProvenanceMsg};
///
/// fn handle_bind_name(
///     name: String,
///     address: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///    let msg = bind_name_unrestricted(&name, &address);
///     HandleResponse {
///         messages: vec![msg], // Dispatches to the provenance name module.
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn bind_name_unrestricted<S: Into<String>, H: Into<HumanAddr>>(
    name: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::BindName {
        name: name.into(),
        address: address.into(),
        restrict: false,
    })
}

/// Create a message that will un-bind a name from an address.
///
/// ### Example
///
/// ```rust
/// # use cosmwasm_std::HandleResponse;
/// # use provwasm_std::{unbind_name, ProvenanceMsg};
///
/// fn handle_unbind_name(name: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = unbind_name(&name);
///     HandleResponse {
///         messages: vec![msg], // Dispatches to the provenance name module.
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn unbind_name<S: Into<String>>(name: S) -> CosmosMsg<ProvenanceMsg> {
    create_name_msg(NameMsgParams::DeleteName { name: name.into() })
}

/// Input params for creating attribute module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeMsgParams {
    AddAttribute {
        address: HumanAddr,
        name: String,
        value: Binary,
        value_type: AttributeValueType,
    },
    DeleteAttribute {
        address: HumanAddr,
        name: String,
    },
}

// Create a custom cosmos message using attribute module params.
fn create_attribute_msg(params: AttributeMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Attribute,
        params: ProvenanceMsgParams::Attribute(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will add a an attribute (a typed key-value pair) to an account.
///
/// ### Example
///
/// ```rust
/// # use cosmwasm_std::{Binary, Env, HandleResponse, HumanAddr};
/// # use provwasm_std::{add_attribute, AttributeValueType, ProvenanceMsg};
///
/// // Add a greeting attribute to an account.
/// // NOTE: The name below must resolve to the contract address.
/// fn handle_add_greeting(
///     env: Env,
///     address: HumanAddr,
///     text: String,
/// ) -> HandleResponse<ProvenanceMsg> {
///     let attr_name = String::from("greeting.my-contract.sc.pb");
///     let greeting = String::from("hello");
///     let msg = add_attribute(
///         &address,
///         &attr_name,
///         Binary::from(greeting.as_bytes()),
///         AttributeValueType::String,
///     );
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn add_attribute<H: Into<HumanAddr>, S: Into<String>, B: Into<Binary>>(
    address: H,
    name: S,
    value: B,
    value_type: AttributeValueType,
) -> CosmosMsg<ProvenanceMsg> {
    create_attribute_msg(AttributeMsgParams::AddAttribute {
        address: address.into(),
        name: name.into(),
        value: value.into(),
        value_type,
    })
}

/// Create a message that will add a JSON attribute to an account. Serializable types can be passed
/// into this function, but it's up to the user to handle StdResult error case.
///
/// ### Example
///
/// ```rust
/// # use cosmwasm_std::{Env, HandleResponse, HumanAddr, StdError};
/// # use provwasm_std::{add_json_attribute, ProvenanceMsg};
/// # use schemars::JsonSchema;
/// # use serde::{Deserialize, Serialize};
///
/// // Add a label attribute. NOTE: The name below must resolve to the contract address.
/// fn handle_add_label(
///     env: Env,
///     address: HumanAddr,
///     text: String,
/// ) -> Result<HandleResponse<ProvenanceMsg>, StdError> {
///     let attr_name = String::from("label.my-contract.sc.pb");
///     let timestamp = env.block.time;
///     let label = Label { text, timestamp };
///     let msg = add_json_attribute(&address, &attr_name, &label)?;
///     Ok(HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     })
/// }
///
/// // Text with a timestamp.
/// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// #[serde(rename_all = "snake_case")]
/// pub struct Label {
///     pub text: String,
///     pub timestamp: u64,
/// }
///
/// ```
pub fn add_json_attribute<H: Into<HumanAddr>, S: Into<String>, T: Serialize + ?Sized>(
    address: H,
    name: S,
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
///
/// ### Example
///
/// ```rust
/// # use cosmwasm_std::{HandleResponse, HumanAddr};
/// # use provwasm_std::{delete_attributes, ProvenanceMsg};
///
/// // Delete all label attributes. NOTE: The name below must resolve to the contract address.
/// fn handle_delete_labels(
///     address: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///     let attr_name = String::from("label.my-contract.sc.pb");
///     let msg = delete_attributes(&address, &attr_name);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn delete_attributes<H: Into<HumanAddr>, S: Into<String>>(
    address: H,
    name: S,
) -> CosmosMsg<ProvenanceMsg> {
    create_attribute_msg(AttributeMsgParams::DeleteAttribute {
        address: address.into(),
        name: name.into(),
    })
}

/// Input params for creating marker module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerMsgParams {
    CreateMarker {
        coin: Coin,
        marker_type: MarkerType,
    },
    GrantMarkerAccess {
        denom: String,
        address: HumanAddr,
        permissions: Vec<MarkerAccess>,
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
    TransferMarkerCoins {
        coin: Coin,
        to: HumanAddr,
        from: HumanAddr,
    },
}

// Create a custom cosmos message using marker module params.
fn create_marker_msg(params: MarkerMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Marker,
        params: ProvenanceMsgParams::Marker(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will propose a new marker with the default type.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will create a new proposed marker.
/// use provwasm_std::{create_marker, ProvenanceMsg};
/// use cosmwasm_std::HandleResponse;
/// fn try_create_marker(supply: u128, denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = create_marker(supply, &denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn create_marker<S: Into<String>>(amount: u128, denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_with_type(amount, denom, MarkerType::Coin)
}

/// Create a message that will propose a new marker with the type set to restricted.
pub fn create_restricted_marker<S: Into<String>>(
    amount: u128,
    denom: S,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_with_type(amount, denom, MarkerType::Restricted)
}

// Create a message that will propose a new marker with a given type.
fn create_marker_with_type<S: Into<String>>(
    amount: u128,
    denom: S,
    marker_type: MarkerType,
) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::CreateMarker { coin, marker_type })
}

/// Create a message that will grant permissions on a marker.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will grant specific permissions to a marker for an address.
/// use cosmwasm_std::{HumanAddr, HandleResponse};
/// use provwasm_std::{ProvenanceMsg, grant_marker_access};
/// fn try_grant_marker_access(
///     denom: String,
///     address: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///     let permissions = vec![MarkerPermission::Burn, MarkerPermission::Mint];
///     let msg = grant_marker_access(&denom, &address, permissions);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn grant_marker_access<S: Into<String>, H: Into<HumanAddr>>(
    denom: S,
    address: H,
    permissions: Vec<MarkerAccess>,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::GrantMarkerAccess {
        denom: denom.into(),
        address: address.into(),
        permissions,
    })
}

/// Create a message that will grant all available permissions (admin, burn, delete, deposit, mint, transfer, withdraw) on a marker to an account.
///
/// ### Example
///
// ```rust
// // Create and dispatch a message that will grant all permissions to a marker for an address.
// fn try_grant_marker_access(
//     denom: String,
//     address: HumanAddr,
// ) -> HandleResponse<ProvenanceMsg> {
//     let msg = grant_marker_access_all(&denom, &address);
//     HandleResponse {
//         messages: vec![msg],
//         attributes: vec![],
//         data: None,
//     }
// }
// ```
pub fn grant_marker_access_all<S: Into<String>, H: Into<HumanAddr>>(
    denom: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    grant_marker_access(
        denom,
        address,
        vec![
            MarkerAccess::Admin,
            MarkerAccess::Burn,
            MarkerAccess::Delete,
            MarkerAccess::Deposit,
            MarkerAccess::Mint,
            MarkerAccess::Transfer,
            MarkerAccess::Withdraw,
        ],
    )
}

/// Create a message that will revoke marker permissions.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will revoke all permissions from a marker for an address.
/// use cosmwasm_std::{HumanAddr, HandleResponse};
/// use provwasm_std::{ProvenanceMsg, revoke_marker_access};
/// fn try_revoke_marker_access(
///     denom: String,
///     address: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///     let msg = revoke_marker_access(&denom, &address);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn revoke_marker_access<S: Into<String>, H: Into<HumanAddr>>(
    denom: S,
    address: H,
) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::RevokeMarkerAccess {
        denom: denom.into(),
        address: address.into(),
    })
}

/// Create a message that will finalize a proposed marker.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will finalize a proposed marker.
/// use cosmwasm_std::HandleResponse;
/// use provwasm_std::{ProvenanceMsg, finalize_marker};
/// fn try_finalize_marker(denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = finalize_marker(&denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn finalize_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::FinalizeMarker {
        denom: denom.into(),
    })
}

/// Create a message that will activate a finalized marker.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will activate a finalized marker.
/// use cosmwasm_std::HandleResponse;
/// use provwasm_std::{ProvenanceMsg, activate_marker};
/// fn try_activate_marker(denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = activate_marker(&denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn activate_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::ActivateMarker {
        denom: denom.into(),
    })
}

/// Create a message that will cancel a marker.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will cancel a marker.
/// use cosmwasm_std::HandleResponse;
/// use provwasm_std::{ProvenanceMsg, cancel_marker};
/// fn try_cancel_marker(denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = cancel_marker(&denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn cancel_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::CancelMarker {
        denom: denom.into(),
    })
}

/// Create a message that will destroy a marker.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will destroy a marker.
/// use cosmwasm_std::HandleResponse;
/// use provwasm_std::{ProvenanceMsg, destroy_marker};
/// fn try_destroy_marker(denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = destroy_marker(&denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn destroy_marker<S: Into<String>>(denom: S) -> CosmosMsg<ProvenanceMsg> {
    create_marker_msg(MarkerMsgParams::DestroyMarker {
        denom: denom.into(),
    })
}

/// Create a message that will mint marker coins.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will mint marker supply.
/// use cosmwasm_std::HandleResponse;
/// use provwasm_std::{ProvenanceMsg, mint_marker_supply};
/// fn try_mint_marker(amount: u128, denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = mint_marker_supply(amount, &denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn mint_marker_supply<S: Into<String>>(amount: u128, denom: S) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::MintMarkerSupply { coin })
}

/// Create a message that will burn marker coins.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will burn marker supply.
/// use cosmwasm_std::HandleResponse;
/// use provwasm_std::{ProvenanceMsg, burn_marker_supply};
/// fn try_burn_marker(amount: u128, denom: String) -> HandleResponse<ProvenanceMsg> {
///     let msg = burn_marker_supply(amount, &denom);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn burn_marker_supply<S: Into<String>>(amount: u128, denom: S) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::BurnMarkerSupply { coin })
}

/// Create a message that will transfer marker coins to a recipient account.
///
/// ### Example
///
/// ```rust
/// // Create and dispatch a message that will withdraw coins from a marker.
/// use cosmwasm_std::{HumanAddr, HandleResponse};
/// use provwasm_std::{ProvenanceMsg, withdraw_marker_coins};
/// fn try_withdraw_marker_coins(
///     amount: u128,
///     denom: String,
///     recipient: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///     let msg = withdraw_marker_coins(amount, &denom, &recipient);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn withdraw_marker_coins<S: Into<String>, H: Into<HumanAddr>>(
    amount: u128,
    denom: S,
    recipient: H,
) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::WithdrawMarkerCoins {
        coin,
        recipient: recipient.into(),
    })
}

/// Create a message that will transfer a marker amount from one account to another.
///
/// ### Example
///
/// ```rust
///
/// // Create and dispatch a message that will transfer marker coins from one account to another.
/// // NOTE: Transfer is only enabled for restricted markers.
/// use cosmwasm_std::{HumanAddr, HandleResponse};
/// use provwasm_std::{ProvenanceMsg, transfer_marker_coins};
/// fn try_transfer_marker_coins(
///     amount: u128,
///     denom: String,
///     to: HumanAddr,
///     from: HumanAddr,
/// ) -> HandleResponse<ProvenanceMsg> {
///     let msg = transfer_marker_coins(amount, &denom, &to, &from);
///     HandleResponse {
///         messages: vec![msg],
///         attributes: vec![],
///         data: None,
///     }
/// }
/// ```
pub fn transfer_marker_coins<S: Into<String>, H: Into<HumanAddr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
) -> CosmosMsg<ProvenanceMsg> {
    let coin = coin(amount, denom);
    create_marker_msg(MarkerMsgParams::TransferMarkerCoins {
        coin,
        to: to.into(),
        from: from.into(),
    })
}
