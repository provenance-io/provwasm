use provwasm_proc_macro::CosmwasmExt;
/// EventNFTRegistered is the event emitted when a registry is created for an NFT.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.EventNFTRegistered")]
pub struct EventNftRegistered {
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
}
/// EventRoleGranted is the event emitted when an address is granted a role in a registry.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.EventRoleGranted")]
pub struct EventRoleGranted {
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventRoleRevoked is the event emitted when an address has a role revoked in a registry.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.EventRoleRevoked")]
pub struct EventRoleRevoked {
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub role: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventNFTUnregistered is the event emitted when a registry is deleted from an NFT.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.EventNFTUnregistered")]
pub struct EventNftUnregistered {
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
}
/// EventRegistryBulkUpdated is the event emitted to indicate that a registry has been updated as part of a bulk process.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.EventRegistryBulkUpdated")]
pub struct EventRegistryBulkUpdated {
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
}
/// RegistryKey represents a unique identifier for registry entries.
/// It links registry entries to specific NFT assets and their associated asset classes.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.RegistryKey")]
pub struct RegistryKey {
    /// Identifier for the nft that this ledger is linked to.
    /// This could be a `x/metadata` scope id or an `x/nft` nft id.
    /// In order to create a ledger for an NFT, the NFT class must be registered in the ledger module as a LedgerClass.
    #[prost(string, tag = "1")]
    pub nft_id: ::prost::alloc::string::String,
    /// asset_class_id is the Scope Specification ID or NFT Class ID.
    /// This identifies the class or specification that the NFT belongs to.
    /// If it is a Scope Specification, the address is used, e.g."scopespec1qnwg86nsatx5pl56muw0v9ytlz3qu3jx6m".
    /// Otherwise, it is the NFT Class ID, e.g. "heloc-loan-class".
    #[prost(string, tag = "2")]
    pub asset_class_id: ::prost::alloc::string::String,
}
/// RegistryEntry represents a single entry in the registry, mapping a blockchain address to its roles.
/// Each entry contains the key information and the roles assigned to various addresses.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.RegistryEntry")]
pub struct RegistryEntry {
    /// key ties the registry entry to an asset class and NFT id.
    /// This provides the unique identifier for the registry entry.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<RegistryKey>,
    /// roles is a list of roles and addresses that can perform that role.
    /// Each role entry contains a role type and the addresses authorized for that role.
    #[prost(message, repeated, tag = "2")]
    pub roles: ::prost::alloc::vec::Vec<RolesEntry>,
}
/// RolesEntry represents a mapping between a role and the addresses that can perform that role.
/// This allows multiple addresses to be assigned the same role for a given registry entry.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.RolesEntry")]
pub struct RolesEntry {
    /// role is the type of role being assigned.
    #[prost(enumeration = "RegistryRole", tag = "1")]
    pub role: i32,
    /// addresses is the list of blockchain addresses that can perform this role.
    /// These addresses have the permissions associated with the specified role.
    #[prost(string, repeated, tag = "2")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RegistryRole defines the different types of roles that can be assigned to addresses.
/// These roles determine the permissions and capabilities that an address has within the registry system.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, ::prost::Enumeration, ::schemars::JsonSchema,
)]
#[repr(i32)]
pub enum RegistryRole {
    /// REGISTRY_ROLE_UNSPECIFIED indicates no role is assigned.
    /// This is the default state for addresses that have not been granted any specific permissions.
    Unspecified = 0,
    /// REGISTRY_ROLE_SERVICER indicates the address has servicer privileges.
    /// Servicers are responsible for maintaining and servicing the underlying assets.
    Servicer = 1,
    /// REGISTRY_ROLE_SUBSERVICER indicates the address has subservicer privileges.
    /// Subservicers assist servicers in their duties and may have limited administrative capabilities.
    Subservicer = 2,
    /// REGISTRY_ROLE_CONTROLLER indicates the address has controller privileges.
    /// Controllers have administrative control over the registry entries and can modify roles.
    Controller = 3,
    /// REGISTRY_ROLE_CUSTODIAN indicates the address has custodian privileges.
    /// Custodians are responsible for holding and safeguarding the underlying assets.
    Custodian = 4,
    /// REGISTRY_ROLE_BORROWER indicates the address has borrower privileges.
    /// Borrowers can borrow against the underlying assets within defined parameters.
    Borrower = 5,
    /// REGISTRY_ROLE_ORIGINATOR indicates the address has originator privileges.
    /// Originators are responsible for creating and originating the underlying assets.
    Originator = 6,
}
impl RegistryRole {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Unspecified => "REGISTRY_ROLE_UNSPECIFIED",
            Self::Servicer => "REGISTRY_ROLE_SERVICER",
            Self::Subservicer => "REGISTRY_ROLE_SUBSERVICER",
            Self::Controller => "REGISTRY_ROLE_CONTROLLER",
            Self::Custodian => "REGISTRY_ROLE_CUSTODIAN",
            Self::Borrower => "REGISTRY_ROLE_BORROWER",
            Self::Originator => "REGISTRY_ROLE_ORIGINATOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REGISTRY_ROLE_UNSPECIFIED" => Some(Self::Unspecified),
            "REGISTRY_ROLE_SERVICER" => Some(Self::Servicer),
            "REGISTRY_ROLE_SUBSERVICER" => Some(Self::Subservicer),
            "REGISTRY_ROLE_CONTROLLER" => Some(Self::Controller),
            "REGISTRY_ROLE_CUSTODIAN" => Some(Self::Custodian),
            "REGISTRY_ROLE_BORROWER" => Some(Self::Borrower),
            "REGISTRY_ROLE_ORIGINATOR" => Some(Self::Originator),
            _ => None,
        }
    }
}
/// GenesisState defines the registry module's genesis state.
/// This contains all the registry entries that exist when the blockchain is first initialized.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.GenesisState")]
pub struct GenesisState {
    /// entries is the list of registry entries.
    /// These entries define the initial state of the registry module.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<RegistryEntry>,
}
/// QueryGetRegistryRequest is the request type for the Query/GetRegistry RPC method.
/// It contains the key information needed to retrieve a specific registry entry.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.QueryGetRegistryRequest")]
#[proto_query(
    path = "/provenance.registry.v1.Query/GetRegistry",
    response_type = QueryGetRegistryResponse
)]
pub struct QueryGetRegistryRequest {
    /// key is the registry key to query.
    /// This contains the NFT ID and asset class ID that uniquely identify the registry entry.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<RegistryKey>,
}
/// QueryGetRegistryResponse is the response type for the Query/GetRegistry RPC method.
/// It contains the complete registry entry for the requested key.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.QueryGetRegistryResponse")]
pub struct QueryGetRegistryResponse {
    /// registry is the registry entry for the requested key.
    /// This includes all roles and addresses associated with the specified NFT and asset class.
    #[prost(message, optional, tag = "1")]
    pub registry: ::core::option::Option<RegistryEntry>,
}
/// QueryGetRegistriesRequest is the paginated request type for the Query/GetRegistries RPC method.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.QueryGetRegistriesRequest")]
#[proto_query(
    path = "/provenance.registry.v1.Query/GetRegistries",
    response_type = QueryGetRegistriesResponse
)]
pub struct QueryGetRegistriesRequest {
    /// asset_class_id is the Scope Specification ID or NFT Class ID.
    /// This identifies the class or specification that the NFTs belong to.
    #[prost(string, tag = "1")]
    pub asset_class_id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryGetRegistriesResponse is the paginated response type for the Query/GetRegistries RPC method.
/// It contains the complete registry entry for the requested key.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.QueryGetRegistriesResponse")]
pub struct QueryGetRegistriesResponse {
    /// registries is the collection of registry entries.
    /// This includes all roles and addresses associated with the specified NFT and asset class.
    #[prost(message, repeated, tag = "1")]
    pub registries: ::prost::alloc::vec::Vec<RegistryEntry>,
    /// pagination is the pagination details for this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryHasRoleRequest is the request type for the Query/HasRole RPC method.
/// It contains the information needed to verify if an address has a specific role.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.QueryHasRoleRequest")]
#[proto_query(
    path = "/provenance.registry.v1.Query/HasRole",
    response_type = QueryHasRoleResponse
)]
pub struct QueryHasRoleRequest {
    /// key is the registry key to query.
    /// This identifies the specific registry entry to check.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<RegistryKey>,
    /// address is the blockchain address to check for the role.
    /// This is the address whose role permissions are being verified.
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    /// role is the specific role to check for.
    /// This determines which role permission is being verified.
    #[prost(enumeration = "RegistryRole", tag = "3")]
    pub role: i32,
}
/// QueryHasRoleResponse is the response type for the Query/HasRole RPC method.
/// It provides a boolean result indicating whether the address has the specified role.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.QueryHasRoleResponse")]
pub struct QueryHasRoleResponse {
    /// has_role is true if the address has the specified role for the given key.
    /// This boolean value indicates whether the role verification was successful.
    #[prost(bool, tag = "1")]
    pub has_role: bool,
}
/// MsgRegisterNFT represents a message to register a new NFT in the registry.
/// This message creates a new registry entry with the specified roles and addresses.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgRegisterNFT")]
pub struct MsgRegisterNft {
    /// signer is the address that is authorized to register NFTs.
    /// This address must have the appropriate permissions to create registry entries.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// key is the registry key to register.
    /// This contains the NFT ID and asset class ID that uniquely identify the registry entry.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<RegistryKey>,
    /// roles is a list of roles and addresses that can perform that role.
    /// Each role entry defines a role type and the addresses authorized for that role.
    #[prost(message, repeated, tag = "3")]
    pub roles: ::prost::alloc::vec::Vec<RolesEntry>,
}
/// MsgRegisterNFTResponse defines the response for RegisterNFT.
/// This is an empty response indicating successful registration.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgRegisterNFTResponse")]
pub struct MsgRegisterNftResponse {}
/// MsgGrantRole represents a message to grant a role to one or more addresses.
/// This message adds the specified addresses to an existing role for the given registry key.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgGrantRole")]
pub struct MsgGrantRole {
    /// signer is the address that is authorized to grant the role.
    /// This address must have the appropriate permissions to modify role assignments.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// key is the registry key to grant the role to.
    /// This identifies the specific registry entry to modify.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<RegistryKey>,
    /// role is the role to grant.
    /// This specifies which role type is being assigned to the addresses.
    #[prost(enumeration = "RegistryRole", tag = "3")]
    pub role: i32,
    /// addresses is the list of addresses to grant the role to.
    /// These addresses will be added to the specified role for the registry key.
    #[prost(string, repeated, tag = "4")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgGrantRoleResponse defines the response for GrantRole.
/// This is an empty response indicating successful role grant.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgGrantRoleResponse")]
pub struct MsgGrantRoleResponse {}
/// MsgRevokeRole represents a message to revoke a role from one or more addresses.
/// This message removes the specified addresses from an existing role for the given registry key.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgRevokeRole")]
pub struct MsgRevokeRole {
    /// signer is the address that is authorized to revoke the role.
    /// This address must have the appropriate permissions to modify role assignments.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// key is the registry key to revoke the role from.
    /// This identifies the specific registry entry to modify.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<RegistryKey>,
    /// role is the role to revoke.
    /// This specifies which role type is being removed from the addresses.
    #[prost(enumeration = "RegistryRole", tag = "3")]
    pub role: i32,
    /// addresses is the list of addresses to revoke the role from.
    /// These addresses will be removed from the specified role for the registry key.
    #[prost(string, repeated, tag = "4")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgRevokeRoleResponse defines the response for RevokeRole.
/// This is an empty response indicating successful role revocation.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgRevokeRoleResponse")]
pub struct MsgRevokeRoleResponse {}
/// MsgUnregisterNFT represents a message to unregister an NFT from the registry.
/// This message removes the entire registry entry for the specified key.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgUnregisterNFT")]
pub struct MsgUnregisterNft {
    /// signer is the address that is authorized to unregister NFTs.
    /// This address must have the appropriate permissions to remove registry entries.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// key is the registry key to remove.
    /// This identifies the specific registry entry to delete.
    #[prost(message, optional, tag = "2")]
    pub key: ::core::option::Option<RegistryKey>,
}
/// MsgUnregisterNFTResponse defines the response for UnregisterNFT.
/// This is an empty response indicating successful unregistration.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgUnregisterNFTResponse")]
pub struct MsgUnregisterNftResponse {}
/// MsgRegistryBulkUpdate represents a bulk update operation for multiple registry entries.
/// This allows for efficient batch processing of registry modifications.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgRegistryBulkUpdate")]
pub struct MsgRegistryBulkUpdate {
    /// signer is the address that is authorized to register NFTs.
    /// This address must have the appropriate permissions to create registry entries.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// entries is the list of bulk update entries to be processed.
    /// Each entry contains the registry entry to be updated.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<RegistryEntry>,
}
/// MsgRegistryBulkUpdateResponse defines the response for RegistryBulkUpdate.
/// This is an empty response indicating successful bulk update.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provenance.registry.v1.MsgRegistryBulkUpdateResponse")]
pub struct MsgRegistryBulkUpdateResponse {}
pub struct RegistryQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> RegistryQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn get_registry(
        &self,
        key: ::core::option::Option<RegistryKey>,
    ) -> Result<QueryGetRegistryResponse, cosmwasm_std::StdError> {
        QueryGetRegistryRequest { key }.query(self.querier)
    }
    pub fn get_registries(
        &self,
        asset_class_id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryGetRegistriesResponse, cosmwasm_std::StdError> {
        QueryGetRegistriesRequest {
            asset_class_id,
            pagination,
        }
        .query(self.querier)
    }
    pub fn has_role(
        &self,
        key: ::core::option::Option<RegistryKey>,
        address: ::prost::alloc::string::String,
        role: i32,
    ) -> Result<QueryHasRoleResponse, cosmwasm_std::StdError> {
        QueryHasRoleRequest { key, address, role }.query(self.querier)
    }
}
