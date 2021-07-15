# Changelog for provwasm

## Unreleased changes

* Add metadata module query support for scopes, sessions, and records.
* Add missing attribute value types to fix de-serialization errors.

## Releases

### [v0.14.1](https://github.com/provenance-io/provwasm/tree/v0.14.1)

* Upgrade to cosmwasm v0.14.1

### [v0.14.0](https://github.com/provenance-io/provwasm/tree/v0.14.0)

* Upgrade to cosmwasm v0.14.0
* Allow bank balances to be set in the provenance mock querier.
* Make the internal base querier available in smart contract unit tests.
* Use the correct mock contract address.

### [v0.14.0-beta1](https://github.com/provenance-io/provwasm/tree/v0.14.0-beta1)

* Migrate to cosmwasm v0.14.0-beta1.
* Cleanup API using Rust API Guidelines.
* Add contributing document.
* Expand marker integration test contract.
* Extend marker withdrawals to support any denom.

### [v0.13.3](https://github.com/provenance-io/provwasm/tree/v0.13.3)

* Expose useful types for use in smart contracts.
* Prefer using the `From` trait instead of `Into`.

### [v0.13.2](https://github.com/provenance-io/provwasm/tree/v0.13.2)

* Initial release to crates.io (support for the attribute, marker, and name Provenance Blockchain modules).
