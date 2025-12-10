use provwasm_proc_macro::CosmwasmExt;
/// EventDeposit is an event emitted when assets are deposited into a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventDeposit")]
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
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventWithdraw")]
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
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultCreated")]
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
    /// underlying_asset is the vault’s primary collateral and valuation/base denomination.
    #[prost(string, tag = "4")]
    pub underlying_asset: ::prost::alloc::string::String,
}
/// EventDenomUnit describes a single denom unit entry that is included in the
/// share denom metadata emitted with EventSetShareDenomMetadata.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventDenomUnit")]
pub struct EventDenomUnit {
    /// denom is the unit name (e.g., "nushare", "ushare").
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// exponent is the base-10 exponent for this unit relative to the base denom.
    /// For example, exponent=6 means 1 display unit = 10^6 base units.
    #[prost(string, tag = "2")]
    pub exponent: ::prost::alloc::string::String,
    /// aliases lists optional alternative names for this unit. May be empty.
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventSetShareDenomMetadata is emitted when denom metadata is set for a vault’s
/// share denom (via MsgSetShareDenomMetadata).
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSetShareDenomMetadata")]
pub struct EventSetShareDenomMetadata {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// metadata_base is the base denomination (e.g., "nushare").
    #[prost(string, tag = "2")]
    pub metadata_base: ::prost::alloc::string::String,
    /// metadata_description is a human-readable description of the share denom.
    #[prost(string, tag = "3")]
    pub metadata_description: ::prost::alloc::string::String,
    /// metadata_display is the display denomination (e.g., "ushare" or "SHARE").
    #[prost(string, tag = "4")]
    pub metadata_display: ::prost::alloc::string::String,
    /// metadata_denom_units lists all denom units and their exponents.
    #[prost(message, repeated, tag = "5")]
    pub metadata_denom_units: ::prost::alloc::vec::Vec<EventDenomUnit>,
    /// administrator is the bech32 address of the signer that set the metadata.
    #[prost(string, tag = "6")]
    pub administrator: ::prost::alloc::string::String,
    /// metadata_name is the descriptive name for the share denom.
    #[prost(string, tag = "7")]
    pub metadata_name: ::prost::alloc::string::String,
    /// metadata_symbol is the short ticker-style symbol (optional).
    #[prost(string, tag = "8")]
    pub metadata_symbol: ::prost::alloc::string::String,
}
/// EventSwapIn is an event emitted when assets are swapped in for vault shares.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSwapIn")]
pub struct EventSwapIn {
    /// owner is the address of the account that initiated the swap.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// amount_in is the amount of underlying assets that were swapped in.
    #[prost(string, tag = "2")]
    pub amount_in: ::prost::alloc::string::String,
    /// shares_received is the amount of vault shares that were minted.
    #[prost(string, tag = "3")]
    pub shares_received: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "4")]
    pub vault_address: ::prost::alloc::string::String,
}
/// EventSwapOut is an event emitted when vault shares are swapped out for underlying assets.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSwapOut")]
pub struct EventSwapOut {
    /// owner is the address of the account that initiated the swap.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// shares_burned is the amount of vault shares that were burned.
    #[prost(string, tag = "2")]
    pub shares_burned: ::prost::alloc::string::String,
    /// amount_out is the amount of underlying assets that were sent to the recipient.
    #[prost(string, tag = "3")]
    pub amount_out: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "4")]
    pub vault_address: ::prost::alloc::string::String,
}
/// EventVaultReconcile is an event emitted when a vault's interest is reconciled.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultReconcile")]
pub struct EventVaultReconcile {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// principal_before is the principal amount before applying interest.
    #[prost(string, tag = "2")]
    pub principal_before: ::prost::alloc::string::String,
    /// principal_after is the principal amount after applying interest.
    #[prost(string, tag = "3")]
    pub principal_after: ::prost::alloc::string::String,
    /// rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing annual interest rate for the period.
    #[prost(string, tag = "4")]
    pub rate: ::prost::alloc::string::String,
    /// time is the payout duration in seconds.
    #[prost(int64, tag = "5")]
    pub time: i64,
    /// interest_earned is the interest amount (can be positive or negative).
    #[prost(string, tag = "6")]
    pub interest_earned: ::prost::alloc::string::String,
}
/// EventVaultInterestChange is an event emitted when a vault's interest rate is changed.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultInterestChange")]
pub struct EventVaultInterestChange {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// current_rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing the actual annual interest rate the vault is using.
    #[prost(string, tag = "2")]
    pub current_rate: ::prost::alloc::string::String,
    /// desired_rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing the the annual interest rate the admin wants to use.
    #[prost(string, tag = "3")]
    pub desired_rate: ::prost::alloc::string::String,
}
/// EventInterestDeposit is an event emitted when funds are deposited for paying interest.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventInterestDeposit")]
pub struct EventInterestDeposit {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that deposited the funds.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// amount is the amount of funds deposited.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
/// EventInterestWithdrawal is an event emitted when unused interest funds are withdrawn.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventInterestWithdrawal")]
pub struct EventInterestWithdrawal {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that withdrew the funds.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// amount is the amount of funds withdrawn.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
/// EventToggleSwapIn is an event emitted when swap-in operations are enabled or disabled for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventToggleSwapIn")]
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
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventToggleSwapOut")]
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
/// EventDepositPrincipalFunds is an event emitted when principal funds are deposited by the authority.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventDepositPrincipalFunds")]
pub struct EventDepositPrincipalFunds {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that deposited the funds.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// amount is the amount of funds deposited.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
/// EventWithdrawPrincipalFunds is an event emitted when principal funds are withdrawn by the authority.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventWithdrawPrincipalFunds")]
pub struct EventWithdrawPrincipalFunds {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that withdrew the funds.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// amount is the amount of funds withdrawn.
    #[prost(string, tag = "3")]
    pub amount: ::prost::alloc::string::String,
}
/// EventMinInterestRateUpdated is emitted when the minimum interest rate is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventMinInterestRateUpdated")]
pub struct EventMinInterestRateUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that updated the limit.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// min_rate is the newly set minimum annual interest rate as a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%).
    /// An empty string "" represents no minimum.
    #[prost(string, tag = "3")]
    pub min_rate: ::prost::alloc::string::String,
}
/// EventMaxInterestRateUpdated is emitted when the maximum interest rate is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventMaxInterestRateUpdated")]
pub struct EventMaxInterestRateUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that updated the limit.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// max_rate is the newly set maximum annual interest rate as a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%).
    /// An empty string "" represents no maximum.
    #[prost(string, tag = "3")]
    pub max_rate: ::prost::alloc::string::String,
}
/// EventSwapOutRequested is emitted when a user successfully queues a swap out.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSwapOutRequested")]
pub struct EventSwapOutRequested {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// owner is the bech32 address of the user who initiated the swap out.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// redeem_denom is the denomination of the asset to be redeemed.
    #[prost(string, tag = "3")]
    pub redeem_denom: ::prost::alloc::string::String,
    /// shares is the amount of vault shares the user escrowed for this request.
    #[prost(string, tag = "4")]
    pub shares: ::prost::alloc::string::String,
    /// request_id is the unique identifier for this pending swap out request.
    #[prost(uint64, tag = "5")]
    pub request_id: u64,
}
/// EventSwapOutCompleted is emitted when a pending swap out is successfully processed.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSwapOutCompleted")]
pub struct EventSwapOutCompleted {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// owner is the bech32 address of the user who received the payout.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// assets is the amount of assets paid out to the user.
    #[prost(string, tag = "3")]
    pub assets: ::prost::alloc::string::String,
    /// request_id is the unique identifier of the swap out request that was completed.
    #[prost(uint64, tag = "4")]
    pub request_id: u64,
}
/// EventSwapOutRefunded is emitted when a pending swap out fails and the user's
/// escrowed shares are returned.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSwapOutRefunded")]
pub struct EventSwapOutRefunded {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// owner is the bech32 address of the user whose shares were refunded.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// shares is the amount of vault shares that were returned to the user.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
    /// request_id is the unique identifier of the swap out request that failed.
    #[prost(uint64, tag = "4")]
    pub request_id: u64,
    /// reason is a string detailing why the swap out failed.
    #[prost(string, tag = "5")]
    pub reason: ::prost::alloc::string::String,
}
/// EventPendingSwapOutExpedited is an event emitted when a pending swap-out is expedited by the authority.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventPendingSwapOutExpedited")]
pub struct EventPendingSwapOutExpedited {
    /// request_id is the numerical identifier of the pending swap-out.
    #[prost(uint64, tag = "1")]
    pub request_id: u64,
    /// vault is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that expedited the swap-out.
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// EventVaultPaused is emitted when a vault is paused.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultPaused")]
pub struct EventVaultPaused {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that paused the vault.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// reason is the reason for pausing the vault.
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// total_vault_value is the total value of the vault's assets at the time of pausing.
    #[prost(string, tag = "4")]
    pub total_vault_value: ::prost::alloc::string::String,
}
/// EventVaultUnpaused is emitted when a vault is unpaused.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultUnpaused")]
pub struct EventVaultUnpaused {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that unpaused the vault.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// total_vault_value is the new total value of the vault's assets at the time of unpausing.
    #[prost(string, tag = "3")]
    pub total_vault_value: ::prost::alloc::string::String,
}
/// EventBridgeAddressSet is emitted when the bridge address for a vault is configured or updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventBridgeAddressSet")]
pub struct EventBridgeAddressSet {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that set the bridge address.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// bridge_address is the configured external address allowed to mint/burn shares.
    #[prost(string, tag = "3")]
    pub bridge_address: ::prost::alloc::string::String,
}
/// EventBridgeToggled is emitted when the bridge functionality is enabled or disabled for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventBridgeToggled")]
pub struct EventBridgeToggled {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the account that toggled bridge functionality.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// enabled is the new state of the bridge functionality.
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// EventBridgeMintShares is emitted when shares are minted via the bridge flow.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventBridgeMintShares")]
pub struct EventBridgeMintShares {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// bridge is the bech32 address of the bridge signer.
    #[prost(string, tag = "2")]
    pub bridge: ::prost::alloc::string::String,
    /// shares is the amount of shares minted.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
}
/// EventBridgeBurnShares is emitted when shares are burned via the bridge flow.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventBridgeBurnShares")]
pub struct EventBridgeBurnShares {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// bridge is the bech32 address of the bridge signer.
    #[prost(string, tag = "2")]
    pub bridge: ::prost::alloc::string::String,
    /// shares is the amount of shares burned.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
}
/// EventAssetManagerSet is emitted when a vault's asset manager is set or cleared.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventAssetManagerSet")]
pub struct EventAssetManagerSet {
    /// vault_address is the address of the vault whose asset manager was updated.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the admin who performed the update.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// asset_manager is the new asset manager address. If empty, it indicates the asset manager was cleared.
    #[prost(string, tag = "3")]
    pub asset_manager: ::prost::alloc::string::String,
}
/// VaultAccount represents a central holding place for assets, governed by a set of rules.
/// It is based on the ERC-4626 standard and builds upon the Provenance Marker module.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.VaultAccount")]
pub struct VaultAccount {
    /// base_account cosmos account information including address and coin holdings.
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
    /// total_shares is the total number of shares that have ever been issued by the vault.
    /// It serves as the canonical supply-of-record for all shares, regardless of whether
    /// they are held locally on Provenance or externally (e.g., bridged).
    #[prost(message, optional, tag = "2")]
    pub total_shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// underlying_asset is the vault’s single principal collateral AND valuation/base unit.
    /// - Exactly one denom.
    /// - Total Vault Value (TVV) and NAV-per-share are computed and reported in this denom.
    /// - Interest accrual and internal accounting are measured in this denom.
    /// - Any other coin accepted for I/O must have a NAV record priced INTO this denom.
    #[prost(string, tag = "3")]
    pub underlying_asset: ::prost::alloc::string::String,
    /// payment_denom is the single optional external payment coin supported for user I/O
    /// alongside the underlying_asset.
    /// - If unset, the vault operates single-denom: deposits/withdrawals only in underlying_asset.
    /// - If set, swap-in/out accept either underlying_asset OR payment_denom (one denom per call).
    /// - Must differ from share_denom and underlying_asset.
    /// - Requires an on-chain NAV record mapping payment_denom -> underlying_asset to value deposits
    ///    and redemptions.
    #[prost(string, tag = "4")]
    pub payment_denom: ::prost::alloc::string::String,
    /// admin is the address that has administrative privileges over the vault.
    #[prost(string, tag = "5")]
    pub admin: ::prost::alloc::string::String,
    /// current_interest_rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing the actual annual interest rate currently being applied.
    /// This may be adjusted programmatically (e.g., due to lack of funds).
    #[prost(string, tag = "6")]
    pub current_interest_rate: ::prost::alloc::string::String,
    /// desired_interest_rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing the target annual interest rate that the vault intends to apply.
    #[prost(string, tag = "7")]
    pub desired_interest_rate: ::prost::alloc::string::String,
    /// min_interest_rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing the lowest annual interest rate the admin is allowed to set.
    /// If unset (empty string), there is no lower limit.
    #[prost(string, tag = "8")]
    pub min_interest_rate: ::prost::alloc::string::String,
    /// max_interest_rate is a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%) representing the highest annual interest rate the admin is allowed to set.
    /// If unset (empty string), there is no upper limit.
    #[prost(string, tag = "9")]
    pub max_interest_rate: ::prost::alloc::string::String,
    /// The start time (in Unix seconds) of the current interest accrual period.
    #[prost(int64, tag = "10")]
    pub period_start: i64,
    /// The expire time (in Unix seconds) of the current interest accrual period.
    #[prost(int64, tag = "11")]
    pub period_timeout: i64,
    /// swap_in_enabled indicates whether users are allowed to deposit into the vault.
    #[prost(bool, tag = "12")]
    pub swap_in_enabled: bool,
    /// swap_out_enabled indicates whether users are allowed to withdraw from the vault.
    #[prost(bool, tag = "13")]
    pub swap_out_enabled: bool,
    /// withdrawal_delay_seconds is the configured time period (in seconds) that a withdrawal
    /// request must wait in the pending queue before being processed.
    #[prost(uint64, tag = "14")]
    pub withdrawal_delay_seconds: u64,
    /// paused indicates that all user-facing swap-in and swap-out operations are disabled.
    #[prost(bool, tag = "15")]
    pub paused: bool,
    /// paused_balance is the total vault value snapshot taken at the moment of pausing.
    /// This value is used for all NAV calculations while the vault is paused to prevent
    /// apparent devaluation during collateral rebalancing. It is cleared upon unpausing.
    #[prost(message, optional, tag = "16")]
    pub paused_balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// paused_reason is a human-readable string explaining why the vault was paused, particularly for automatic pauses.
    #[prost(string, tag = "17")]
    pub paused_reason: ::prost::alloc::string::String,
    /// bridge_address is the single external address allowed to mint or burn shares on behalf
    /// of this vault (e.g., for bridging to another chain). All mint/burn must flow through the
    /// vault keeper, which enforces that marker supply never exceeds total_shares.
    #[prost(string, tag = "18")]
    pub bridge_address: ::prost::alloc::string::String,
    /// bridge_enabled indicates whether the bridge functionality is active. If false, the
    /// bridge_address has no effect and cannot mint or burn.
    #[prost(bool, tag = "19")]
    pub bridge_enabled: bool,
    /// asset_manager is an optional address that, when set, is authorized to manage certain
    /// collateral operations alongside the admin (e.g., pausing/unpausing, depositing/withdrawing
    /// principal or interest funds). If unset (empty string), only the admin may perform those actions.
    #[prost(string, tag = "20")]
    pub asset_manager: ::prost::alloc::string::String,
}
/// AccountBalance represents the coin balance of a single account.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.AccountBalance")]
pub struct AccountBalance {
    /// address is the account address.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// coins is the balance of the account.
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// PendingSwapOut are swap outs that have not yet been processed and completed.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.PendingSwapOut")]
pub struct PendingSwapOut {
    /// owner is the address initiating the swap out.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// vault_address is the address of the vault processing the withdrawal.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// shares are the shares that were escrowed by the user.
    #[prost(message, optional, tag = "3")]
    pub shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// redeem_denom is the denomination of the asset to be redeemed.
    #[prost(string, tag = "4")]
    pub redeem_denom: ::prost::alloc::string::String,
}
/// QueueEntry is a (time, addr) pair used by the vault payout deferral queue.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueueEntry")]
pub struct QueueEntry {
    /// time is the UNIX timestamp (in seconds) when the entry becomes eligible.
    #[prost(uint64, tag = "1")]
    pub time: u64,
    /// addr is the bech32 vault address associated with the entry.
    #[prost(string, tag = "2")]
    pub addr: ::prost::alloc::string::String,
}
/// PendingSwapOutQueueEntry represents a single pending swap-out request queued for later processing.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.PendingSwapOutQueueEntry")]
pub struct PendingSwapOutQueueEntry {
    /// time is the UNIX timestamp (in seconds) when this pending swap-out was enqueued or becomes eligible.
    #[prost(int64, tag = "1")]
    pub time: i64,
    /// id is the unique identifier of the pending swap-out request.
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// swap_out contains the pending swap-out details.
    #[prost(message, optional, tag = "3")]
    pub swap_out: ::core::option::Option<PendingSwapOut>,
}
/// PendingSwapOutQueue holds the latest sequence number and all queued swap-out entries.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.PendingSwapOutQueue")]
pub struct PendingSwapOutQueue {
    /// latest_sequence_number is the most recently assigned pending swap-out ID.
    #[prost(uint64, tag = "1")]
    pub latest_sequence_number: u64,
    /// entries contains all currently queued pending swap-out entries.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<PendingSwapOutQueueEntry>,
}
/// GenesisState defines the vault module's genesis state.
/// NOTE: payout verification queue is not imported or exported.  It will always be empty after endblocker processes it.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.GenesisState")]
pub struct GenesisState {
    /// vaults defines the vaults that exist at genesis.
    #[prost(message, repeated, tag = "1")]
    pub vaults: ::prost::alloc::vec::Vec<VaultAccount>,
    /// payout_timeout_queue contains (time, addr) entries for vaults that are
    /// temporarily deferred from automatic payout/interest verification until the
    /// given UNIX timestamp (seconds). These entries are re-enqueued on InitGenesis.
    #[prost(message, repeated, tag = "2")]
    pub payout_timeout_queue: ::prost::alloc::vec::Vec<QueueEntry>,
    /// pending_swap_out_queue contains entries for pending swap outs.
    #[prost(message, optional, tag = "3")]
    pub pending_swap_out_queue: ::core::option::Option<PendingSwapOutQueue>,
}
/// QueryVaultPendingSwapOutsRequest is the request message for the Query/VaultPendingSwapOuts endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultPendingSwapOutsRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/VaultPendingSwapOuts",
    response_type = QueryVaultPendingSwapOutsResponse
)]
pub struct QueryVaultPendingSwapOutsRequest {
    /// id is the bech32 address of the vault or the vault's share denom to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVaultPendingSwapOutsResponse is the response message for the Query/VaultPendingSwapOuts endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultPendingSwapOutsResponse")]
pub struct QueryVaultPendingSwapOutsResponse {
    /// pending_swap_outs is a list of all pending swap outs.
    #[prost(message, repeated, tag = "1")]
    pub pending_swap_outs: ::prost::alloc::vec::Vec<PendingSwapOutWithTimeout>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryPendingSwapOutsRequest is the request message for the Query/PendingSwapOuts endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryPendingSwapOutsRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/PendingSwapOuts",
    response_type = QueryPendingSwapOutsResponse
)]
pub struct QueryPendingSwapOutsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryPendingSwapOutsResponse is the response message for the Query/PendingSwapOuts endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryPendingSwapOutsResponse")]
pub struct QueryPendingSwapOutsResponse {
    /// pending_swap_outs is a list of all pending swap outs.
    #[prost(message, repeated, tag = "1")]
    pub pending_swap_outs: ::prost::alloc::vec::Vec<PendingSwapOutWithTimeout>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// PendingSwapOutWithTimeout is a pending swap out with its timeout.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.PendingSwapOutWithTimeout")]
pub struct PendingSwapOutWithTimeout {
    /// request_id is the unique identifier for the pending swap out request.
    #[prost(uint64, tag = "1")]
    pub request_id: u64,
    /// pending_swap_out contains the details of the swap out request.
    #[prost(message, optional, tag = "2")]
    pub pending_swap_out: ::core::option::Option<PendingSwapOut>,
    /// timeout is the time at which the pending swap out will expire if not processed.
    #[prost(message, optional, tag = "3")]
    pub timeout: ::core::option::Option<crate::shim::Timestamp>,
}
/// QueryVaultsRequest is the request message for the Query/Vaults endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultsRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/Vaults",
    response_type = QueryVaultsResponse
)]
pub struct QueryVaultsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVaultsResponse is the response message for the Query/Vaults endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultsResponse")]
pub struct QueryVaultsResponse {
    /// vaults is a list of all vaults.
    #[prost(message, repeated, tag = "1")]
    pub vaults: ::prost::alloc::vec::Vec<VaultAccount>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryVaultRequest is the request message for the Query/Vault endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/Vault",
    response_type = QueryVaultResponse
)]
pub struct QueryVaultRequest {
    /// id is the bech32 address of the vault or the vault's share denom to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryVaultResponse is the response message for the Query/Vault endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultResponse")]
pub struct QueryVaultResponse {
    /// vault is the requested vault.
    #[prost(message, optional, tag = "1")]
    pub vault: ::core::option::Option<VaultAccount>,
    /// principal is the total amount of principal held in the vault's marker.
    #[prost(message, optional, tag = "2")]
    pub principal: ::core::option::Option<AccountBalance>,
    /// reserves is the total amount of reserves held in the vault account for interest payments.
    #[prost(message, optional, tag = "3")]
    pub reserves: ::core::option::Option<AccountBalance>,
    /// total_vault_value is the estimated total value of the vault in its
    /// underlying asset. It includes current principal and estimated unpaid
    /// interest (at query block height), but excludes reserves. The value is approximate and may differ
    /// from the reconciled amount.
    #[prost(message, optional, tag = "4")]
    pub total_vault_value: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryEstimateSwapInRequest is the request message for the Query/EstimateSwapIn endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryEstimateSwapInRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/EstimateSwapIn",
    response_type = QueryEstimateSwapInResponse
)]
pub struct QueryEstimateSwapInRequest {
    /// vault_address is the bech32 address of the vault to query.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of underlying or payment denom to swap in.
    #[prost(message, optional, tag = "2")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryEstimateSwapInResponse is the response message for the Query/EstimateSwapIn endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryEstimateSwapInResponse")]
pub struct QueryEstimateSwapInResponse {
    /// assets is the estimated amount of shares that would be received.
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The block height when the estimate occurred.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// The UTC block time when the estimate occurred.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
}
/// QueryEstimateSwapOutRequest is the request message for the Query/EstimateSwapOut endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryEstimateSwapOutRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/EstimateSwapOut",
    response_type = QueryEstimateSwapOutResponse
)]
pub struct QueryEstimateSwapOutRequest {
    /// vault_address is the bech32 address of the vault to query.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// shares is the amount of shares to swap out.
    #[prost(string, tag = "2")]
    pub shares: ::prost::alloc::string::String,
    /// redeem_denom is the payout denom to estimate; if empty, the underlying asset is used.
    #[prost(string, tag = "3")]
    pub redeem_denom: ::prost::alloc::string::String,
}
/// QueryEstimateSwapOutResponse is the response message for the Query/EstimateSwapOut endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryEstimateSwapOutResponse")]
pub struct QueryEstimateSwapOutResponse {
    /// assets is the estimated amount of underlying assets that would be received.
    #[prost(message, optional, tag = "1")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// The block height when the estimate occurred.
    #[prost(int64, tag = "2")]
    pub height: i64,
    /// The UTC block time when the estimate occurred.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgCreateVaultRequest is the request message for the CreateVault endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgCreateVaultRequest")]
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
    /// payment_denom is an optional secondary denomination the vault can accept.
    #[prost(string, tag = "4")]
    pub payment_denom: ::prost::alloc::string::String,
    /// withdrawal_delay_seconds is the time period (in seconds) that a withdrawal
    /// must wait in the pending queue before being processed.
    #[prost(uint64, tag = "5")]
    pub withdrawal_delay_seconds: u64,
}
/// MsgCreateVaultResponse is the response message for the CreateVault endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgCreateVaultResponse")]
pub struct MsgCreateVaultResponse {
    /// vault_address is the bech32 address of the newly created vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
}
/// MsgSetShareDenomMetadataRequest defines the request to set bank denom metadata for a vault's share denom.
/// The target denom is derived from the vault identified by vault_address.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSetShareDenomMetadataRequest")]
pub struct MsgSetShareDenomMetadataRequest {
    /// metadata is the bank module Metadata to assign to the vault's share denom.
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<super::super::super::cosmos::bank::v1beta1::Metadata>,
    /// admin is the address of the vault administrator authorizing this update.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault whose share denom metadata is being set.
    #[prost(string, tag = "3")]
    pub vault_address: ::prost::alloc::string::String,
}
/// MsgSetShareDenomMetadataResponse defines the response for setting a vault's share denom metadata.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSetShareDenomMetadataResponse")]
pub struct MsgSetShareDenomMetadataResponse {}
/// MsgSwapInRequest is the request message for depositing underlying assets into a vault in exchange for shares.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSwapInRequest")]
pub struct MsgSwapInRequest {
    /// owner is the address initiating the swap in (deposit).
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// vault_address is the address of the target vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of underlying assets to deposit.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSwapInResponse is the response message for a successful SwapIn.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSwapInResponse")]
pub struct MsgSwapInResponse {
    /// shares_received is the amount of vault shares minted to the depositor.
    #[prost(message, optional, tag = "1")]
    pub shares_received: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgSwapOutRequest is the request message for redeeming vault shares in exchange for underlying assets.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSwapOutRequest")]
pub struct MsgSwapOutRequest {
    /// owner is the address initiating the swap out (withdraw).
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// vault_address is the address of the vault to redeem from.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// assets is the amount of underlying assets to withdraw.
    #[prost(message, optional, tag = "3")]
    pub assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// redeem_denom selects the payout coin.
    /// - If empty, defaults to the vault’s underlying_asset.
    /// - Must be either the vault’s underlying_asset or its payment_denom.
    #[prost(string, tag = "4")]
    pub redeem_denom: ::prost::alloc::string::String,
}
/// MsgSwapOutResponse is the response message for the SwapOut endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSwapOutResponse")]
pub struct MsgSwapOutResponse {
    /// request_id is the unique identifier for the newly queued swap out request.
    #[prost(uint64, tag = "1")]
    pub request_id: u64,
}
/// MsgUpdateMinInterestRateRequest is the request message for updating the minimum interest rate of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMinInterestRateRequest")]
pub struct MsgUpdateMinInterestRateRequest {
    /// The address of the account authorized to update the minimum interest rate for the vault.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The bech32 address of the vault whose minimum interest rate is being updated.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// min_rate is the minimum allowable interest rate(APY) for the vault as a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%).
    /// An empty string "" represents no minimum.
    #[prost(string, tag = "3")]
    pub min_rate: ::prost::alloc::string::String,
}
/// MsgUpdateMinInterestRateResponse is the response message for the UpdateMinInterestRate endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMinInterestRateResponse")]
pub struct MsgUpdateMinInterestRateResponse {}
/// MsgUpdateMaxInterestRateRequest is the request message for updating the maximum interest rate of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMaxInterestRateRequest")]
pub struct MsgUpdateMaxInterestRateRequest {
    /// The address of the account authorized to update the maximum interest rate for the vault.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// The bech32 address of the vault whose maximum interest rate is being updated.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// max_rate is the maximum allowable annual interest rate for the vault as a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%).
    /// An empty string "" represents no maximum.
    #[prost(string, tag = "3")]
    pub max_rate: ::prost::alloc::string::String,
}
/// MsgUpdateMaxInterestRateResponse is the response message for the UpdateMaxInterestRate endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMaxInterestRateResponse")]
pub struct MsgUpdateMaxInterestRateResponse {}
/// MsgUpdateInterestRateRequest is the request message for updating the annual interest rate of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateInterestRateRequest")]
pub struct MsgUpdateInterestRateRequest {
    /// authority is the address of the vault administrator or asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// new_rate is the new annual interest rate for the the vault as a decimal string (e.g., "0.9" for 90% and "0.9001353" for 90.01353%).
    #[prost(string, tag = "3")]
    pub new_rate: ::prost::alloc::string::String,
}
/// MsgUpdateInterestRateResponse is the response message for the UpdateInterestRate endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateInterestRateResponse")]
pub struct MsgUpdateInterestRateResponse {}
/// MsgToggleSwapInRequest is the request message for enabling or disabling swap-in operations for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgToggleSwapInRequest")]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgToggleSwapInResponse")]
pub struct MsgToggleSwapInResponse {}
/// MsgToggleSwapOutRequest is the request message for enabling or disabling swap-out operations for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgToggleSwapOutRequest")]
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
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgToggleSwapOutResponse")]
pub struct MsgToggleSwapOutResponse {}
/// MsgDepositInterestFundsRequest is the request message for depositing funds to be used for paying interest.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgDepositInterestFundsRequest")]
pub struct MsgDepositInterestFundsRequest {
    /// authority is the address of the account depositing the funds.
    /// Must match either the vault admin or the configured asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to which funds are being deposited.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to deposit.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositInterestFundsResponse is the response message for the DepositInterestFunds endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgDepositInterestFundsResponse")]
pub struct MsgDepositInterestFundsResponse {}
/// MsgWithdrawInterestFundsRequest is the request message for withdrawing unused interest funds.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgWithdrawInterestFundsRequest")]
pub struct MsgWithdrawInterestFundsRequest {
    /// authority is the address of the vault administrator or asset manager initiating the withdrawal.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault from which funds are being withdrawn.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to withdraw.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawInterestFundsResponse is the response message for the WithdrawInterestFunds endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgWithdrawInterestFundsResponse")]
pub struct MsgWithdrawInterestFundsResponse {}
/// MsgDepositPrincipalFundsRequest is the request message for depositing principal funds into a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgDepositPrincipalFundsRequest")]
pub struct MsgDepositPrincipalFundsRequest {
    /// authority is the address of the account depositing the funds.
    /// Must match either the vault admin or the configured asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to which funds are being deposited.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to deposit.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgDepositPrincipalFundsResponse is the response message for the DepositPrincipalFunds endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgDepositPrincipalFundsResponse")]
pub struct MsgDepositPrincipalFundsResponse {}
/// MsgWithdrawPrincipalFundsRequest is the request message for withdrawing principal funds from a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgWithdrawPrincipalFundsRequest")]
pub struct MsgWithdrawPrincipalFundsRequest {
    /// authority is the address of the vault administrator or asset manager initiating the withdrawal.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault from which funds are being withdrawn.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// amount is the amount of funds to withdraw.
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawPrincipalFundsResponse is the response message for the WithdrawPrincipalFunds endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgWithdrawPrincipalFundsResponse")]
pub struct MsgWithdrawPrincipalFundsResponse {}
/// MsgExpeditePendingSwapOutRequest is the request message for expediting a swap out from a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgExpeditePendingSwapOutRequest")]
pub struct MsgExpeditePendingSwapOutRequest {
    /// authority is the address of the vault admin or asset manager initiating the expedite.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// request_id is the id of the pending swap out to expedite.
    #[prost(uint64, tag = "2")]
    pub request_id: u64,
}
/// MsgExpeditePendingSwapOutResponse is the response message for the ExpeditePendingSwapOut endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgExpeditePendingSwapOutResponse")]
pub struct MsgExpeditePendingSwapOutResponse {}
/// MsgPauseVaultRequest is the request message to pause a vault. When processed,
/// the vault disables user-facing swap operations and records the provided reason.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgPauseVaultRequest")]
pub struct MsgPauseVaultRequest {
    /// authority is the address of the vault administrator or asset manager initiating the pause.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to pause.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// reason is a human-readable explanation for pausing the vault. This is recorded
    /// for operators and clients to understand the context (e.g., maintenance or anomaly).
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
}
/// MsgPauseVaultResponse is the response message for the PauseVault endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgPauseVaultResponse")]
pub struct MsgPauseVaultResponse {}
/// MsgUnpauseVaultRequest is the request message to unpause a vault. When processed,
/// the vault re-enables user-facing swap operations (subject to existing flags).
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUnpauseVaultRequest")]
pub struct MsgUnpauseVaultRequest {
    /// authority is the address of the vault administrator or asset manager initiating the unpause.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to unpause.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
}
/// MsgUnpauseVaultResponse is the response message for the UnpauseVault endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUnpauseVaultResponse")]
pub struct MsgUnpauseVaultResponse {}
/// MsgSetBridgeAddressRequest is the request message for configuring the bridge address for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSetBridgeAddressRequest")]
pub struct MsgSetBridgeAddressRequest {
    /// admin is the address of the vault administrator.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to update.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// bridge_address is the single external address allowed to mint or burn shares on behalf of this vault.
    #[prost(string, tag = "3")]
    pub bridge_address: ::prost::alloc::string::String,
}
/// MsgSetBridgeAddressResponse is the response message for the SetBridgeAddress endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSetBridgeAddressResponse")]
pub struct MsgSetBridgeAddressResponse {}
/// MsgToggleBridgeRequest is the request message for enabling or disabling the bridge for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgToggleBridgeRequest")]
pub struct MsgToggleBridgeRequest {
    /// admin is the address of the vault administrator.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to update.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// enabled indicates whether bridge operations are allowed.
    #[prost(bool, tag = "3")]
    pub enabled: bool,
}
/// MsgToggleBridgeResponse is the response message for the ToggleBridge endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgToggleBridgeResponse")]
pub struct MsgToggleBridgeResponse {}
/// MsgBridgeMintSharesRequest is the request message for minting local share marker supply; must be signed by the configured bridge address.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgBridgeMintSharesRequest")]
pub struct MsgBridgeMintSharesRequest {
    /// bridge is the signer and must match the vault's configured bridge_address.
    #[prost(string, tag = "1")]
    pub bridge: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault whose local share marker supply will be increased.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// shares is the amount of shares to mint into local marker supply.
    #[prost(message, optional, tag = "3")]
    pub shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgBridgeMintSharesResponse is the response message for the BridgeMintShares endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgBridgeMintSharesResponse")]
pub struct MsgBridgeMintSharesResponse {}
/// MsgBridgeBurnSharesRequest is the request message for burning local share marker supply; must be signed by the configured bridge address.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgBridgeBurnSharesRequest")]
pub struct MsgBridgeBurnSharesRequest {
    /// bridge is the signer and must match the vault's configured bridge_address.
    #[prost(string, tag = "1")]
    pub bridge: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault whose local share marker supply will be decreased.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// shares is the amount of shares to burn from local marker supply.
    #[prost(message, optional, tag = "3")]
    pub shares: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgBridgeBurnSharesResponse is the response message for the BridgeBurnShares endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgBridgeBurnSharesResponse")]
pub struct MsgBridgeBurnSharesResponse {}
/// MsgSetAssetManagerRequest sets or clears the optional asset manager address for a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSetAssetManagerRequest")]
pub struct MsgSetAssetManagerRequest {
    /// admin is the address of the vault administrator.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to update.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// asset_manager is the address that will be allowed to manage certain vault operations alongside the admin.
    /// Passing an empty string clears any configured asset manager.
    #[prost(string, tag = "3")]
    pub asset_manager: ::prost::alloc::string::String,
}
/// MsgSetAssetManagerResponse is the response message for the SetAssetManager endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgSetAssetManagerResponse")]
pub struct MsgSetAssetManagerResponse {}
pub struct VaultQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> VaultQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn vaults(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryVaultsResponse, cosmwasm_std::StdError> {
        QueryVaultsRequest { pagination }.query(self.querier)
    }
    pub fn vault(
        &self,
        id: ::prost::alloc::string::String,
    ) -> Result<QueryVaultResponse, cosmwasm_std::StdError> {
        QueryVaultRequest { id }.query(self.querier)
    }
    pub fn estimate_swap_in(
        &self,
        vault_address: ::prost::alloc::string::String,
        assets: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
        shares: ::prost::alloc::string::String,
        redeem_denom: ::prost::alloc::string::String,
    ) -> Result<QueryEstimateSwapOutResponse, cosmwasm_std::StdError> {
        QueryEstimateSwapOutRequest {
            vault_address,
            shares,
            redeem_denom,
        }
        .query(self.querier)
    }
    pub fn pending_swap_outs(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryPendingSwapOutsResponse, cosmwasm_std::StdError> {
        QueryPendingSwapOutsRequest { pagination }.query(self.querier)
    }
    pub fn vault_pending_swap_outs(
        &self,
        id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryVaultPendingSwapOutsResponse, cosmwasm_std::StdError> {
        QueryVaultPendingSwapOutsRequest { id, pagination }.query(self.querier)
    }
}
