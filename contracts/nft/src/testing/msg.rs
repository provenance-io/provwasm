use crate::core::msg::InstantiateMsg;
use cosmwasm_std::Addr;

pub fn mock_instantiate_msg() -> InstantiateMsg {
    InstantiateMsg::Default {
        minter: Addr::unchecked("minter"),
        name: "TestNftContract".to_string(),
        symbol: "HASH".to_string(),
        contract_spec_uuid: "9fe17f9a-56e1-4158-a8af-450680ac9e60".to_string(),
        scope_spec_uuid: "7a65b199-66bc-4d7d-af46-7321b3b017f1".to_string(),
    }
}
