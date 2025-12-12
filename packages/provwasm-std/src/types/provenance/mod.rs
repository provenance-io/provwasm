// Conditionally compile Provenance modules based on feature flags
#[cfg(any(feature = "provenance-all", feature = "provenance-asset"))]
pub mod asset;

#[cfg(any(feature = "provenance-all", feature = "provenance-attribute"))]
pub mod attribute;

#[cfg(any(feature = "provenance-all", feature = "provenance-exchange"))]
pub mod exchange;

#[cfg(any(feature = "provenance-all", feature = "provenance-flatfees"))]
pub mod flatfees;

#[cfg(any(feature = "provenance-all", feature = "provenance-hold"))]
pub mod hold;

#[cfg(any(feature = "provenance-all", feature = "provenance-ibchooks"))]
pub mod ibchooks;

#[cfg(any(feature = "provenance-all", feature = "provenance-ibcratelimit"))]
pub mod ibcratelimit;

#[cfg(any(feature = "provenance-all", feature = "provenance-ledger"))]
pub mod ledger;

#[cfg(any(feature = "provenance-all", feature = "provenance-marker"))]
pub mod marker;

#[cfg(any(feature = "provenance-all", feature = "provenance-metadata"))]
pub mod metadata;

#[cfg(any(feature = "provenance-all", feature = "provenance-msgfees"))]
pub mod msgfees;

#[cfg(any(feature = "provenance-all", feature = "provenance-name"))]
pub mod name;

#[cfg(any(feature = "provenance-all", feature = "provenance-oracle"))]
pub mod oracle;

#[cfg(any(feature = "provenance-all", feature = "provenance-registry"))]
pub mod registry;

#[cfg(any(feature = "provenance-all", feature = "provenance-trigger"))]
pub mod trigger;
