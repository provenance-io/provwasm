use cosmwasm_std::to_binary;

use crate::{
    core::{
        aliases::{ProvDeps, ProvQueryResponse},
        msg::QueryOwnerResponse,
    },
    storage,
};

pub fn handle(deps: ProvDeps) -> ProvQueryResponse {
    let res = QueryOwnerResponse {
        owner: storage::state::get_owner(deps.storage)?,
    };
    Ok(to_binary(&res)?)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, Addr, Coin};
    use provwasm_mocks::mock_dependencies;

    use crate::{
        core::msg::QueryOwnerResponse,
        testing::{
            constants::{OWNER, TEST_DENOM},
            setup,
        },
    };

    use super::handle;

    #[test]
    fn test_query_owner_has_correct_response() {
        let mut deps = mock_dependencies(&[Coin::new(1000000000, TEST_DENOM)]);
        setup::mock_contract(deps.as_mut());
        let bin = handle(deps.as_ref()).unwrap();
        let response: QueryOwnerResponse = from_binary(&bin).unwrap();
        assert_eq!(Addr::unchecked(OWNER), response.owner);
    }
}
