use provwasm_proc_macro::CosmwasmExt;
/// EventDeposit is an event emitted when assets are deposited into a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventDeposit")]
pub struct EventDeposit {
    /// caller is the address of the account that initiated the deposit.
    #[prost(string, tag = "1")]
    pub caller: ::prost::alloc::string::String,
    /// owner is the address of the account that will receive the minted shares.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// assets is the coins amount string of the underlying assets that were deposited.
    #[prost(string, tag = "3")]
    pub assets: ::prost::alloc::string::String,
    /// shares is the coins amount string of the vault shares that were minted.
    #[prost(string, tag = "4")]
    pub shares: ::prost::alloc::string::String,
    /// vault_id is the numerical identifier of the vault.
    #[prost(uint32, tag = "5")]
    pub vault_id: u32,
}
/// EventWithdraw is an event emitted when assets are withdrawn from a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventWithdraw")]
pub struct EventWithdraw {
    /// caller is the address of the account that initiated the withdrawal.
    #[prost(string, tag = "1")]
    pub caller: ::prost::alloc::string::String,
    /// receiver is the address of the account that will receive the underlying assets.
    #[prost(string, tag = "2")]
    pub receiver: ::prost::alloc::string::String,
    /// owner is the address of the account from which the shares were burned.
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    /// assets is the coins amount string of the underlying assets that were withdrawn.
    #[prost(string, tag = "4")]
    pub assets: ::prost::alloc::string::String,
    /// shares is the coins amount string of the vault shares that were burned.
    #[prost(string, tag = "5")]
    pub shares: ::prost::alloc::string::String,
    /// vault_id is the numerical identifier of the vault.
    #[prost(uint32, tag = "6")]
    pub vault_id: u32,
}
/// EventVaultCreated is an event emitted when a vault is created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventVaultCreated")]
pub struct EventVaultCreated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that manages the vault.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// share_denom is the name of the assets created by the vault used for distribution.
    #[prost(string, tag = "3")]
    pub share_denom: ::prost::alloc::string::String,
    /// underlying_assets is the denominations of the assets supported by the vault.
    #[prost(string, repeated, tag = "4")]
    pub underlying_assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventSwapIn is an event emitted when assets are swapped in for vault shares.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventSwapIn")]
