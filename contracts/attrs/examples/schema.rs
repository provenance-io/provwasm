use attrs::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        migrate: MigrateMsg,
        query: QueryMsg,
    }
}
