use cosmwasm_std::Coin;
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

use attrs::msg::{ExecuteMsg, InitMsg, Label, LabelNameResponse, LabelsResponse, QueryMsg};

#[test]
fn create_update_delete_attr() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::default();
    let accs = app
        .init_accounts(&[Coin::new(100_000_000_000_000, "nhash")], 1)
        .unwrap();
    let admin = &accs[0];

    let wasm = Wasm::new(&app);
    let wasm_byte_code = std::fs::read("../contracts/attrs/artifacts/attrs.wasm").unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                name: "attrs-test.sc.pb".to_string(),
            },
            Some(&admin.address()),
            None,
            &[],
            admin,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(&contract_addr, &ExecuteMsg::BindLabelName {}, &[], admin)?;
    let label_name_response =
        wasm.query::<QueryMsg, LabelNameResponse>(&contract_addr, &QueryMsg::GetLabelName {})?;
    assert_eq!(label_name_response.name, "label.attrs-test.sc.pb");

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::AddLabel {
            text: "hello".to_string(),
        },
        &[],
        admin,
    )?;
    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::AddLabel {
            text: "wasm".to_string(),
        },
        &[],
        admin,
    )?;
    let label_name_response =
        wasm.query::<QueryMsg, LabelsResponse>(&contract_addr, &QueryMsg::GetLabels {})?;
    assert!(label_name_response.labels.contains(&Label {
        text: "hello".to_string()
    }));
    assert!(label_name_response.labels.contains(&Label {
        text: "wasm".to_string()
    }));

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::UpdateLabel {
            original_text: "hello".to_string(),
            update_text: "goodbye".to_string(),
        },
        &[],
        admin,
    )?;
    let label_name_response =
        wasm.query::<QueryMsg, LabelsResponse>(&contract_addr, &QueryMsg::GetLabels {})?;
    assert!(label_name_response.labels.contains(&Label {
        text: "goodbye".to_string()
    }));
    assert!(label_name_response.labels.contains(&Label {
        text: "wasm".to_string()
    }));

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::DeleteDistinctLabel {
            text: "wasm".to_string(),
        },
        &[],
        admin,
    )?;
    let label_name_response =
        wasm.query::<QueryMsg, LabelsResponse>(&contract_addr, &QueryMsg::GetLabels {})?;
    assert!(label_name_response.labels.contains(&Label {
        text: "goodbye".to_string()
    }));

    wasm.execute::<ExecuteMsg>(&contract_addr, &ExecuteMsg::DeleteLabels {}, &[], admin)?;

    let label_name_response =
        wasm.query::<QueryMsg, LabelsResponse>(&contract_addr, &QueryMsg::GetLabels {})?;
    assert!(label_name_response.labels.is_empty());

    Ok(())
}
