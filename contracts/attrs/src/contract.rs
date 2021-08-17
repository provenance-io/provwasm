use cosmwasm_std::{to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError};
use provwasm_std::{
    add_json_attribute, bind_name, delete_attributes, NameBinding, ProvenanceMsg, ProvenanceQuerier,
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InitMsg, Label, LabelNameResponse, LabelsResponse, MigrateMsg, QueryMsg,
};
use crate::state::{config, config_read, State};

/// Initialize the smart contract config state and bind a name to the contract address.
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Create and save contract config state.
    config(deps.storage).save(&State {
        contract_owner: info.sender.clone(),
        contract_name: msg.name.clone(),
    })?;

    // Create bind name message.
    let bind_name_msg = bind_name(&msg.name, env.contract.address, NameBinding::Restricted)?;

    // Dispatch message to handler and add event attributes.
    let res = Response::new()
        .add_message(bind_name_msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.attrs.init")
        .add_attribute("contract_name", msg.name)
        .add_attribute("contract_owner", info.sender);
    Ok(res)
}

/// Handle state change requests
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    // Load contract state
    let state = config_read(deps.storage).load()?;

    // Validate the message sender is the contact owner.
    if info.sender != state.contract_owner {
        return Err(ContractError::Unauthorized {});
    }

    // Create label name using stored contract name.
    let attr_name = format!("{}.{}", "label", state.contract_name);

    // Dispatch message to the appropriate handler
    match msg {
        ExecuteMsg::BindLabelName {} => try_bind_label_name(env, attr_name),
        ExecuteMsg::AddLabel { text } => try_add_label(env, attr_name, text),
        ExecuteMsg::DeleteLabels {} => try_delete_labels(env, attr_name),
    }
}

