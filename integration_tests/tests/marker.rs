use cosmwasm_std::{Coin, Uint128};
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

use marker::msg::{ExecuteMsg, InitMsg, QueryMsg};
use marker::types::Marker;

#[test]
fn create_and_withdraw() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app.init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 1)?;
    let admin = &accs[0];

    let wasm = Wasm::new(&app);
    let wasm_byte_code = std::fs::read("../contracts/marker/artifacts/marker.wasm").unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                name: "marker-test.sc.pb".to_string(),
            },
            Some(&admin.address()),
            None,
            &[],
            admin,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Create {
            supply: Uint128::new(100),
            denom: "spy".into(),
            allow_forced_transfer: false,
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::GrantAccess {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Finalize {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Activate {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Withdraw {
            amount: Uint128::new(20),
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    let marker = wasm.query::<QueryMsg, Marker>(
        &contract_addr,
        &QueryMsg::GetByDenom {
            denom: "spy".into(),
        },
    )?;

    assert_eq!(marker.marker_account.denom, "spy");

    Ok(())
}
