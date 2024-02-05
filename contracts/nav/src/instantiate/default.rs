use cosmwasm_std::{Addr, DepsMut, Env, Response};
use cw2::set_contract_version;

use crate::{
    core::{
        aliases::ProvTxResponse,
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
    },
    storage,
    util::{
        action::{Action, ActionType},
        fee::{assess_custom_fee, Fee},
        state::State,
    },
};

/// Performs the instantiation logic for the Default variant of InstantiateMsg.
///
/// The contract first stores the owner and fee into the state. It then sets the contract version,
/// and returns a response containing the message fee submessage.
///
/// # Arguments
///
/// * `deps` - A mutable version of the dependencies. The API, Querier, and storage can all be accessed from it.
/// * `env` - Information about the Blockchain's environment such as block height.
/// * `owner` - The address of the contract's owner.
/// * `fee` - The amount of additional funds to charge.
///
/// # Examples
/// ```
/// let msg = InstantiateMsg::Default {owner: Addr::unchecked("owner"), fee: Fee {recipient: Some(Addr::unchecked("owner")), amount: Coin::new(0, "nhash")}};
/// let res = handle(deps, env, msg.owner, msg.fee)?;
/// ```
pub fn handle(deps: DepsMut, env: Env, owner: Addr, fee: Fee) -> ProvTxResponse {
    storage::state::set(deps.storage, &State::new(owner, fee.clone()))?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let fee_message = assess_custom_fee(
        fee.amount.clone(),
        Some("contract_fee"),
        env.contract.address,
        fee.recipient,
    )?;
    Ok(Response::default()
        .set_action(ActionType::Initialize {})
        .add_message(fee_message))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, SubMsg};
    use cw2::get_contract_version;
    use provwasm_mocks::mock_provenance_dependencies;

    use crate::{
        core::constants::{CONTRACT_NAME, CONTRACT_VERSION},
        storage,
        testing::{constants::OWNER, setup::mock_fee},
        util::{action::ActionType, fee::assess_custom_fee, state::State},
    };

    use super::handle;

    #[test]
    fn test_handle() {
        let mut deps = mock_provenance_dependencies();
        let env = mock_env();
        let owner = Addr::unchecked(OWNER);
        let fee = mock_fee();

        let expected_state = State::new(owner.clone(), fee.clone());

        let res = handle(deps.as_mut(), env.clone(), owner, fee.clone()).unwrap();
        let state = storage::state::get(&deps.storage).unwrap();
        let contract_version = get_contract_version(&deps.storage).unwrap();
        let expected_fee = assess_custom_fee(
            fee.amount.clone(),
            Some("contract_fee"),
            env.contract.address,
            fee.recipient,
        )
        .unwrap();

        assert_eq!(expected_state, state);
        assert_eq!(CONTRACT_NAME, contract_version.contract);
        assert_eq!(CONTRACT_VERSION, contract_version.version);
        assert_eq!(
            vec![Attribute::from(ActionType::Initialize {})],
            res.attributes
        );
        assert_eq!(vec![SubMsg::new(expected_fee)], res.messages);
    }
}