// Bind the label attibute name to the contract address.
fn try_bind_label_name(
    env: Env,
    attr_name: String,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let bind_name_msg = bind_name(&attr_name, env.contract.address, NameBinding::Restricted)?;
    let res = Response::new()
        .add_message(bind_name_msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.attrs.try_bind_label_name")
        .add_attribute("attribute_name", attr_name);
    Ok(res)
}

// Add a label attribute.
fn try_add_label(
    env: Env,
    attr_name: String,
    text: String,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let timestamp = env.block.time.nanos();
    let label = Label { text, timestamp };
    let msg = add_json_attribute(env.contract.address, &attr_name, &label)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.attrs.try_add_label")
        .add_attribute("attribute_name", attr_name);
    Ok(res)
}

// Delete all label attributes.
fn try_delete_labels(
    env: Env,
    attr_name: String,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let msg = delete_attributes(env.contract.address, &attr_name)?;
    let res = Response::new()
        .add_message(msg)
        .add_attribute("integration_test", "v2")
        .add_attribute("action", "provwasm.contracts.attrs.try_delete_labels")
        .add_attribute("attribute_name", attr_name);
    Ok(res)
}

/// Handle label query requests.
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<QueryResponse, StdError> {
    let state = config_read(deps.storage).load()?;
    let attr_name = format!("{}.{}", "label", state.contract_name);
    match msg {
        QueryMsg::GetLabelName {} => to_binary(&LabelNameResponse { name: attr_name }),
        QueryMsg::GetLabels {} => {
            let querier = ProvenanceQuerier::new(&deps.querier);
            let labels: Vec<Label> =
                querier.get_json_attributes(env.contract.address, &attr_name)?;
            to_binary(&LabelsResponse { labels })
        }
    }
}

/// Called when migrating a contract instance to a new code ID.
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{from_binary, CosmosMsg};
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::{
        AttributeMsgParams, AttributeValueType, NameMsgParams, ProvenanceMsgParams,
    };

    #[test]
    fn init_test() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Call init
        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Ensure a message was created to bind the name to the contract address.
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Name(p) => match &p {
                    NameMsgParams::BindName { name, .. } => assert_eq!(name, "contract.pb"),
                    _ => panic!("unexpected name params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn handle_bind_label_name() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Handle bind label name request
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            ExecuteMsg::BindLabelName {},
        )
        .unwrap(); // Panics on error

        // Ensure a message was generated for binding the label name under the contract name.
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Name(p) => match &p {
                    NameMsgParams::BindName { name, .. } => assert_eq!(name, "label.contract.pb"),
                    _ => panic!("unexpected name params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn handle_bind_label_name_unauthorized() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Handle bind label name request
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("other", &[]), //  error: not 'sender'
            ExecuteMsg::BindLabelName {},
        )
        .unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn handle_add_label() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Handle a add label request
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            ExecuteMsg::AddLabel {
                text: "text".into(),
            },
        )
        .unwrap();

        // Ensure a message was created to add a named JSON attribute.
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Attribute(p) => match &p {
                    AttributeMsgParams::AddAttribute {
                        name, value_type, ..
                    } => {
                        assert_eq!(name, "label.contract.pb");
                        assert_eq!(value_type, &AttributeValueType::Json);
                    }
                    _ => panic!("unexpected attribute params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn handle_add_label_unauthorized() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Try to add a label with some other sender address
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("other", &[]), // error: not 'sender'
            ExecuteMsg::AddLabel {
                text: "text".into(),
            },
        )
        .unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn handle_delete_labels() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Handle a delete label request
        let res = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            ExecuteMsg::DeleteLabels {},
        )
        .unwrap();

        // Ensure a message was created to delete all named JSON attributes.
        assert_eq!(1, res.messages.len());
        match &res.messages[0].msg {
            CosmosMsg::Custom(msg) => match &msg.params {
                ProvenanceMsgParams::Attribute(p) => match &p {
                    AttributeMsgParams::DeleteAttribute { name, .. } => {
                        assert_eq!(name, "label.contract.pb");
                    }
                    _ => panic!("unexpected attribute params"),
                },
                _ => panic!("unexpected provenance params"),
            },
            _ => panic!("unexpected cosmos message"),
        }
    }

    #[test]
    fn handle_delete_labels_unauthorized() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Try to delete labels with some other sender address
        let err = execute(
            deps.as_mut(),
            mock_env(),
            mock_info("other", &[]), // error: not 'sender'
            ExecuteMsg::DeleteLabels {},
        )
        .unwrap_err();

        // Assert an unauthorized error was returned
        match err {
            ContractError::Unauthorized {} => {}
            e => panic!("unexpected error: {:?}", e),
        }
    }

    #[test]
    fn query_get_label_name() {
        // Create default provenance mocks.
        let mut deps = mock_dependencies(&[]);

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Call the smart contract query function.
        let bin = query(deps.as_ref(), mock_env(), QueryMsg::GetLabelName {}).unwrap();

        // Ensure that we got the expected response
        let rep: LabelNameResponse = from_binary(&bin).unwrap();
        assert_eq!(
            rep,
            LabelNameResponse {
                name: "label.contract.pb".into()
            }
        );
    }

    #[test]
    fn query_get_labels() {
        // Create custom provenance mocks.
        let mut deps = mock_dependencies(&[]);
        deps.querier.with_attributes(
            MOCK_CONTRACT_ADDR,
            &[(
                "label.contract.pb",
                "{\"text\":\"text\",\"timestamp\":123456789}",
                "json",
            )],
        );

        // Init so we have state
        let _ = instantiate(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            InitMsg {
                name: "contract.pb".into(),
            },
        )
        .unwrap(); // Panics on error

        // Call the smart contract query function.
        let bin = query(deps.as_ref(), mock_env(), QueryMsg::GetLabels {}).unwrap();

        // Ensure that we got the expected response
        let rep: LabelsResponse = from_binary(&bin).unwrap();
        assert_eq!(rep.labels.len(), 1);
        assert_eq!(
            rep.labels[0],
            Label {
                text: "text".into(),
                timestamp: 123456789
            }
        )
    }
}
