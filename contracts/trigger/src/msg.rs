use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint64;
use provwasm_std::shim::Any;
use provwasm_std::types::provenance::trigger::v1::{
    QueryTriggerByIdResponse, QueryTriggersResponse,
};

#[cw_serde]
pub enum Event {
    BlockHeightEvent { block_height: Uint64 },
    BlockTimeEvent { timestamp: Uint64 },
}

#[cw_serde]
pub struct InitMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    CreateTrigger { event: Event, to_address: String },
    DeleteTrigger { id: Uint64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(provwasm_std::types::provenance::trigger::v1::QueryTriggerByIdResponse)]
    #[returns(provwasm_std::types::provenance::trigger::v1::QueryTriggersResponse)]
    GetTrigger { id: Option<Uint64> },
}

#[cw_serde]
pub struct TriggerData {
    pub id: u64,
    pub owner: String,
    pub event: Option<Any>,
    pub trigger: Vec<Any>,
}

#[cw_serde]
pub struct TriggersByIdResp {
    pub trigger: Option<TriggerData>,
}

impl From<QueryTriggerByIdResponse> for TriggersByIdResp {
    fn from(response: QueryTriggerByIdResponse) -> Self {
        TriggersByIdResp {
            trigger: match response.trigger {
                None => None,
                Some(trigger) => Some(TriggerData {
                    id: trigger.id,
                    owner: trigger.owner,
                    event: trigger.event,
                    trigger: trigger.actions,
                }),
            },
        }
    }
}

#[cw_serde]
pub struct TriggersResp {
    pub triggers: Vec<TriggerData>,
}

impl From<QueryTriggersResponse> for TriggersResp {
    fn from(response: QueryTriggersResponse) -> Self {
        TriggersResp {
            triggers: response
                .triggers
                .into_iter()
                .map(|trigger| TriggerData {
                    id: trigger.id,
                    owner: trigger.owner,
                    event: trigger.event,
                    trigger: trigger.actions,
                })
                .collect(),
        }
    }
}
