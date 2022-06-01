use cosmwasm_std::{
    coin, to_binary, Addr, Binary, Coin, CosmosMsg, CustomMsg, StdError, StdResult,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::{validate_address, validate_string};
use crate::types::{AttributeValueType, MarkerAccess, MarkerType, NameBinding, ProvenanceRoute};
use crate::Scope;

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

impl CustomMsg for ProvenanceMsg {}

/// Input params for custom provenance message encoders.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ProvenanceMsgParams {
    Name(NameMsgParams),
    Attribute(AttributeMsgParams),
    Marker(MarkerMsgParams),
    Metadata(MetadataMsgParams),
    MsgFees(MsgFeesMsgParams),
}

/// Input params for creating name module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NameMsgParams {
    BindName {
        name: String,
        address: Addr,
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

/// Create a message that will bind a name to an address.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdResult};
/// use provwasm_std::{bind_name, NameBinding, ProvenanceMsg};
///
/// // Bind a name to an address.
/// fn exec_bind_name(
///     name: String,
///     address: Addr,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///    let msg = bind_name(&name, address, NameBinding::Restricted)?;
///    let mut res = Response::new().add_message(msg);
///    Ok(res)
/// }
/// ```
pub fn bind_name<S: Into<String>, H: Into<Addr>>(
    name: S,
    address: H,
    binding: NameBinding,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_name_msg(NameMsgParams::BindName {
        name: validate_string(name, "name")?,
        address: validate_address(address)?,
        restrict: matches!(binding, NameBinding::Restricted),
    }))
}

/// Create a message that will un-bind a name from an address.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{unbind_name, ProvenanceMsg};
///
/// // Unbind a name
/// fn exec_unbind_name(name: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = unbind_name(&name)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn unbind_name<S: Into<String>>(name: S) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_name_msg(NameMsgParams::DeleteName {
        name: validate_string(name, "name")?,
    }))
}

/// Input params for creating attribute module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeMsgParams {
    AddAttribute {
        address: Addr,
        name: String,
        value: Binary,
        value_type: AttributeValueType,
    },
    DeleteAttribute {
        address: Addr,
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

/// Create a message that will add an attribute (a typed key-value pair) to an account.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Binary, Env, Addr, Response, StdResult};
/// use provwasm_std::{add_attribute, AttributeValueType, ProvenanceMsg};
///
/// // Add a greeting attribute to an account.
/// // NOTE: The name below must resolve to the contract address.
/// fn exec_add_greeting(
///     env: Env,
///     address: Addr,
///     text: String,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let attr_name = String::from("greeting.my-contract.sc.pb");
///     let greeting = String::from("hello");
///     let msg = add_attribute(
///         address,
///         &attr_name,
///         Binary::from(greeting.as_bytes()),
///         AttributeValueType::String,
///     )?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn add_attribute<H: Into<Addr>, S: Into<String>, B: Into<Binary>>(
    address: H,
    name: S,
    value: B,
    value_type: AttributeValueType,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if value_type == AttributeValueType::Unspecified {
        return Err(StdError::generic_err(
            "cannot add attribute with unspecified value type",
        ));
    }
    Ok(create_attribute_msg(AttributeMsgParams::AddAttribute {
        address: validate_address(address)?,
        name: validate_string(name, "name")?,
        value: value.into(),
        value_type,
    }))
}

/// Create a message that will add a JSON attribute to an account. Serializable types can be passed
/// into this function, but it's up to the user to handle StdResult error case.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Env, Addr, Response, StdResult};
/// use provwasm_std::{add_json_attribute, ProvenanceMsg};
/// use schemars::JsonSchema;
/// use serde::{Deserialize, Serialize};
///
/// // Add a label attribute. NOTE: The name below must resolve to the contract address.
/// fn exec_add_label(
///     env: Env,
///     address: Addr,
///     text: String,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let attr_name = String::from("label.my-contract.sc.pb");
///     let timestamp = env.block.time.nanos();
///     let label = Label { text, timestamp };
///     let msg = add_json_attribute(address, &attr_name, &label)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
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
pub fn add_json_attribute<H: Into<Addr>, S: Into<String>, T: Serialize + ?Sized>(
    address: H,
    name: S,
    data: &T,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    // Serialize the value, bailing on error
    let value = to_binary(data)?;
    // Create and return json typed message
    add_attribute(address, name, value, AttributeValueType::Json)
}