pub struct EventSwapIn {
    /// owner is the address of the account that initiated the swap.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// amount_in is the amount of underlying assets that were swapped in.
    #[prost(message, optional, tag = "2")]
    pub amount_in: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// shares_received is the amount of vault shares that were minted.
    #[prost(message, optional, tag = "3")]
    pub shares_received: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "4")]
    pub vault_address: ::prost::alloc::string::String,
}
/// EventSwapOut is an event emitted when vault shares are swapped out for underlying assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventSwapOut")]
pub struct EventSwapOut {
    /// owner is the address of the account that initiated the swap.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// shares_burned is the amount of vault shares that were burned.
    #[prost(message, optional, tag = "2")]
    pub shares_burned: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// amount_out is the amount of underlying assets that were sent to the recipient.
    #[prost(message, optional, tag = "3")]
    pub amount_out: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "4")]
    pub vault_address: ::prost::alloc::string::String,
}
/// EventVaultReconcile is an event emitted when a vault's interest is reconciled.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventVaultReconcile")]
pub struct EventVaultReconcile {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// principal_before is the principal amount before applying interest.
    #[prost(message, optional, tag = "2")]
    pub principal_before: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// principal_after is the principal amount after applying interest.
    #[prost(message, optional, tag = "3")]
    pub principal_after: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// rate is the interest rate for the period.
    #[prost(string, tag = "4")]
    pub rate: ::prost::alloc::string::String,
    /// time is the payout duration in seconds.
    #[prost(int64, tag = "5")]
    pub time: i64,
    /// interest_earned is the interest amount (can be positive or negative).
    #[prost(message, optional, tag = "6")]
    pub interest_earned: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// EventVaultInterestChange is an event emitted when a vault's interest rate is changed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventVaultInterestChange")]
pub struct EventVaultInterestChange {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// current_rate is the interest rate actual rate the vault is using.
    #[prost(string, tag = "2")]
    pub current_rate: ::prost::alloc::string::String,
    /// desired_rate is the interest rate the admin wants to use.
    #[prost(string, tag = "3")]
    pub desired_rate: ::prost::alloc::string::String,
}
/// EventInterestDeposit is an event emitted when funds are deposited for paying interest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventInterestDeposit")]
pub struct EventInterestDeposit {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that deposited the funds.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// amount is the amount of funds deposited.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// EventInterestWithdrawal is an event emitted when unused interest funds are withdrawn.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventInterestWithdrawal")]
pub struct EventInterestWithdrawal {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that withdrew the funds.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// amount is the amount of funds withdrawn.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// EventToggleSwapIn is an event emitted when swap-in operations are enabled or disabled for a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventToggleSwapIn")]
pub struct EventToggleSwapIn {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that toggled the swap-in operations.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// enabled is the new state of swap-in operations.
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// EventToggleSwapOut is an event emitted when swap-out operations are enabled or disabled for a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventToggleSwapOut")]
pub struct EventToggleSwapOut {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that toggled the swap-out operations.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// enabled is the new state of swap-out operations.
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// EventDepositPrincipalFunds is an event emitted when principal funds are deposited by the admin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventDepositPrincipalFunds")]
pub struct EventDepositPrincipalFunds {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that deposited the funds.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// amount is the amount of funds deposited.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// EventWithdrawPrincipalFunds is an event emitted when principal funds are withdrawn by the admin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventWithdrawPrincipalFunds")]
pub struct EventWithdrawPrincipalFunds {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that withdrew the funds.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// amount is the amount of funds withdrawn.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// EventMinInterestRateUpdated is emitted when the minimum interest rate is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventMinInterestRateUpdated")]
pub struct EventMinInterestRateUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that updated the limit.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// min_rate is the newly set minimum interest rate (as string, can be "").
    #[prost(string, tag = "3")]
    pub min_rate: ::prost::alloc::string::String,
}
/// EventMaxInterestRateUpdated is emitted when the maximum interest rate is updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.EventMaxInterestRateUpdated")]
pub struct EventMaxInterestRateUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that updated the limit.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// max_rate is the newly set maximum interest rate (as string, can be "").
    #[prost(string, tag = "3")]
    pub max_rate: ::prost::alloc::string::String,
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.Params")]
pub struct Params {}
/// VaultAccount represents a central holding place for assets, governed by a set of rules.
/// It is based on the ERC-4626 standard and builds upon the Provenance Marker module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.VaultAccount")]
pub struct VaultAccount {
    /// base_account cosmos account information including address and coin holdings.
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::cosmos::auth::v1beta1::BaseAccount>,
    /// share_denom is the denomination used to represent shares in the vault (e.g., vault tokens).
    #[prost(string, tag = "2")]
    pub share_denom: ::prost::alloc::string::String,
    /// underlying_assets specifies the denomination(s) of the asset(s) managed by the vault.
    #[prost(string, repeated, tag = "3")]
    pub underlying_assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// admin is the address that has administrative privileges over the vault.
    #[prost(string, tag = "4")]
    pub admin: ::prost::alloc::string::String,
    /// current_interest_rate is the actual interest rate currently being applied.
    /// This may be adjusted programmatically (e.g., due to lack of funds).
    #[prost(string, tag = "5")]
    pub current_interest_rate: ::prost::alloc::string::String,
    /// desired_interest_rate is the target interest rate that the vault intends to apply.
    #[prost(string, tag = "6")]
    pub desired_interest_rate: ::prost::alloc::string::String,
    /// min_interest_rate is the lowest interest rate the admin is allowed to set.
    /// If unset (empty string), there is no lower limit.
    #[prost(string, tag = "7")]
    pub min_interest_rate: ::prost::alloc::string::String,
    /// max_interest_rate is the highest interest rate the admin is allowed to set.
    /// If unset (empty string), there is no upper limit.
    #[prost(string, tag = "8")]
    pub max_interest_rate: ::prost::alloc::string::String,
    /// swap_in_enabled indicates whether users are allowed to deposit into the vault.
    #[prost(bool, tag = "9")]
    pub swap_in_enabled: bool,
    /// swap_out_enabled indicates whether users are allowed to withdraw from the vault.
    #[prost(bool, tag = "10")]
    pub swap_out_enabled: bool,
}
/// VaultInterestDetails stores metadata related to interest accrual and payment for a vault.
///
/// period_start represents the Unix timestamp (in seconds) when the current interest
/// accrual period began. This value is updated when interest is successfully paid out.
/// If multiple transactions occur in the same block, only the first will trigger interest reconciliation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.VaultInterestDetails")]
pub struct VaultInterestDetails {
    /// The start time (in Unix seconds) of the current interest accrual period.
    #[prost(int64, tag = "1")]
    pub period_start: i64,
    /// The expire time (in Unix seconds) of the current interest accrual period.
    #[prost(int64, tag = "2")]
    pub expire_time: i64,
}
/// GenesisState defines the vault module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// vaults defines the vaults that exist at genesis.
    #[prost(message, repeated, tag = "2")]
    pub vaults: ::prost::alloc::vec::Vec<VaultAccount>,
}
/// QueryVaultsRequest is the request message for the Query/Vaults endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryVaultsRequest")]
#[proto_query(path = "/vault.v1.Query/Vaults", response_type = QueryVaultsResponse)]
pub struct QueryVaultsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVaultsResponse is the response message for the Query/Vaults endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryVaultsResponse")]
pub struct QueryVaultsResponse {
    /// vaults is a list of all vaults.
    #[prost(message, repeated, tag = "1")]
    pub vaults: ::prost::alloc::vec::Vec<VaultAccount>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryVaultRequest is the request message for the Query/Vault endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryVaultRequest")]
#[proto_query(path = "/vault.v1.Query/Vault", response_type = QueryVaultResponse)]
pub struct QueryVaultRequest {
    /// vault_address is the bech32 address of the vault to query.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
}
/// QueryVaultResponse is the response message for the Query/Vault endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryVaultResponse")]
pub struct QueryVaultResponse {
    /// vault is the requested vault.
    #[prost(message, optional, tag = "1")]
    pub vault: ::core::option::Option<VaultAccount>,
}
/// QueryEstimateSwapInRequest is the request message for the Query/EstimateSwapIn endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryEstimateSwapInRequest")]
#[proto_query(
    path = "/vault.v1.Query/EstimateSwapIn",
    response_type = QueryEstimateSwapInResponse
)]
pub struct QueryEstimateSwapInRequest {
    /// vault_address is the bech32 address of the vault to query.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of underlying assets to swap in.
    #[prost(message, optional, tag = "2")]
    pub assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryEstimateSwapInResponse is the response message for the Query/EstimateSwapIn endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryEstimateSwapInResponse")]
pub struct QueryEstimateSwapInResponse {
    /// assets is the estimated amount of shares that would be received.
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// The block height when the estimate occurred.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// The UTC block time when the estimate occurred.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
}
/// QueryEstimateSwapOutRequest is the request message for the Query/EstimateSwapOut endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryEstimateSwapOutRequest")]
#[proto_query(
    path = "/vault.v1.Query/EstimateSwapOut",
    response_type = QueryEstimateSwapOutResponse
)]
pub struct QueryEstimateSwapOutRequest {
    /// vault_address is the bech32 address of the vault to query.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of shares to swap out.
    #[prost(message, optional, tag = "2")]
    pub assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryEstimateSwapOutResponse is the response message for the Query/EstimateSwapOut endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.QueryEstimateSwapOutResponse")]
pub struct QueryEstimateSwapOutResponse {
    /// assets is the estimated amount of underlying assets that would be received.
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// The block height when the estimate occurred.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// The UTC block time when the estimate occurred.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgCreateVaultRequest is the request message for the CreateVault endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgCreateVaultRequest")]
pub struct MsgCreateVaultRequest {
    /// admin is the creator and initial administrator of the vault.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// share_denom is the name of the assets created by the vault used for distribution.
    #[prost(string, tag = "2")]
    pub share_denom: ::prost::alloc::string::String,
    /// underlying_asset is the denomination of the asset supported by the vault.
    #[prost(string, tag = "3")]
    pub underlying_asset: ::prost::alloc::string::String,
}
/// MsgCreateVaultResponse is the response message for the CreateVault endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgCreateVaultResponse")]
pub struct MsgCreateVaultResponse {
    /// vault_address is the bech32 address of the newly created vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
}
/// MsgSwapInRequest is the request message for depositing underlying assets into a vault in exchange for shares.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgSwapInRequest")]
pub struct MsgSwapInRequest {
    /// owner is the address initiating the swap in (deposit).
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// vault_address is the address of the target vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of underlying assets to deposit.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSwapInResponse is the response message for a successful SwapIn.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgSwapInResponse")]
pub struct MsgSwapInResponse {
    /// shares_received is the amount of vault shares minted.
    #[prost(message, optional, tag = "1")]
    pub shares_received: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSwapOutRequest is the request message for redeeming vault shares in exchange for underlying assets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgSwapOutRequest")]
pub struct MsgSwapOutRequest {
    /// owner is the address initiating the swap out (withdraw).
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// vault_address is the address of the vault to redeem from.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of underlying assets to withdraw.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSwapOutResponse is the response message for a successful SwapOut.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgSwapOutResponse")]
pub struct MsgSwapOutResponse {
    /// shares_burned is the amount of shares burned in exchange for assets.
    #[prost(message, optional, tag = "1")]
    pub shares_burned: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgUpdateMinInterestRateRequest")]
pub struct MsgUpdateMinInterestRateRequest {
    /// The address of the account authorized to update the minimum interest rate for the vault.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The bech32 address of the vault whose minimum interest rate is being updated.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// The minimum allowable interest rate for the vault.
    /// Provide an empty string ("") to disable the minimum interest rate limit.
    #[prost(string, tag = "3")]
    pub min_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgUpdateMinInterestRateResponse")]
pub struct MsgUpdateMinInterestRateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgUpdateMaxInterestRateRequest")]
pub struct MsgUpdateMaxInterestRateRequest {
    /// The address of the account authorized to update the maximum interest rate for the vault.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The bech32 address of the vault whose maximum interest rate is being updated.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// The maximum allowable interest rate for the vault.
    /// Provide an empty string ("") to disable the maximum interest rate limit.
    #[prost(string, tag = "3")]
    pub max_rate: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgUpdateMaxInterestRateResponse")]
pub struct MsgUpdateMaxInterestRateResponse {}
/// MsgUpdateInterestRateRequest is the request message for updating the interest rate of a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgUpdateInterestRateRequest")]
pub struct MsgUpdateInterestRateRequest {
    /// admin is the address of the vault administrator.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// new_rate is the new interest rate for the vault, expressed as an APY string (e.g., "-5.00" for -5%).
    #[prost(string, tag = "3")]
    pub new_rate: ::prost::alloc::string::String,
}
/// MsgUpdateInterestRateResponse is the response message for the UpdateInterestRate endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgUpdateInterestRateResponse")]
pub struct MsgUpdateInterestRateResponse {}
/// MsgDepositInterestFundsRequest is the request message for depositing funds to be used for paying interest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgDepositInterestFundsRequest")]
pub struct MsgDepositInterestFundsRequest {
    /// admin is the address of the account depositing the funds.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to which funds are being deposited.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to deposit.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositInterestFundsResponse is the response message for the DepositInterestFunds endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgDepositInterestFundsResponse")]
pub struct MsgDepositInterestFundsResponse {}
/// MsgWithdrawInterestFundsRequest is the request message for withdrawing unused interest funds.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgWithdrawInterestFundsRequest")]
pub struct MsgWithdrawInterestFundsRequest {
    /// admin is the address of the vault administrator initiating the withdrawal.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault from which funds are being withdrawn.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to withdraw.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawInterestFundsResponse is the response message for the WithdrawInterestFunds endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgWithdrawInterestFundsResponse")]
pub struct MsgWithdrawInterestFundsResponse {}
/// MsgToggleSwapInRequest is the request message for enabling or disabling swap-in operations for a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgToggleSwapInRequest")]
pub struct MsgToggleSwapInRequest {
    /// admin is the address of the vault administrator.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// enabled specifies whether swap-in operations should be enabled (true) or disabled (false).
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// MsgToggleSwapInResponse is the response message for the ToggleSwapIn endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgToggleSwapInResponse")]
pub struct MsgToggleSwapInResponse {}
/// MsgToggleSwapOutRequest is the request message for enabling or disabling swap-out operations for a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgToggleSwapOutRequest")]
pub struct MsgToggleSwapOutRequest {
    /// admin is the address of the vault administrator.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// enabled specifies whether swap-out operations should be enabled (true) or disabled (false).
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// MsgToggleSwapOutResponse is the response message for the ToggleSwapOut endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgToggleSwapOutResponse")]
pub struct MsgToggleSwapOutResponse {}
/// MsgDepositPrincipalFundsRequest is the request message for depositing principal funds into a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgDepositPrincipalFundsRequest")]
pub struct MsgDepositPrincipalFundsRequest {
    /// admin is the address of the account depositing the funds.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to which funds are being deposited.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to deposit.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositPrincipalFundsResponse is the response message for the DepositPrincipalFunds endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgDepositPrincipalFundsResponse")]
pub struct MsgDepositPrincipalFundsResponse {}
/// MsgWithdrawPrincipalFundsRequest is the request message for withdrawing principal funds from a vault.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgWithdrawPrincipalFundsRequest")]
pub struct MsgWithdrawPrincipalFundsRequest {
    /// admin is the address of the vault administrator initiating the withdrawal.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault from which funds are being withdrawn.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to withdraw.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawPrincipalFundsResponse is the response message for the WithdrawPrincipalFunds endpoint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/vault.v1.MsgWithdrawPrincipalFundsResponse")]
pub struct MsgWithdrawPrincipalFundsResponse {}
pub struct VaultQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> VaultQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn vaults(
        &self,
        pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryVaultsResponse, cosmwasm_std::StdError> {
        QueryVaultsRequest { pagination }.query(self.querier)
    }
    pub fn vault(
        &self,
        vault_address: ::prost::alloc::string::String,
    ) -> Result<QueryVaultResponse, cosmwasm_std::StdError> {
        QueryVaultRequest { vault_address }.query(self.querier)
    }
    pub fn estimate_swap_in(
        &self,
        vault_address: ::prost::alloc::string::String,
        assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    ) -> Result<QueryEstimateSwapInResponse, cosmwasm_std::StdError> {
        QueryEstimateSwapInRequest {
            vault_address,
            assets,
        }
        .query(self.querier)
    }
    pub fn estimate_swap_out(
        &self,
        vault_address: ::prost::alloc::string::String,
        assets: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    ) -> Result<QueryEstimateSwapOutResponse, cosmwasm_std::StdError> {
        QueryEstimateSwapOutRequest {
            vault_address,
            assets,
        }
        .query(self.querier)
    }
}
