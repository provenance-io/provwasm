use std::env::current_dir;

use cosmwasm_schema::{export_schema, schema_for, write_api};

use ibc_local::ibc_msg::PacketMsg;
use ibc_local::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    // Clear & write standard API
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }

    // Schemas for ibc
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    export_schema(&schema_for!(PacketMsg), &out_dir);
}
