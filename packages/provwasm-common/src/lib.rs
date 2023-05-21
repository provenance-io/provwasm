use cosmwasm_std::{Binary, QuerierResult};
pub trait MockableQuerier {
    fn register_custom_query(
        &mut self,
        path: String,
        response_fn: Box<dyn Fn(&Binary) -> QuerierResult>,
    );
}
