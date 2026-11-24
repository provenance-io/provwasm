use provwasm_proc_macro::CosmwasmExt;
/// Params defines the set of module parameters.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.Params")]
pub struct Params {
    /// tech_fee_address is the address where all AUM fees are collected and which holds
    /// the authority to update per-vault bips.
    #[prost(string, tag = "1")]
    pub tech_fee_address: ::prost::alloc::string::String,
    /// default_aum_fee_bips is the default fee rate (in basis points) applied to new vaults upon creation.
    #[prost(uint32, tag = "2")]
    pub default_aum_fee_bips: u32,
    /// gov_only_vault_creation restricts CreateVault to the governance module account.
    /// When true, a vault can only come into existence through a passed governance proposal.
    /// When false (the default), any account may sign CreateVault directly, which is how
    /// development, docker, and testnet chains are expected to run.
    #[prost(bool, tag = "3")]
    pub gov_only_vault_creation: bool,
}
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
    /// redeem_denom is the denomination of the asset to be redeemed. Always the
    /// vault's underlying_asset; retained for event-schema compatibility.
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
/// EventSwapOutRetryScheduled is emitted when a pending swap out could not be
/// settled or refunded and was re-keyed to a later retry time. The shares stay
/// escrowed, so a rising failure_count signals escrow needing operator attention.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventSwapOutRetryScheduled")]
pub struct EventSwapOutRetryScheduled {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// owner is the bech32 address of the user whose shares remain escrowed.
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// shares is the amount of vault shares still escrowed by the vault.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
    /// request_id is the unique identifier of the swap out request that failed.
    #[prost(uint64, tag = "4")]
    pub request_id: u64,
    /// reason is a string detailing why the attempt failed.
    #[prost(string, tag = "5")]
    pub reason: ::prost::alloc::string::String,
    /// failure_count is the number of consecutive failed attempts, including this one.
    #[prost(uint32, tag = "6")]
    pub failure_count: u32,
    /// retry_time is the UNIX timestamp (in seconds) at which the request becomes due again.
    #[prost(int64, tag = "7")]
    pub retry_time: i64,
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
    /// reason is the reason for pausing the vault. For a manual pause (PauseVault
    /// tx) this is the user-supplied reason. For an automated auto-pause triggered
    /// in the begin/end blocker, this carries the hard-coded reason describing the
    /// critical error that forced the pause.
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// total_vault_value is the total value of the vault's assets at the time of pausing.
    #[prost(string, tag = "4")]
    pub total_vault_value: ::prost::alloc::string::String,
    /// forced indicates the pause used the force path, waiving the strict
    /// reconcile/valuation gate. False for a normal strict pause; true for a
    /// forced manual pause and for automated auto-pauses.
    #[prost(bool, tag = "5")]
    pub forced: bool,
    /// forced_error records the reconcile and/or valuation error tolerated by a
    /// manual force pause (PauseVault tx). Empty when nothing failed. When
    /// non-empty, total_vault_value may be stale or zero. Only the tx path sets
    /// this; automated auto-pauses leave it empty and instead encode their error
    /// in reason.
    #[prost(string, tag = "6")]
    pub forced_error: ::prost::alloc::string::String,
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
/// EventWithdrawalDelayUpdated is an event emitted when a vault's withdrawal delay is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventWithdrawalDelayUpdated")]
pub struct EventWithdrawalDelayUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that updated the withdrawal delay.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// withdrawal_delay_seconds is the newly set withdrawal delay in seconds.
    #[prost(uint64, tag = "3")]
    pub withdrawal_delay_seconds: u64,
}
/// EventVaultFeeCollected is an event emitted when a vault's AUM fee is collected.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultFeeCollected")]
pub struct EventVaultFeeCollected {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// collected_amount is the amount of the technology fee that was actually collected (e.g., "100nhash").
    #[prost(string, tag = "2")]
    pub collected_amount: ::prost::alloc::string::String,
    /// requested_amount is the amount of the technology fee that was calculated for the period (e.g., "120nhash").
    #[prost(string, tag = "3")]
    pub requested_amount: ::prost::alloc::string::String,
    /// aum_snapshot is the total vault value at the time of fee calculation (e.g., "1000000underlying").
    #[prost(string, tag = "4")]
    pub aum_snapshot: ::prost::alloc::string::String,
    /// duration_seconds is the duration for which the fee was calculated.
    #[prost(int64, tag = "5")]
    pub duration_seconds: i64,
    /// outstanding_amount is the amount of AUM fee that remains unpaid after this collection (e.g., "20nhash").
    #[prost(string, tag = "6")]
    pub outstanding_amount: ::prost::alloc::string::String,
}
/// EventParamsUpdated is an event emitted when the module parameters are updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventParamsUpdated")]
pub struct EventParamsUpdated {
    /// params are the newly updated module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// EventVaultAUMFeeBipsUpdated is an event emitted when a vault's AUM fee bips are updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventVaultAUMFeeBipsUpdated")]
pub struct EventVaultAumFeeBipsUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (tech fee address) that updated the fee bips.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// aum_fee_bips is the newly set AUM fee bips for the vault.
    #[prost(uint32, tag = "3")]
    pub aum_fee_bips: u32,
}
/// EventMinSwapInValueUpdated is an event emitted when a vault's minimum swap-in value is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventMinSwapInValueUpdated")]
pub struct EventMinSwapInValueUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that updated the limit.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// min_swap_in is the newly set minimum swap-in value, measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - An empty string "" or "0" indicates the minimum was cleared / no configured minimum.
    #[prost(string, tag = "3")]
    pub min_swap_in: ::prost::alloc::string::String,
}
/// EventMinSwapOutValueUpdated is an event emitted when a vault's minimum swap-out value is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventMinSwapOutValueUpdated")]
pub struct EventMinSwapOutValueUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that updated the limit.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// min_swap_out is the newly set minimum swap-out value, measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - An empty string "" or "0" indicates the minimum was cleared / no configured minimum.
    #[prost(string, tag = "3")]
    pub min_swap_out: ::prost::alloc::string::String,
}
/// EventMaxSwapInValueUpdated is an event emitted when a vault's maximum swap-in value is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventMaxSwapInValueUpdated")]
pub struct EventMaxSwapInValueUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that updated the limit.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// max_swap_in is the newly set maximum swap-in value, measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" indicates no maximum limit.
    #[prost(string, tag = "3")]
    pub max_swap_in: ::prost::alloc::string::String,
}
/// EventMaxSwapOutValueUpdated is an event emitted when a vault's maximum swap-out value is updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventMaxSwapOutValueUpdated")]
pub struct EventMaxSwapOutValueUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// authority is the address (admin or asset manager) that updated the limit.
    #[prost(string, tag = "2")]
    pub authority: ::prost::alloc::string::String,
    /// max_swap_out is the newly set maximum swap-out value, measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" indicates no maximum limit.
    #[prost(string, tag = "3")]
    pub max_swap_out: ::prost::alloc::string::String,
}
/// EventNAVUpdated is emitted when a vault's internal NAV entry for a denom is created or updated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventNAVUpdated")]
pub struct EventNavUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// denom is the asset denomination whose NAV entry was updated.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// price is the total value of volume units of the denom, denominated in the vault's underlying asset.
    #[prost(string, tag = "3")]
    pub price: ::prost::alloc::string::String,
    /// volume is the number of units of the denom that price covers.
    #[prost(string, tag = "4")]
    pub volume: ::prost::alloc::string::String,
    /// source identifies the origin of this NAV update.
    #[prost(string, tag = "5")]
    pub source: ::prost::alloc::string::String,
    /// signer is the NAV authority address that performed the update.
    #[prost(string, tag = "6")]
    pub signer: ::prost::alloc::string::String,
    /// updated_block_height is the block height at which the NAV entry was updated.
    #[prost(int64, tag = "7")]
    pub updated_block_height: i64,
}
/// EventNAVRemoved is emitted when a vault's internal NAV entry for a denom is removed,
/// such as when an outbound settlement leaves the vault holding zero of the denom.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventNAVRemoved")]
pub struct EventNavRemoved {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// denom is the asset denomination whose NAV entry was removed.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    /// last_price is the total value of last_volume units of the denom recorded
    /// before removal, rendered as a coin string.
    #[prost(string, tag = "3")]
    pub last_price: ::prost::alloc::string::String,
    /// last_volume is the number of units of the denom that last_price covered.
    #[prost(string, tag = "4")]
    pub last_volume: ::prost::alloc::string::String,
    /// signer is the NAV authority address that removed the entry. It is empty when
    /// the protocol removed the entry itself, such as an outbound settlement draining
    /// the denom.
    #[prost(string, tag = "5")]
    pub signer: ::prost::alloc::string::String,
}
/// EventNAVAuthorityUpdated is emitted when a vault's NAV authority is rotated.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventNAVAuthorityUpdated")]
pub struct EventNavAuthorityUpdated {
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// admin is the address of the vault administrator that performed the rotation.
    #[prost(string, tag = "2")]
    pub admin: ::prost::alloc::string::String,
    /// new_authority is the address that is now authorized to mutate the vault's internal NAV table.
    #[prost(string, tag = "3")]
    pub new_authority: ::prost::alloc::string::String,
}
/// EventAssetAccepted is emitted when the asset manager settles a pending
/// exchange-module payment whose target is the vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventAssetAccepted")]
pub struct EventAssetAccepted {
    /// vault_address is the bech32 address of the vault that settled the payment.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// source is the bech32 address of the account that created the settled payment.
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// external_id, together with source, identifies the settled payment.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
    /// source_amount is the funds the source paid the vault, rendered as a coins string.
    #[prost(string, tag = "4")]
    pub source_amount: ::prost::alloc::string::String,
    /// target_amount is the funds the vault paid the source, rendered as a coins string.
    #[prost(string, tag = "5")]
    pub target_amount: ::prost::alloc::string::String,
    /// direction indicates whether the asset moved into the vault ("inbound") or
    /// out of the vault ("outbound") relative to the vault's underlying asset.
    #[prost(string, tag = "6")]
    pub direction: ::prost::alloc::string::String,
}
/// EventAssetRejected is emitted when the asset manager declines a pending
/// exchange-module payment whose target is the vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.EventAssetRejected")]
pub struct EventAssetRejected {
    /// vault_address is the bech32 address of the vault that rejected the payment.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// source is the bech32 address of the account that created the rejected payment.
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// external_id, together with source, identifies the rejected payment.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
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
    /// - Interest accrual, fees, and internal accounting are measured in this denom.
    /// - Held assets acquired via settlement must have an internal NAV record priced INTO this denom.
    #[prost(string, tag = "3")]
    pub underlying_asset: ::prost::alloc::string::String,
    /// payment_denom always equals underlying_asset. Vaults are single-denom; the
    /// field is inert and retained only for wire compatibility.
    ///
    /// Deprecated: The mixed-denom vault concept has been flattened into a single
    /// underlying denom. Deletion of this field is deferred to a future major release.
    #[deprecated]
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
    /// Nothing done during the pause moves it, neither a principal deposit or withdrawal
    /// nor a NAV repricing; those all take effect together when the vault is unpaused.
    #[prost(message, optional, tag = "16")]
    pub paused_balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// paused_reason is a human-readable string explaining why the vault was paused, particularly for automatic pauses.
    #[prost(string, tag = "17")]
    pub paused_reason: ::prost::alloc::string::String,
    /// paused_by is the address that initiated the current pause, empty for an automatic
    /// pause and cleared on unpause. MsgRepriceVault consults it so the NAV authority
    /// can only resume a pause it took itself.
    #[prost(string, tag = "30")]
    pub paused_by: ::prost::alloc::string::String,
    /// paused_forced reports whether the current pause waived the strict reconcile and
    /// valuation gate, true for a forced MsgPauseVault and every automatic pause. A forced
    /// pause records a tolerated failure, so MsgRepriceVault refuses to resume one.
    #[prost(bool, tag = "31")]
    pub paused_forced: bool,
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
    /// fee_period_start is the start time (in Unix seconds) of the current AUM fee collection period.
    /// This is a module-managed timestamp kept in sync with the fee timeout queue machinery.
    #[prost(int64, tag = "21")]
    pub fee_period_start: i64,
    /// fee_period_timeout is the end time (in Unix seconds) of the current AUM fee collection period.
    /// This is a module-managed timestamp kept in sync with the fee timeout queue machinery.
    #[prost(int64, tag = "22")]
    pub fee_period_timeout: i64,
    /// outstanding_aum_fee is the amount of AUM fee that has been calculated but not yet collected
    /// due to insufficient liquidity in the principal marker. This amount is always denominated
    /// in the vault's underlying_asset, must be preserved, and is carried into valuation computations.
    #[prost(message, optional, tag = "23")]
    pub outstanding_aum_fee:
        ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// aum_fee_bips is the AUM fee rate (in basis points) for this specific vault.
    /// Units: Basis Points (1 bps = 0.01%).
    /// Valid Range: 0 to 10000 (0% to 100%).
    /// A value of 0 disables AUM fee collection for this vault.
    /// Negative values are not supported (uint32).
    #[prost(uint32, tag = "24")]
    pub aum_fee_bips: u32,
    /// min_swap_in_value is a string representing the minimum value required for a deposit.
    /// - The value is measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - If empty ("") or "0", there is no minimum limit.
    #[prost(string, tag = "25")]
    pub min_swap_in_value: ::prost::alloc::string::String,
    /// min_swap_out_value is a string representing the minimum value required for a withdrawal.
    /// - The value is measured in the underlying_asset.
    /// - Outgoing redemptions are converted to this unit before checking.
    /// - Values must be non-negative (>= 0).
    /// - If empty ("") or "0", there is no minimum limit.
    #[prost(string, tag = "26")]
    pub min_swap_out_value: ::prost::alloc::string::String,
    /// max_swap_in_value is a string representing the maximum value allowed for a deposit.
    /// - The value is measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" indicates no maximum limit.
    #[prost(string, tag = "27")]
    pub max_swap_in_value: ::prost::alloc::string::String,
    /// max_swap_out_value is a string representing the maximum value allowed for a withdrawal.
    /// - The value is measured in the underlying_asset.
    /// - Outgoing redemptions are converted to this unit before checking.
    /// - Values must be positive (> 0).
    /// - An empty string "" indicates no maximum limit.
    #[prost(string, tag = "28")]
    pub max_swap_out_value: ::prost::alloc::string::String,
    /// nav_authority is the address authorized to mutate this vault's internal NAV
    /// table via MsgUpdateVaultNAV. If empty, the vault admin is treated as the NAV
    /// authority. It is rotated only via MsgUpdateNAVAuthority.
    #[prost(string, tag = "29")]
    pub nav_authority: ::prost::alloc::string::String,
}
/// VaultNAV is a single internal net asset value entry recording the price of one
/// asset denom held by a vault. The vault module is the sole source of truth for
/// these values; the NAV authority maintains them via MsgUpdateVaultNAV.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.VaultNAV")]
pub struct VaultNav {
    /// denom is the asset denomination this entry prices.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// price is the total value of `volume` units of the denom, denominated in the
    /// vault's underlying asset. The per-unit value is price divided by volume.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// volume is the number of units of the denom that price covers. It must be
    /// positive; the per-unit value of the denom is price divided by volume.
    #[prost(string, tag = "3")]
    pub volume: ::prost::alloc::string::String,
    /// source identifies the origin of this NAV entry (for example an oracle name
    /// or the settlement path), mirroring the Marker module's NAV source attribution.
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
    /// updated_block_height is the block height at which this entry was last updated.
    #[prost(int64, tag = "5")]
    pub updated_block_height: i64,
    /// updated_time is the block time at which this entry was last updated.
    #[prost(message, optional, tag = "6")]
    pub updated_time: ::core::option::Option<crate::shim::Timestamp>,
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
    /// redeem_denom is the denomination of the asset to be redeemed. Always the
    /// vault's underlying_asset; retained for wire compatibility.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub redeem_denom: ::prost::alloc::string::String,
    /// failure_count is the number of consecutive failed processing attempts that
    /// left this request queued. Each failure re-keys the request to a later retry
    /// time so it cannot hold the front of the queue. Reset when expedited.
    #[prost(uint32, tag = "5")]
    pub failure_count: u32,
}
/// Payment is the vault module's view of a Provenance exchange-module payment. It
/// mirrors the exchange Payment, exposing only the fields relevant to the vault's
/// asset settlement workflow.
///
/// It serves two roles. As a query result it reports a pending payment targeting the
/// vault. In MsgAcceptAssetRequest it carries the terms the asset manager reviewed, and
/// settlement proceeds only if they still match the stored payment exactly.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.Payment")]
pub struct Payment {
    /// source is the account that created the payment and owns the escrowed source_amount.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// source_amount is the funds the source pays the target. A hold is placed on this
    /// amount in the source account until the payment is accepted, rejected, or cancelled.
    #[prost(message, repeated, tag = "2")]
    pub source_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// target is the account that can accept the payment; for the vault's settlement
    /// workflow it is the vault.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// target_amount is the funds the target pays the source in exchange for source_amount.
    #[prost(message, repeated, tag = "4")]
    pub target_amount: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// external_id, together with source, uniquely identifies the payment.
    #[prost(string, tag = "5")]
    pub external_id: ::prost::alloc::string::String,
}
/// QueueEntry is a (time, addr) pair used by various vault timeout queues
/// (e.g., payout deferral and fee collection).
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
/// VaultNAVEntry pairs a vault address with one of its internal NAV records for
/// genesis import and export.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.VaultNAVEntry")]
pub struct VaultNavEntry {
    /// vault_address is the bech32 address of the vault that owns this NAV record.
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    /// nav is the internal NAV record held by the vault.
    #[prost(message, optional, tag = "2")]
    pub nav: ::core::option::Option<VaultNav>,
}
/// GenesisState defines the vault module's genesis state.
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
    /// fee_timeout_queue contains (time, addr) entries for vaults that are
    /// temporarily deferred from automatic AUM fee collection until the
    /// given UNIX timestamp (seconds). These entries are re-enqueued on InitGenesis.
    #[prost(message, repeated, tag = "4")]
    pub fee_timeout_queue: ::prost::alloc::vec::Vec<QueueEntry>,
    /// params defines the module parameters.
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<Params>,
    /// navs contains the internal NAV table entries for all vaults at genesis.
    #[prost(message, repeated, tag = "6")]
    pub navs: ::prost::alloc::vec::Vec<VaultNavEntry>,
    /// payout_verification_set contains bech32 addresses of vaults awaiting their next interest affordability check.
    #[prost(string, repeated, tag = "7")]
    pub payout_verification_set: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    /// assets is the amount of the underlying asset to swap in.
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
    /// shares is the amount of shares to swap out, as a non-negative integer of at most 80 characters.
    #[prost(string, tag = "2")]
    pub shares: ::prost::alloc::string::String,
    /// redeem_denom previously selected the payout denom to estimate. The estimate
    /// is always in the vault's underlying_asset; if set, this must equal
    /// underlying_asset. Retained for wire compatibility with released clients.
    #[deprecated]
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
/// QueryParamsRequest is the request message for the Query/Params endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryParamsRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response message for the Query/Params endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params defines the vault module parameters.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryVaultNavsRequest is the request message for the Query/VaultNavs endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultNavsRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/VaultNavs",
    response_type = QueryVaultNavsResponse
)]
pub struct QueryVaultNavsRequest {
    /// id is the bech32 address of the vault or the vault's share denom to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVaultNavsResponse is the response message for the Query/VaultNavs endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultNavsResponse")]
pub struct QueryVaultNavsResponse {
    /// navs is the list of internal NAV entries held by the vault.
    #[prost(message, repeated, tag = "1")]
    pub navs: ::prost::alloc::vec::Vec<VaultNav>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryNavValueRequest is the request message for the Query/NavValue endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryNavValueRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/NavValue",
    response_type = QueryNavValueResponse
)]
pub struct QueryNavValueRequest {
    /// id is the bech32 address of the vault or the vault's share denom to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// denom is the asset denomination whose NAV entry is being queried.
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryNavValueResponse is the response message for the Query/NavValue endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryNavValueResponse")]
pub struct QueryNavValueResponse {
    /// nav is the internal NAV entry for the requested vault and denom.
    #[prost(message, optional, tag = "1")]
    pub nav: ::core::option::Option<VaultNav>,
}
/// QueryVaultPaymentRequest is the request message for the Query/VaultPayment endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultPaymentRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/VaultPayment",
    response_type = QueryVaultPaymentResponse
)]
pub struct QueryVaultPaymentRequest {
    /// id is the bech32 address of the vault or the vault's share denom to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// source is the bech32 address of the account that created the payment.
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
    /// external_id, together with source, uniquely identifies the payment.
    #[prost(string, tag = "3")]
    pub external_id: ::prost::alloc::string::String,
}
/// QueryVaultPaymentResponse is the response message for the Query/VaultPayment endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultPaymentResponse")]
pub struct QueryVaultPaymentResponse {
    /// payment is the pending exchange-module payment targeting the vault.
    #[prost(message, optional, tag = "1")]
    pub payment: ::core::option::Option<Payment>,
}
/// QueryVaultPaymentsRequest is the request message for the Query/VaultPayments endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultPaymentsRequest")]
#[proto_query(
    path = "/provlabs.vault.v1.Query/VaultPayments",
    response_type = QueryVaultPaymentsResponse
)]
pub struct QueryVaultPaymentsRequest {
    /// id is the bech32 address of the vault or the vault's share denom to query.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryVaultPaymentsResponse is the response message for the Query/VaultPayments endpoint.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.QueryVaultPaymentsResponse")]
pub struct QueryVaultPaymentsResponse {
    /// payments is the list of pending exchange-module payments targeting the vault.
    #[prost(message, repeated, tag = "1")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// MsgCreateVaultRequest is the request message for the CreateVault endpoint.
/// Who may sign depends on the module's gov_only_vault_creation param: when it is
/// enabled only the governance module account may sign, so a vault can only come
/// into existence through a passed proposal; when it is disabled any account may
/// sign and create a vault directly.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgCreateVaultRequest")]
pub struct MsgCreateVaultRequest {
    /// admin is the initial administrator of the vault. It is designated by the
    /// signer and is not required to be the signer itself.
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
    /// share_denom is the name of the assets created by the vault used for distribution.
    #[prost(string, tag = "2")]
    pub share_denom: ::prost::alloc::string::String,
    /// underlying_asset is the denomination of the asset supported by the vault.
    #[prost(string, tag = "3")]
    pub underlying_asset: ::prost::alloc::string::String,
    /// payment_denom previously configured a secondary accepted denomination.
    /// Vaults are single-denom on underlying_asset; if set, this must equal
    /// underlying_asset. Retained for wire compatibility with released clients.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub payment_denom: ::prost::alloc::string::String,
    /// withdrawal_delay_seconds is the time period (in seconds) that a withdrawal
    /// must wait in the pending queue before being processed.
    #[prost(uint64, tag = "5")]
    pub withdrawal_delay_seconds: u64,
    /// min_swap_in_value is the minimum value required for a deposit, measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - An empty string "" or "0" indicates no minimum.
    #[prost(string, tag = "6")]
    pub min_swap_in_value: ::prost::alloc::string::String,
    /// min_swap_out_value is the minimum value required for a withdrawal, measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - An empty string "" or "0" indicates no minimum.
    #[prost(string, tag = "7")]
    pub min_swap_out_value: ::prost::alloc::string::String,
    /// max_swap_in_value is the maximum value allowed for a deposit, measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" clears the maximum / represents no maximum.
    #[prost(string, tag = "8")]
    pub max_swap_in_value: ::prost::alloc::string::String,
    /// max_swap_out_value is the maximum value allowed for a withdrawal, measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" indicates no maximum limit.
    #[prost(string, tag = "9")]
    pub max_swap_out_value: ::prost::alloc::string::String,
    /// authority is the address signing the message. It must be the governance module
    /// account while the gov_only_vault_creation param is enabled; otherwise it may be
    /// any account.
    #[prost(string, tag = "10")]
    pub authority: ::prost::alloc::string::String,
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
    /// redeem_denom previously selected the payout coin. The payout is always the
    /// vault's underlying_asset; if set, this must equal underlying_asset.
    /// Retained for wire compatibility with released clients.
    #[deprecated]
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
/// MsgUpdateWithdrawalDelayRequest is the request message for updating the withdrawal delay of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateWithdrawalDelayRequest")]
pub struct MsgUpdateWithdrawalDelayRequest {
    /// authority is the address of the vault admin or asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// withdrawal_delay_seconds is the time period (in seconds) that a withdrawal
    /// must wait in the pending queue before being processed.
    #[prost(uint64, tag = "3")]
    pub withdrawal_delay_seconds: u64,
}
/// MsgUpdateWithdrawalDelayResponse is the response message for the UpdateWithdrawalDelay endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateWithdrawalDelayResponse")]
pub struct MsgUpdateWithdrawalDelayResponse {}
/// MsgUpdateMinSwapInValueRequest is the request message for updating the minimum swap-in value of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMinSwapInValueRequest")]
pub struct MsgUpdateMinSwapInValueRequest {
    /// authority is the bech32 address of the vault administrator or asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// min_swap_in_value is the minimum value required for a deposit, measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - An empty string "" or "0" clears the minimum / represents no minimum.
    #[prost(string, tag = "3")]
    pub min_swap_in_value: ::prost::alloc::string::String,
}
/// MsgUpdateMinSwapInValueResponse is the response message for the UpdateMinSwapInValue endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMinSwapInValueResponse")]
pub struct MsgUpdateMinSwapInValueResponse {}
/// MsgUpdateMinSwapOutValueRequest is the request message for updating the minimum swap-out value of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMinSwapOutValueRequest")]
pub struct MsgUpdateMinSwapOutValueRequest {
    /// authority is the bech32 address of the vault administrator or asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// min_swap_out_value is the minimum value required for a withdrawal, measured in the underlying_asset.
    /// - Values must be non-negative (>= 0).
    /// - An empty string "" or "0" clears the minimum / represents no minimum.
    #[prost(string, tag = "3")]
    pub min_swap_out_value: ::prost::alloc::string::String,
}
/// MsgUpdateMinSwapOutValueResponse is the response message for the UpdateMinSwapOutValue endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMinSwapOutValueResponse")]
pub struct MsgUpdateMinSwapOutValueResponse {}
/// MsgUpdateMaxSwapInValueRequest is the request message for updating the maximum swap-in value of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMaxSwapInValueRequest")]
pub struct MsgUpdateMaxSwapInValueRequest {
    /// authority is the bech32 address of the vault administrator or asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// max_swap_in_value is the maximum value allowed for a deposit, measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" clears the maximum / represents no maximum.
    #[prost(string, tag = "3")]
    pub max_swap_in_value: ::prost::alloc::string::String,
}
/// MsgUpdateMaxSwapInValueResponse is the response message for the UpdateMaxSwapInValue endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMaxSwapInValueResponse")]
pub struct MsgUpdateMaxSwapInValueResponse {}
/// MsgUpdateMaxSwapOutValueRequest is the request message for updating the maximum swap-out value of a vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMaxSwapOutValueRequest")]
pub struct MsgUpdateMaxSwapOutValueRequest {
    /// authority is the bech32 address of the vault administrator or asset manager.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// max_swap_out_value is the maximum value allowed for a withdrawal, measured in the underlying_asset.
    /// - Values must be positive (> 0).
    /// - An empty string "" clears the maximum / represents no maximum.
    #[prost(string, tag = "3")]
    pub max_swap_out_value: ::prost::alloc::string::String,
}
/// MsgUpdateMaxSwapOutValueResponse is the response message for the UpdateMaxSwapOutValue endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateMaxSwapOutValueResponse")]
pub struct MsgUpdateMaxSwapOutValueResponse {}
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
    /// authority is the address initiating the pause: the vault administrator, the asset
    /// manager, or the NAV authority. The NAV authority gets no matching unpause; it can
    /// only resume its own pause via RepriceVault.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to pause.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// reason is a human-readable explanation for pausing the vault. This is recorded
    /// for operators and clients to understand the context (e.g., maintenance or anomaly).
    #[prost(string, tag = "3")]
    pub reason: ::prost::alloc::string::String,
    /// force, when true, allows the pause to proceed even if the pre-pause
    /// reconcile or vault valuation fails. The pause is applied best-effort:
    /// tolerated failures are logged and surfaced on EventVaultPaused, and
    /// PausedBalance may be the net TVV, zero (when valuation itself failed), or
    /// otherwise approximate. When false (default), any reconcile or valuation
    /// failure aborts the pause and the vault remains unpaused.
    #[prost(bool, tag = "4")]
    pub force: bool,
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
///
/// Rotation leaves any share balance on the outgoing bridge in place, reducing mint capacity by that amount
/// until it is transferred to the new bridge and burned. Drain the outgoing bridge before rotating.
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
/// MsgUpdateParamsRequest is the request message for updating the module parameters.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateParamsRequest")]
pub struct MsgUpdateParamsRequest {
    /// authority is the address of the governance module account.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the vault module parameters to update.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse is the response message for the UpdateParams endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgUpdateVaultAUMFeeBipsRequest is the request message for updating the AUM fee bips for a specific vault.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateVaultAUMFeeBipsRequest")]
pub struct MsgUpdateVaultAumFeeBipsRequest {
    /// authority is the tech fee address authorized to update per-vault bips.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault to update.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// aum_fee_bips is the new fee rate (in basis points) for the vault.
    /// The value must be between 0 and 10,000 (inclusive), where 10,000 represents 100%.
    #[prost(uint32, tag = "3")]
    pub aum_fee_bips: u32,
}
/// MsgUpdateVaultAUMFeeBipsResponse is the response message for the UpdateVaultAUMFeeBips endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateVaultAUMFeeBipsResponse")]
pub struct MsgUpdateVaultAumFeeBipsResponse {}
/// MsgUpdateVaultNAVRequest is the request message for creating or updating a vault's internal NAV entry.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateVaultNAVRequest")]
pub struct MsgUpdateVaultNavRequest {
    /// signer is the address of the vault's NAV authority authorizing this update.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault whose NAV entry is being updated.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// denom is the asset denomination being priced. It must not be the vault's
    /// share denom.
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    /// price is the total value of `volume` units of the denom, denominated in
    /// the vault's underlying asset.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// volume is the number of units of the denom that price covers. It must be positive.
    #[prost(string, tag = "5")]
    pub volume: ::prost::alloc::string::String,
    /// source identifies the origin of this NAV update (for example an oracle name).
    /// It is optional.
    #[prost(string, tag = "6")]
    pub source: ::prost::alloc::string::String,
}
/// MsgUpdateVaultNAVResponse is the response message for the UpdateVaultNAV endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateVaultNAVResponse")]
pub struct MsgUpdateVaultNavResponse {}
/// NAVUpdate is one denom's price restatement within a RepriceVault batch. It carries
/// only the operator-supplied fields of a VaultNAV; the module stamps the update height
/// and time itself.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.NAVUpdate")]
pub struct NavUpdate {
    /// denom is the asset denomination being priced. It must not be the vault's share denom.
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// price is the total value of `volume` units of the denom, denominated in
    /// the vault's underlying asset.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// volume is the number of units of the denom that price covers. It must be positive.
    #[prost(string, tag = "3")]
    pub volume: ::prost::alloc::string::String,
    /// source identifies the origin of this NAV update (for example an oracle name).
    /// It is optional.
    #[prost(string, tag = "4")]
    pub source: ::prost::alloc::string::String,
}
/// MsgRepriceVaultRequest applies a batch of internal NAV updates and, when resume is set,
/// unpauses the vault in the same state transition. It is the batched form of
/// MsgUpdateVaultNAV, carrying the identical per-entry rules, plus an optional resume.
///
/// The updates are a batch because a vault holding many priced positions, such as a book of
/// loans, restates them on one cadence. A book too large for one transaction is repriced by
/// sending several of these with resume unset and a final one with resume set, so the vault
/// stays frozen for the whole restatement and reopens once, on the last message.
///
/// With resume set, the whole batch and the unpause commit together, so there is never a
/// block in which the vault is live, a new price is public, and the share price step has not
/// yet landed. It resumes only a strict pause the current NAV authority took itself;
/// operator, forced, and automatic pauses still require a management unpause.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgRepriceVaultRequest")]
pub struct MsgRepriceVaultRequest {
    /// signer is the address of the vault's NAV authority authorizing these updates.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault being repriced.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// navs are the price restatements to apply. At least one is required, each denom may
    /// appear only once, and the batch is capped at MaxRepriceBatchSize.
    #[prost(message, repeated, tag = "3")]
    pub navs: ::prost::alloc::vec::Vec<NavUpdate>,
    /// resume, when true, unpauses the vault after applying the batch. Leave it false to
    /// apply a batch and keep the vault frozen, which is how a restatement too large for one
    /// transaction is continued across several. Setting it requires the vault to be under a
    /// strict pause this same NAV authority took.
    #[prost(bool, tag = "4")]
    pub resume: bool,
}
/// MsgRepriceVaultResponse is the response message for the RepriceVault endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgRepriceVaultResponse")]
pub struct MsgRepriceVaultResponse {}
/// MsgRemoveVaultNAVRequest is the request message for deleting a vault's internal NAV
/// entry for a denom. Only entries for denoms the vault does not hold may be removed; a
/// held asset is written down through UpdateVaultNAV instead, so that its value stays
/// accounted for.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgRemoveVaultNAVRequest")]
pub struct MsgRemoveVaultNavRequest {
    /// signer is the address of the vault's NAV authority authorizing this removal.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault whose NAV entry is being removed.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// denom is the asset denomination whose NAV entry is being removed.
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
}
/// MsgRemoveVaultNAVResponse is the response message for the RemoveVaultNAV endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgRemoveVaultNAVResponse")]
pub struct MsgRemoveVaultNavResponse {}
/// MsgUpdateNAVAuthorityRequest is the request message for rotating a vault's NAV authority.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateNAVAuthorityRequest")]
pub struct MsgUpdateNavAuthorityRequest {
    /// signer is the address of the vault administrator authorizing this rotation.
    #[prost(string, tag = "1")]
    pub signer: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault whose NAV authority is being rotated.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// new_authority is the address that will be authorized to mutate the vault's internal NAV table.
    #[prost(string, tag = "3")]
    pub new_authority: ::prost::alloc::string::String,
}
/// MsgUpdateNAVAuthorityResponse is the response message for the UpdateNAVAuthority endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgUpdateNAVAuthorityResponse")]
pub struct MsgUpdateNavAuthorityResponse {}
/// MsgAcceptAssetRequest is the request message for settling a pending exchange-module
/// payment whose target is the vault. The vault settles only payments where one leg is the
/// vault's underlying asset; the settlement direction (inbound or outbound) is derived from
/// which leg that is.
///
/// The message carries the complete payment, so the asset manager's signature commits to the
/// economic terms of the deal. Settlement requires the payment held by the exchange module to
/// match those terms exactly.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgAcceptAssetRequest")]
pub struct MsgAcceptAssetRequest {
    /// authority is the address of the vault's asset manager authorizing the settlement.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault, which must be the payment's target.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// payment is the full set of terms the asset manager reviewed and is approving. It
    /// identifies the pending payment by its source and external_id, and binds the approval
    /// to that payment's exact legs. Settlement fails if any field does not match the
    /// payment held by the exchange module.
    #[prost(message, optional, tag = "5")]
    pub payment: ::core::option::Option<Payment>,
}
/// MsgAcceptAssetResponse is the response message for the AcceptAsset endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgAcceptAssetResponse")]
pub struct MsgAcceptAssetResponse {}
/// MsgRejectAssetRequest is the request message for declining a pending exchange-module
/// payment whose target is the vault. The exchange module cancels the payment and refunds
/// the source's escrow. The payment is identified by its source account and external_id.
#[derive(Clone, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgRejectAssetRequest")]
pub struct MsgRejectAssetRequest {
    /// authority is the address of the vault's asset manager authorizing the rejection.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault_address is the bech32 address of the vault, which must be the payment's target.
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    /// source is the bech32 address of the account that created the pending payment.
    #[prost(string, tag = "3")]
    pub source: ::prost::alloc::string::String,
    /// external_id, together with source, uniquely identifies the pending payment to reject.
    #[prost(string, tag = "4")]
    pub external_id: ::prost::alloc::string::String,
}
/// MsgRejectAssetResponse is the response message for the RejectAsset endpoint.
#[derive(Clone, Copy, PartialEq, Eq, ::prost::Message, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/provlabs.vault.v1.MsgRejectAssetResponse")]
pub struct MsgRejectAssetResponse {}
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
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn vault_navs(
        &self,
        id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryVaultNavsResponse, cosmwasm_std::StdError> {
        QueryVaultNavsRequest { id, pagination }.query(self.querier)
    }
    pub fn nav_value(
        &self,
        id: ::prost::alloc::string::String,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryNavValueResponse, cosmwasm_std::StdError> {
        QueryNavValueRequest { id, denom }.query(self.querier)
    }
    pub fn vault_payment(
        &self,
        id: ::prost::alloc::string::String,
        source: ::prost::alloc::string::String,
        external_id: ::prost::alloc::string::String,
    ) -> Result<QueryVaultPaymentResponse, cosmwasm_std::StdError> {
        QueryVaultPaymentRequest {
            id,
            source,
            external_id,
        }
        .query(self.querier)
    }
    pub fn vault_payments(
        &self,
        id: ::prost::alloc::string::String,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryVaultPaymentsResponse, cosmwasm_std::StdError> {
        QueryVaultPaymentsRequest { id, pagination }.query(self.querier)
    }
}
