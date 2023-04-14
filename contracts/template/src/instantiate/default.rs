use cosmwasm_std::{Addr, Env, Response};
use cw2::set_contract_version;
use provwasm_std::assess_custom_fee;

use crate::{
    core::{
        aliases::{ProvDepsMut, ProvTxResponse},
        constants::{CONTRACT_NAME, CONTRACT_VERSION},
    },
    storage,
    util::{
        action::{Action, ActionType},
        fee::Fee,
        state::State,
    },
};

pub fn handle(deps: ProvDepsMut, env: Env, owner: Addr, fee: Fee) -> ProvTxResponse {
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
    use cosmwasm_std::{testing::mock_env, Addr, Attribute, Coin, SubMsg};
    use cw2::get_contract_version;
    use provwasm_mocks::mock_dependencies;
    use provwasm_std::assess_custom_fee;

    use crate::{
        core::constants::{CONTRACT_NAME, CONTRACT_VERSION},
        storage,
        testing::{
            constants::{OWNER, TEST_AMOUNT, TEST_DENOM},
            setup::mock_fee,
        },
        util::{action::ActionType, state::State},
    };

    use super::handle;

    #[test]
    fn test_handle() {
        let mut deps = mock_dependencies(&[Coin::new(TEST_AMOUNT, TEST_DENOM)]);
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
