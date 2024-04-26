use cosmwasm_std::{Coin, Uint64};
use provwasm_test_tube::trigger::Trigger as Trigger_Runner;
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

use provwasm_std::types::provenance::trigger::v1::QueryTriggersRequest;
use trigger::msg::Event::{BlockHeightEvent, BlockTimeEvent};
use trigger::msg::{ExecuteMsg, InitMsg};

#[test]
fn create_triggers() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app.init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 3)?;
    let admin = &accs[0];
    let sender = &accs[1];
    let receiver = &accs[2];

    let wasm = Wasm::new(&app);
    let trigger_runner = Trigger_Runner::new(&app);
    let wasm_byte_code = std::fs::read("../contracts/trigger/artifacts/trigger.wasm").unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {},
            Some(&admin.address()),
            None,
            &[],
            admin,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::CreateTrigger {
            event: BlockHeightEvent {
                block_height: Uint64::new((app.get_block_height() + 30) as u64),
            },
            to_address: receiver.address(),
        },
        &[Coin::new(10_000, "nhash")],
        sender,
    )?;
    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::CreateTrigger {
            event: BlockTimeEvent {
                timestamp: Uint64::new((app.get_block_time_seconds() + 30) as u64),
            },
            to_address: receiver.address(),
        },
        &[Coin::new(10_000, "nhash")],
        admin,
    )?;
    let triggers_response =
        trigger_runner.query_triggers(&QueryTriggersRequest { pagination: None })?;
    assert_eq!(triggers_response.triggers.len(), 2);

    Ok(())
}
