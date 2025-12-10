// Conditionally compile Cosmos modules based on feature flags
#[cfg(any(feature = "cosmos-all", feature = "cosmos-app"))]
pub mod app;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-auth"))]
pub mod auth;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-authz"))]
pub mod authz;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-bank"))]
pub mod bank;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-base"))]
pub mod base;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-circuit"))]
pub mod circuit;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-consensus"))]
pub mod consensus;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-crisis"))]
pub mod crisis;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-crypto"))]
pub mod crypto;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-distribution"))]
pub mod distribution;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-evidence"))]
pub mod evidence;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-feegrant"))]
pub mod feegrant;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-genutil"))]
pub mod genutil;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-gov"))]
pub mod gov;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-group"))]
pub mod group;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-ics23"))]
pub mod ics23;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-mint"))]
pub mod mint;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-msg"))]
pub mod msg;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-nft"))]
pub mod nft;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-orm"))]
pub mod orm;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-params"))]
pub mod params;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-quarantine"))]
pub mod quarantine;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-query"))]
pub mod query;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-sanction"))]
pub mod sanction;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-slashing"))]
pub mod slashing;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-staking"))]
pub mod staking;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-store"))]
pub mod store;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-tx"))]
pub mod tx;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-upgrade"))]
pub mod upgrade;

#[cfg(any(feature = "cosmos-all", feature = "cosmos-vesting"))]
pub mod vesting;
