use provwasm_proc_macro::CosmwasmExt;
/// GenesisState defines the ibc channel/v2 submodule's genesis state.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "2")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "3")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "4")]
    pub receipts: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "5")]
    pub async_packets: ::prost::alloc::vec::Vec<PacketState>,
    #[prost(message, repeated, tag = "6")]
    pub send_sequences: ::prost::alloc::vec::Vec<PacketSequence>,
}
/// PacketState defines the generic type necessary to retrieve and store
/// packet commitments, acknowledgements, and receipts.
/// Caller is responsible for knowing the context necessary to interpret this
/// state as a commitment, acknowledgement, or a receipt.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.PacketState")]
pub struct PacketState {
    /// client unique identifier.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// packet sequence.
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    /// embedded data that represents packet state.
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// PacketSequence defines the genesis type necessary to retrieve and store next send sequences.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.PacketSequence")]
pub struct PacketSequence {
    /// client unique identifier.
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
}
/// Packet defines a type that carries data across different chains through IBC
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.Packet")]
pub struct Packet {
    /// number corresponds to the order of sends and receives, where a Packet
    /// with an earlier sequence number must be sent and received before a Packet
    /// with a later sequence number.
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
    /// identifies the sending client on the sending chain.
    #[prost(string, tag = "2")]
    pub source_client: ::prost::alloc::string::String,
    /// identifies the receiving client on the receiving chain.
    #[prost(string, tag = "3")]
    pub destination_client: ::prost::alloc::string::String,
    /// timeout timestamp in seconds after which the packet times out.
    #[prost(uint64, tag = "4")]
    pub timeout_timestamp: u64,
    /// a list of payloads, each one for a specific application.
    #[prost(message, repeated, tag = "5")]
    pub payloads: ::prost::alloc::vec::Vec<Payload>,
}
/// Payload contains the source and destination ports and payload for the application (version, encoding, raw bytes)
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.Payload")]
pub struct Payload {
    /// specifies the source port of the packet.
    #[prost(string, tag = "1")]
    pub source_port: ::prost::alloc::string::String,
    /// specifies the destination port of the packet.
    #[prost(string, tag = "2")]
    pub destination_port: ::prost::alloc::string::String,
    /// version of the specified application.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// the encoding used for the provided value.
    #[prost(string, tag = "4")]
    pub encoding: ::prost::alloc::string::String,
    /// the raw bytes for the payload.
    #[prost(bytes = "vec", tag = "5")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Acknowledgement contains a list of all ack results associated with a single packet.
/// In the case of a successful receive, the acknowledgement will contain an app acknowledgement
/// for each application that received a payload in the same order that the payloads were sent
/// in the packet.
/// If the receive is not successful, the acknowledgement will contain a single app acknowledgment
/// which will be a constant error acknowledgment as defined by the IBC v2 protocol.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.Acknowledgement")]
pub struct Acknowledgement {
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub app_acknowledgements: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// RecvPacketResult speecifies the status of a packet as well as the acknowledgement bytes.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.RecvPacketResult")]
pub struct RecvPacketResult {
    /// status of the packet
    #[prost(enumeration = "PacketStatus", tag = "1")]
    pub status: i32,
    /// acknowledgement of the packet
    #[prost(bytes = "vec", tag = "2")]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
}
/// PacketStatus specifies the status of a RecvPacketResult.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ::prost::Enumeration, ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum PacketStatus {
    /// PACKET_STATUS_UNSPECIFIED indicates an unknown packet status.
    Unspecified = 0,
    /// PACKET_STATUS_SUCCESS indicates a successful packet receipt.
    Success = 1,
    /// PACKET_STATUS_FAILURE indicates a failed packet receipt.
    Failure = 2,
    /// PACKET_STATUS_ASYNC indicates an async packet receipt.
    Async = 3,
}
impl PacketStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "PACKET_STATUS_UNSPECIFIED",
            Self::Success => "PACKET_STATUS_SUCCESS",
            Self::Failure => "PACKET_STATUS_FAILURE",
            Self::Async => "PACKET_STATUS_ASYNC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PACKET_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PACKET_STATUS_SUCCESS" => Some(Self::Success),
            "PACKET_STATUS_FAILURE" => Some(Self::Failure),
            "PACKET_STATUS_ASYNC" => Some(Self::Async),
            _ => None,
        }
    }
}
/// QueryNextSequenceSendRequest is the request type for the Query/QueryNextSequenceSend RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryNextSequenceSendRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/NextSequenceSend",
    response_type = QueryNextSequenceSendResponse
)]
pub struct QueryNextSequenceSendRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
}
/// QueryNextSequenceSendResponse is the response type for the Query/QueryNextSequenceSend RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryNextSequenceSendResponse")]
pub struct QueryNextSequenceSendResponse {
    /// next sequence send number
    #[prost(uint64, tag = "1")]
    pub next_sequence_send: u64,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketCommitmentRequest is the request type for the Query/PacketCommitment RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketCommitmentRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/PacketCommitment",
    response_type = QueryPacketCommitmentResponse
)]
pub struct QueryPacketCommitmentRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
}
/// QueryPacketCommitmentResponse is the response type for the Query/PacketCommitment RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketCommitmentResponse")]
pub struct QueryPacketCommitmentResponse {
    /// packet associated with the request fields
    #[prost(bytes = "vec", tag = "1")]
    pub commitment: ::prost::alloc::vec::Vec<u8>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketCommitmentsRequest is the request type for the Query/PacketCommitments RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketCommitmentsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/PacketCommitments",
    response_type = QueryPacketCommitmentsResponse
)]
pub struct QueryPacketCommitmentsRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
}
/// QueryPacketCommitmentResponse is the response type for the Query/PacketCommitment RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketCommitmentsResponse")]
pub struct QueryPacketCommitmentsResponse {
    /// collection of packet commitments for the requested channel identifier.
    #[prost(message, repeated, tag = "1")]
    pub commitments: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height.
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketAcknowledgementRequest is the request type for the Query/PacketAcknowledgement RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketAcknowledgementRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/PacketAcknowledgement",
    response_type = QueryPacketAcknowledgementResponse
)]
pub struct QueryPacketAcknowledgementRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
}
/// QueryPacketAcknowledgementResponse is the response type for the Query/PacketAcknowledgement RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketAcknowledgementResponse")]
pub struct QueryPacketAcknowledgementResponse {
    /// acknowledgement associated with the request fields
    #[prost(bytes = "vec", tag = "1")]
    pub acknowledgement: ::prost::alloc::vec::Vec<u8>,
    /// merkle proof of existence
    #[prost(bytes = "vec", tag = "2")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketAcknowledgementsRequest is the request type for the
/// Query/QueryPacketCommitments RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketAcknowledgementsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/PacketAcknowledgements",
    response_type = QueryPacketAcknowledgementsResponse
)]
pub struct QueryPacketAcknowledgementsRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// pagination request
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    /// list of packet sequences
    #[prost(uint64, repeated, tag = "3")]
    pub packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryPacketAcknowledgemetsResponse is the request type for the
