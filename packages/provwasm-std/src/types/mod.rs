// Conditionally compile modules based on feature flags
#[cfg(feature = "capability")]
pub mod capability;

#[cfg(any(
    feature = "cosmos-all",
    feature = "cosmos-app",
    feature = "cosmos-auth",
    feature = "cosmos-authz",
    feature = "cosmos-bank",
    feature = "cosmos-base",
    feature = "cosmos-circuit",
    feature = "cosmos-consensus",
    feature = "cosmos-crisis",
    feature = "cosmos-crypto",
    feature = "cosmos-distribution",
    feature = "cosmos-evidence",
    feature = "cosmos-feegrant",
    feature = "cosmos-genutil",
    feature = "cosmos-gov",
    feature = "cosmos-group",
    feature = "cosmos-ics23",
    feature = "cosmos-mint",
    feature = "cosmos-msg",
    feature = "cosmos-nft",
    feature = "cosmos-orm",
    feature = "cosmos-params",
    feature = "cosmos-quarantine",
    feature = "cosmos-query",
    feature = "cosmos-sanction",
    feature = "cosmos-slashing",
    feature = "cosmos-staking",
    feature = "cosmos-store",
    feature = "cosmos-tx",
    feature = "cosmos-upgrade",
    feature = "cosmos-vesting"
))]
pub mod cosmos;

#[cfg(feature = "cosmwasm-all")]
pub mod cosmwasm;

#[cfg(any(
    feature = "ibc-all",
    feature = "ibc-core",
    feature = "ibc-applications",
    feature = "ibc-lightclients"
))]
pub mod ibc;

#[cfg(any(
    feature = "provenance-all",
    feature = "provenance-attribute",
    feature = "provenance-marker",
    feature = "provenance-name",
    feature = "provenance-metadata",
    feature = "provenance-msgfees",
    feature = "provenance-trigger",
    feature = "provenance-asset",
    feature = "provenance-exchange",
    feature = "provenance-hold",
    feature = "provenance-oracle",
    feature = "provenance-registry",
    feature = "provenance-ledger",
    feature = "provenance-ibchooks",
    feature = "provenance-ibcratelimit",
    feature = "provenance-flatfees"
))]
pub mod provenance;

#[cfg(feature = "provlabs-all")]
pub mod provlabs;

#[cfg(feature = "tendermint-all")]
pub mod tendermint;