/// Create a message that will remove all attributes with the given name from an account.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdResult};
/// use provwasm_std::{delete_attributes, ProvenanceMsg};
///
/// // Delete all label attributes. NOTE: The name below must resolve to the contract address.
/// fn exec_delete_labels(
///     address: Addr,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let attr_name = String::from("label.my-contract.sc.pb");
///     let msg = delete_attributes(address, &attr_name)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn delete_attributes<H: Into<Addr>, S: Into<String>>(
    address: H,
    name: S,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_attribute_msg(AttributeMsgParams::DeleteAttribute {
        address: validate_address(address)?,
        name: validate_string(name, "name")?,
    }))
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
        address: Addr,
        permissions: Vec<MarkerAccess>,
    },
    RevokeMarkerAccess {
        denom: String,
        address: Addr,
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
    WithdrawCoins {
        marker_denom: String,
        coin: Coin,
        recipient: Addr,
    },
    TransferMarkerCoins {
        coin: Coin,
        to: Addr,
        from: Addr,
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

/// Create a message that will propose a new marker with a given type.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{create_marker, MarkerType, ProvenanceMsg};
///
/// // Create and dispatch a message that will propose a new unrestricted marker.
/// fn try_create_marker(
///     amount: u128,
///     denom: String,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = create_marker(amount, &denom, MarkerType::Coin)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn create_marker<S: Into<String>>(
    amount: u128,
    denom: S,
    marker_type: MarkerType,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    let coin = coin(amount, validate_string(denom, "denom")?);
    Ok(create_marker_msg(MarkerMsgParams::CreateMarker {
        coin,
        marker_type,
    }))
}

/// Create a message that will grant permissions on a marker.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdResult};
/// use provwasm_std::{grant_marker_access, MarkerAccess, ProvenanceMsg};
///
/// // Create and dispatch a message that will grant specific permissions to a marker for an address.
/// fn try_grant_marker_access(
///     denom: String,
///     address: Addr,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let permissions = vec![MarkerAccess::Burn, MarkerAccess::Mint];
///     let msg = grant_marker_access(&denom, address, permissions)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn grant_marker_access<S: Into<String>, H: Into<Addr>>(
    denom: S,
    address: H,
    permissions: Vec<MarkerAccess>,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_marker_msg(MarkerMsgParams::GrantMarkerAccess {
        denom: validate_string(denom, "denom")?,
        address: validate_address(address)?,
        permissions,
    }))
}

/// Create a message that will revoke marker permissions.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdResult};
/// use provwasm_std::{revoke_marker_access, ProvenanceMsg};
///
/// // Create and dispatch a message that will revoke all permissions from a marker for an address.
/// fn try_revoke_marker_access(
///     denom: String,
///     address: Addr,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = revoke_marker_access(&denom, address)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn revoke_marker_access<S: Into<String>, H: Into<Addr>>(
    denom: S,
    address: H,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_marker_msg(MarkerMsgParams::RevokeMarkerAccess {
        denom: validate_string(denom, "denom")?,
        address: validate_address(address)?,
    }))
}

/// Create a message that will finalize a proposed marker.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{finalize_marker, ProvenanceMsg};
///
/// // Create and dispatch a message that will finalize a proposed marker.
/// fn try_finalize_marker(denom: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = finalize_marker(&denom)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn finalize_marker<S: Into<String>>(denom: S) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_marker_msg(MarkerMsgParams::FinalizeMarker {
        denom: validate_string(denom, "denom")?,
    }))
}

/// Create a message that will activate a finalized marker.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{activate_marker, ProvenanceMsg};
///
/// // Create and dispatch a message that will activate a finalized marker.
/// fn try_activate_marker(denom: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = activate_marker(&denom)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn activate_marker<S: Into<String>>(denom: S) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_marker_msg(MarkerMsgParams::ActivateMarker {
        denom: validate_string(denom, "denom")?,
    }))
}

/// Create a message that will cancel a marker.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{cancel_marker, ProvenanceMsg};
///
/// // Create and dispatch a message that will cancel a marker.
/// fn try_cancel_marker(denom: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = cancel_marker(&denom)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn cancel_marker<S: Into<String>>(denom: S) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_marker_msg(MarkerMsgParams::CancelMarker {
        denom: validate_string(denom, "denom")?,
    }))
}

/// Create a message that will destroy a marker.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{destroy_marker, ProvenanceMsg};
///
/// // Create and dispatch a message that will destroy a marker.
/// fn try_destroy_marker(denom: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = destroy_marker(&denom)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn destroy_marker<S: Into<String>>(denom: S) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    Ok(create_marker_msg(MarkerMsgParams::DestroyMarker {
        denom: validate_string(denom, "denom")?,
    }))
}

/// Create a message that will mint marker coins.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{mint_marker_supply, ProvenanceMsg};
///
/// // Create and dispatch a message that will mint marker supply.
/// fn try_mint_marker(amount: u128, denom: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = mint_marker_supply(amount, &denom)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn mint_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("mint amount must be > 0"));
    }
    let coin = coin(amount, validate_string(denom, "denom")?);
    Ok(create_marker_msg(MarkerMsgParams::MintMarkerSupply {
        coin,
    }))
}

/// Create a message that will burn marker coins.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Response, StdResult};
/// use provwasm_std::{burn_marker_supply, ProvenanceMsg};
///
/// // Create and dispatch a message that will burn marker supply.
/// fn try_burn_marker(amount: u128, denom: String) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = burn_marker_supply(amount, &denom)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn burn_marker_supply<S: Into<String>>(
    amount: u128,
    denom: S,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("burn amount must be > 0"));
    }
    let coin = coin(amount, validate_string(denom, "denom")?);
    Ok(create_marker_msg(MarkerMsgParams::BurnMarkerSupply {
        coin,
    }))
}

