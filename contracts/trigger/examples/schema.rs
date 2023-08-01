use cosmwasm_schema::write_api;
use trigger::msg::{ExecuteMsg, InitMsg, QueryMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
        query: QueryMsg,
    }
}