/// Query/QueryPacketAcknowledgements RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketAcknowledgementsResponse")]
pub struct QueryPacketAcknowledgementsResponse {
    #[prost(message, repeated, tag = "1")]
    pub acknowledgements: ::prost::alloc::vec::Vec<PacketState>,
    /// pagination response
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
    /// query block height
    #[prost(message, optional, tag = "3")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryPacketReceiptRequest is the request type for the Query/PacketReceipt RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketReceiptRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/PacketReceipt",
    response_type = QueryPacketReceiptResponse
)]
pub struct QueryPacketReceiptRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// packet sequence
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
}
/// QueryPacketReceiptResponse is the response type for the Query/PacketReceipt RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryPacketReceiptResponse")]
pub struct QueryPacketReceiptResponse {
    /// success flag for if receipt exists
    #[prost(bool, tag = "2")]
    pub received: bool,
    /// merkle proof of existence or absence
    #[prost(bytes = "vec", tag = "3")]
    pub proof: ::prost::alloc::vec::Vec<u8>,
    /// height at which the proof was retrieved
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryUnreceivedPacketsRequest is the request type for the Query/UnreceivedPackets RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryUnreceivedPacketsRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/UnreceivedPackets",
    response_type = QueryUnreceivedPacketsResponse
)]
pub struct QueryUnreceivedPacketsRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// list of packet sequences
    #[prost(uint64, repeated, tag = "2")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryUnreceivedPacketsResponse is the response type for the Query/UnreceivedPacketCommitments RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryUnreceivedPacketsResponse")]
