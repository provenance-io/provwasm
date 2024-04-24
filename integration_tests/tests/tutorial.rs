use cosmwasm_std::{Coin, Decimal};
use provwasm_test_tube::bank::Bank;
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, Runner, RunnerError};

use provwasm_std::types::cosmos::bank::v1beta1::QueryBalanceRequest;
use provwasm_tutorial::msg::{ExecuteMsg, InitMsg};

#[test]
fn tutorial() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app.init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 2)?;
    let admin = &accs[0];
    let merchant = &accs[1];
    let consumer = &app.init_account(&[
        Coin::new(100_000_000_000_000, "nhash"),
        Coin::new(100_000, "purchasecoin".to_string()),
    ])?;
    let feebucket = &app.init_account(&[Coin::new(100_000_000_000_000, "nhash")])?;

    let wasm = Wasm::new(&app);
    let bank = Bank::new(&app);
    let wasm_byte_code =
        std::fs::read("../contracts/tutorial/artifacts/provwasm_tutorial.wasm").unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                contract_name: "tutorial.sc.pb".to_string(),
                purchase_denom: "purchasecoin".to_string(),
                merchant_address: merchant.address(),
                fee_percent: Decimal::percent(10),
            },
            Some(&feebucket.address()),
            None,
            &[],
            feebucket,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Purchase {
            id: "12345".to_string(),
        },
        &[Coin::new(100, "purchasecoin")],
        consumer,
    )?;
    let merchant_balance_response = bank.query_balance(&QueryBalanceRequest {
        address: merchant.address(),
        denom: "purchasecoin".into(),
    })?;
    let feebucket_balance_response = bank.query_balance(&QueryBalanceRequest {
        address: feebucket.address(),
        denom: "purchasecoin".into(),
    })?;
    assert_eq!(
        merchant_balance_response.balance.unwrap(),
        provwasm_std::types::cosmos::base::v1beta1::Coin {
            denom: "purchasecoin".to_string(),
            amount: "90".to_string()
        }
    );
    assert_eq!(
        feebucket_balance_response.balance.unwrap(),
        provwasm_std::types::cosmos::base::v1beta1::Coin {
            denom: "purchasecoin".to_string(),
            amount: "10".to_string()
        }
    );

    Ok(())
}
