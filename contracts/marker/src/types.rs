use cosmwasm_schema::cw_serde;
use provwasm_std::types::provenance::marker::v1::MarkerAccount;

#[cw_serde]
pub struct Marker {
    pub marker_account: MarkAccount,
    pub coins: Vec<cosmwasm_std::Coin>,
}

#[cw_serde]
pub struct MarkAccount {
    pub base_account: Option<String>,
    pub manager: String,
    pub demon: String,
    pub supply: String,
}

impl From<MarkerAccount> for MarkAccount {
    fn from(value: MarkerAccount) -> Self {
        MarkAccount {
            base_account: value.base_account.unwrap().address.into(),
            manager: value.manager,
            demon: value.denom.to_string(),
            supply: value.supply.to_string(),
        }
    }
}