pub struct QueryUnreceivedPacketsResponse {
    /// list of unreceived packet sequences
    #[prost(uint64, repeated, tag = "1")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// QueryUnreceivedAcks is the request type for the
/// Query/UnreceivedAcks RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryUnreceivedAcksRequest")]
#[proto_query(
    path = "/ibc.core.channel.v2.Query/UnreceivedAcks",
    response_type = QueryUnreceivedAcksResponse
)]
pub struct QueryUnreceivedAcksRequest {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    /// list of acknowledgement sequences
    #[prost(uint64, repeated, tag = "2")]
    pub packet_ack_sequences: ::prost::alloc::vec::Vec<u64>,
}
/// QueryUnreceivedAcksResponse is the response type for the
/// Query/UnreceivedAcks RPC method
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.QueryUnreceivedAcksResponse")]
pub struct QueryUnreceivedAcksResponse {
    /// list of unreceived acknowledgement sequences
    #[prost(uint64, repeated, tag = "1")]
    pub sequences: ::prost::alloc::vec::Vec<u64>,
    /// query block height
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<super::super::client::v1::Height>,
}
/// MsgSendPacket sends an outgoing IBC packet.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgSendPacket")]
pub struct MsgSendPacket {
    #[prost(string, tag = "1")]
    pub source_client: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub timeout_timestamp: u64,
    #[prost(message, repeated, tag = "3")]
    pub payloads: ::prost::alloc::vec::Vec<Payload>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgSendPacketResponse defines the Msg/SendPacket response type.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgSendPacketResponse")]
pub struct MsgSendPacketResponse {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
/// MsgRecvPacket receives an incoming IBC packet.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgRecvPacket")]
pub struct MsgRecvPacket {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof_commitment: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "4")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgRecvPacketResponse defines the Msg/RecvPacket response type.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgRecvPacketResponse")]
pub struct MsgRecvPacketResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
/// MsgTimeout receives timed-out packet
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgTimeout")]
pub struct MsgTimeout {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(bytes = "vec", tag = "2")]
    pub proof_unreceived: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgTimeoutResponse defines the Msg/Timeout response type.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgTimeoutResponse")]
pub struct MsgTimeoutResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
/// MsgAcknowledgement receives incoming IBC acknowledgement.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgAcknowledgement")]
pub struct MsgAcknowledgement {
    #[prost(message, optional, tag = "1")]
    pub packet: ::core::option::Option<Packet>,
    #[prost(message, optional, tag = "2")]
    pub acknowledgement: ::core::option::Option<Acknowledgement>,
    #[prost(bytes = "vec", tag = "3")]
    pub proof_acked: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_height: ::core::option::Option<super::super::client::v1::Height>,
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// MsgAcknowledgementResponse defines the Msg/Acknowledgement response type.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/ibc.core.channel.v2.MsgAcknowledgementResponse")]
pub struct MsgAcknowledgementResponse {
    #[prost(enumeration = "ResponseResultType", tag = "1")]
    pub result: i32,
}
/// ResponseResultType defines the possible outcomes of the execution of a message
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ::prost::Enumeration, ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum ResponseResultType {
    /// Default zero value enumeration
    Unspecified = 0,
    /// The message did not call the IBC application callbacks (because, for example, the packet had already been relayed)
    Noop = 1,
    /// The message was executed successfully
    Success = 2,
    /// The message was executed unsuccessfully
    Failure = 3,
}
impl ResponseResultType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "RESPONSE_RESULT_TYPE_UNSPECIFIED",
            Self::Noop => "RESPONSE_RESULT_TYPE_NOOP",
            Self::Success => "RESPONSE_RESULT_TYPE_SUCCESS",
            Self::Failure => "RESPONSE_RESULT_TYPE_FAILURE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESPONSE_RESULT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "RESPONSE_RESULT_TYPE_NOOP" => Some(Self::Noop),
            "RESPONSE_RESULT_TYPE_SUCCESS" => Some(Self::Success),
            "RESPONSE_RESULT_TYPE_FAILURE" => Some(Self::Failure),
            _ => None,
        }
    }
}
pub struct ChannelQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> ChannelQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn next_sequence_send(
        &self,
        client_id: ::prost::alloc::string::String,
    ) -> Result<QueryNextSequenceSendResponse, cosmwasm_std::StdError> {
        QueryNextSequenceSendRequest { client_id }.query(self.querier)
    }
    pub fn packet_commitment(
        &self,
        client_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<QueryPacketCommitmentResponse, cosmwasm_std::StdError> {
        QueryPacketCommitmentRequest {
            client_id,
            sequence,
        }
        .query(self.querier)
    }
    pub fn packet_commitments(
        &self,
        client_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPacketCommitmentsResponse, cosmwasm_std::StdError> {
        QueryPacketCommitmentsRequest {
            client_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn packet_acknowledgement(
        &self,
        client_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<QueryPacketAcknowledgementResponse, cosmwasm_std::StdError> {
        QueryPacketAcknowledgementRequest {
            client_id,
            sequence,
        }
        .query(self.querier)
    }
    pub fn packet_acknowledgements(
        &self,
        client_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
        packet_commitment_sequences: ::prost::alloc::vec::Vec<u64>,
    ) -> Result<QueryPacketAcknowledgementsResponse, cosmwasm_std::StdError> {
        QueryPacketAcknowledgementsRequest {
            client_id,
            pagination,
            packet_commitment_sequences,
        }
        .query(self.querier)
    }
    pub fn packet_receipt(
        &self,
        client_id: ::prost::alloc::string::String,
        sequence: u64,
    ) -> Result<QueryPacketReceiptResponse, cosmwasm_std::StdError> {
        QueryPacketReceiptRequest {
            client_id,
            sequence,
        }
        .query(self.querier)
    }
    pub fn unreceived_packets(
        &self,
        client_id: ::prost::alloc::string::String,
        sequences: ::prost::alloc::vec::Vec<u64>,
    ) -> Result<QueryUnreceivedPacketsResponse, cosmwasm_std::StdError> {
        QueryUnreceivedPacketsRequest {
            client_id,
            sequences,
        }
        .query(self.querier)
    }
    pub fn unreceived_acks(
        &self,
        client_id: ::prost::alloc::string::String,
        packet_ack_sequences: ::prost::alloc::vec::Vec<u64>,
    ) -> Result<QueryUnreceivedAcksResponse, cosmwasm_std::StdError> {
        QueryUnreceivedAcksRequest {
            client_id,
            packet_ack_sequences,
        }
        .query(self.querier)
    }
}
