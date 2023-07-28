use cosmwasm_schema::write_api;
use trigger::msg::{ExecuteMsg, InitMsg};

fn main() {
    write_api! {
        execute: ExecuteMsg,
        instantiate: InitMsg,
    }
}
