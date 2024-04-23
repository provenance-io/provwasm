use cosmwasm_std::Coin;
use name::msg::{ExecuteMsg, InitMsg, QueryMsg};
use provwasm_std::types::provenance::name::v1::{QueryResolveResponse, QueryReverseLookupResponse};
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

#[test]
fn bind_unbind_prefix() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app
        .init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 1)
        .unwrap();
    let admin = &accs[0];

    let wasm = Wasm::new(&app);
    let wasm_byte_code = std::fs::read("../contracts/name/artifacts/name.wasm").unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                name: "name-test.sc.pb".to_string(),
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
        &ExecuteMsg::BindPrefix {
            prefix: "nm".to_string(),
        },
        &[],
        admin,
    )?;
    let resolve_response = wasm.query::<QueryMsg, QueryResolveResponse>(
        &contract_addr,
        &QueryMsg::Resolve {
            name: "nm.name-test.sc.pb".to_string(),
        },
    )?;
    assert_eq!(resolve_response.address, contract_addr);
    let lookup_response = wasm.query::<QueryMsg, QueryReverseLookupResponse>(
        &contract_addr,
        &QueryMsg::Lookup {
            address: contract_addr.clone(),
        },
    )?;
    assert_eq!(lookup_response.name.len(), 2);
    assert!(lookup_response
        .name
        .contains(&"name-test.sc.pb".to_string()));
    assert!(lookup_response
        .name
        .contains(&"nm.name-test.sc.pb".to_string()));

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::UnbindPrefix {
            prefix: "nm".to_string(),
        },
        &[],
        admin,
    )?;
    let resolve_response = wasm.query::<QueryMsg, QueryResolveResponse>(
        &contract_addr,
        &QueryMsg::Resolve {
            name: "nm.name-test.sc.pb".to_string(),
        },
    );
    assert!(resolve_response.is_err());
    let lookup_response = wasm.query::<QueryMsg, QueryReverseLookupResponse>(
        &contract_addr,
        &QueryMsg::Lookup {
            address: contract_addr.clone(),
        },
    )?;
    assert_eq!(lookup_response.name.len(), 1);
    assert!(lookup_response
        .name
        .contains(&"name-test.sc.pb".to_string()));

    Ok(())
}
