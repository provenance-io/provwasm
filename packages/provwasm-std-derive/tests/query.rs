use cosmwasm_std::{Empty, QueryRequest};
use provwasm_std_derive::CosmwasmExt;

#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/provenance.test.v1.QueryTestRequest")]
#[proto_query(
    path = "/desmos.test.v1.Query/Test",
    response_type = QueryTestResponse
)]
pub struct QueryTestRequest {
    #[prost(string, tag = "1")]
    pub creator: ::prost::alloc::string::String,
}
#[derive(
    Clone, PartialEq, Eq, ::prost::Message, serde::Serialize, serde::Deserialize, CosmwasmExt,
)]
#[proto_message(type_url = "/desmos.test.v1.QueryTestResponse")]
pub struct QueryTestResponse {
    #[prost(string, repeated, tag = "1")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}

fn main() {
    let proto = QueryTestRequest {
        creator: "test".to_string(),
    };
    let request: QueryRequest<Empty> = proto.clone().into();
    assert_eq!(
        QueryRequest::<Empty>::Stargate {
            path: "/desmos.test.v1.Query/Test".into(),
            data: proto.into()
        },
        request
    )
}
