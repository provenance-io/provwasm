use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InitMsg {
    pub name: String, // Bind this name to the contract address (eg contract.pb).
}

#[cw_serde]
pub struct Label {
    pub text: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    BindLabelName {},
    AddLabel {
        text: String,
    },
    DeleteDistinctLabel {
        text: String,
    },
    DeleteLabels {},
    UpdateLabel {
        original_text: String,
        update_text: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(LabelNameResponse)]
    GetLabelName {},
    #[returns(LabelsResponse)]
    GetLabels {},
}

#[cw_serde]
pub struct LabelNameResponse {
    pub name: String,
}

#[cw_serde]
pub struct LabelsResponse {
    pub labels: Vec<Label>,
}

/// Migrate the contract.
#[cw_serde]
pub struct MigrateMsg {}
