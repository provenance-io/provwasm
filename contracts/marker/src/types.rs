use cosmwasm_schema::cw_serde;
use provwasm_std::types::{cosmos::base::v1beta1::Coin, provenance::marker::v1::MarkerAccount};

#[cw_serde]
pub struct Marker {
    pub marker_account: MarkerAccount,
    pub coins: Vec<Coin>,
}
