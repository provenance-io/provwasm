use cosmwasm_schema::write_api;
use provwasm_tutorial::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        migrate: MigrateMsg,
        query: QueryMsg,
    }
}
