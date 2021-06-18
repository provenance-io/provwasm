use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use provwasm_std::{
    Attributes, Marker, Names, ProvenanceMsg, ProvenanceQuery, ProvenanceRoute, Scope,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(Attributes), &out_dir);
    export_schema(&schema_for!(Marker), &out_dir);
    export_schema(&schema_for!(Names), &out_dir);
    export_schema(&schema_for!(Scope), &out_dir);

    export_schema(&schema_for!(ProvenanceMsg), &out_dir);
    export_schema(&schema_for!(ProvenanceQuery), &out_dir);
    export_schema(&schema_for!(ProvenanceRoute), &out_dir);
}
