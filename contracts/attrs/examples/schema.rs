use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use attrs::msg::{ExecuteMsg, InitMsg, Label, LabelNameResponse, LabelsResponse, QueryMsg};
use attrs::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Label), &out_dir);
    export_schema(&schema_for!(LabelNameResponse), &out_dir);
    export_schema(&schema_for!(LabelsResponse), &out_dir);

    export_schema(&schema_for!(InitMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
}