/// Create a message that will withdraw coins from a marker account to a recipient account.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdResult};
/// use provwasm_std::{withdraw_coins, ProvenanceMsg};
///
/// // Create and dispatch a message that will withdraw coins from a marker.
/// fn try_withdraw_coins(
///     marker_denom: String,
///     amount: u128,
///     denom: String,
///     recipient: Addr,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = withdraw_coins(&marker_denom, amount, &denom, recipient)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn withdraw_coins<S: Into<String>, H: Into<Addr>>(
    marker_denom: S,
    amount: u128,
    denom: S,
    recipient: H,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("withdraw amount must be > 0"));
    }
    let coin = coin(amount, validate_string(denom, "denom")?);
    Ok(create_marker_msg(MarkerMsgParams::WithdrawCoins {
        marker_denom: validate_string(marker_denom, "marker_denom")?,
        coin,
        recipient: validate_address(recipient)?,
    }))
}

/// Create a message that will transfer a marker amount from one account to another.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdResult};
/// use provwasm_std::{transfer_marker_coins, ProvenanceMsg};
///
/// // Create and dispatch a message that will transfer marker coins from one account to another.
/// // NOTE: Transfer is only enabled for restricted markers.
/// fn try_transfer_marker_coins(
///     amount: u128,
///     denom: String,
///     to: Addr,
///     from: Addr,
/// ) -> StdResult<Response<ProvenanceMsg>> {
///     let msg = transfer_marker_coins(amount, &denom, to, from)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn transfer_marker_coins<S: Into<String>, H: Into<Addr>>(
    amount: u128,
    denom: S,
    to: H,
    from: H,
) -> StdResult<CosmosMsg<ProvenanceMsg>> {
    if amount == 0 {
        return Err(StdError::generic_err("transfer amount must be > 0"));
    }
    let coin = coin(amount, validate_string(denom, "denom")?);
    let msg = create_marker_msg(MarkerMsgParams::TransferMarkerCoins {
        coin,
        to: validate_address(to)?,
        from: validate_address(from)?,
    });
    Ok(msg)
}

/// Input params for creating marker module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MetadataMsgParams {
    WriteScope { scope: Scope, signers: Vec<Addr> },
}

// Create a custom cosmos message using metadata module params.
fn create_metadata_msg(params: MetadataMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::Metadata,
        params: ProvenanceMsgParams::Metadata(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will create, or update, a scope.
///
/// ### Example
///
/// ```rust
/// // Imports required
/// use cosmwasm_std::{Addr, Response, StdError};
/// use provwasm_std::{ProvenanceMsg, Scope, write_scope};
///
/// // Create and dispatch a message that will create, or update, a scope.
/// fn try_write_scope(scope: Scope, signers: Vec<Addr>) -> Result<Response<ProvenanceMsg>, StdError> {
///     let msg = write_scope(scope, signers)?;
///     let mut res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn write_scope<S: Into<Scope>, H: Into<Addr>>(
    scope: S,
    signers: Vec<H>,
) -> Result<CosmosMsg<ProvenanceMsg>, StdError> {
    Ok(create_metadata_msg(MetadataMsgParams::WriteScope {
        scope: scope.into(),
        signers: signers
            .into_iter()
            .map(validate_address)
            .collect::<Result<Vec<Addr>, _>>()?,
    }))
}

/// Input params for creating msgfee module messages.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MsgFeesMsgParams {
    AssessCustomFee {
        amount: Coin,
        from: Addr,
        name: Option<String>,
        recipient: Option<Addr>,
    },
}

// Create a custom cosmos message using msgfees module params.
fn create_msgfees_msg(params: MsgFeesMsgParams) -> CosmosMsg<ProvenanceMsg> {
    CosmosMsg::Custom(ProvenanceMsg {
        route: ProvenanceRoute::MsgFees,
        params: ProvenanceMsgParams::MsgFees(params),
        version: String::from(MSG_DATAFMT_VERSION),
    })
}

/// Create a message that will assess a custom fee
/// ### Example
///
/// ```rust
/// use cosmwasm_std::{Addr, Coin, Response, StdError};
/// use provwasm_std::{create_assess_custom_fee_msg, MsgFeesMsgParams, ProvenanceMsg};
///
/// fn try_assess_custom_fee(amount: Coin, name: Option<String>, from: Addr, recipient: Option<Addr>) -> Result<Response<ProvenanceMsg>, StdError>{
///     let msg = create_assess_custom_fee_msg(amount, name, from, recipient)?;
///     let res = Response::new().add_message(msg);
///     Ok(res)
/// }
/// ```
pub fn create_assess_custom_fee_msg<S: Into<String>>(
    amount: Coin,
    name: Option<S>,
    from: Addr,
    recipient: Option<Addr>,
) -> Result<CosmosMsg<ProvenanceMsg>, StdError> {
    Ok(create_msgfees_msg(MsgFeesMsgParams::AssessCustomFee {
        amount,
        name: name.map(|name| name.into()),
        from,
        recipient,
    }))
}
