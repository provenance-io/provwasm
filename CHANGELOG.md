# Changelog for provwasm

## Unreleased changes

None

## Releases

### [v1.1.0](https://github.com/provenance-io/provwasm/tree/v1.1.0)

* Smart Contract MsgFee Support [#87](https://github.com/provenance-io/provwasm/issues/87)
* Add `UpdateAttribute` and `DeleteDistinctAttribute` bindings [#86](https://github.com/provenance-io/provwasm/issues/86)

### [v1.0.0](https://github.com/provenance-io/provwasm/tree/v1.0.0)

* Upgrade to CosmWasm v1.0.0 [#88](https://github.com/provenance-io/provwasm/issues/88)

### [v1.0.0-rc.0](https://github.com/provenance-io/provwasm/tree/v1.0.0-rc.0)

* Upgrade to CosmWasm v1.0.0-rc.0 [#83](https://github.com/provenance-io/provwasm/issues/83)

### [v1.0.0-beta3](https://github.com/provenance-io/provwasm/tree/v1.0.0-beta3)

* Update example tutorial to work with the latest version of Provenance
* expose MetadataMsgParams to library consumers
* Add example scope create/update message to metadata contract README

### [v1.0.0-beta2](https://github.com/provenance-io/provwasm/tree/v1.0.0-beta2)

* Add missing CustomMsg impl for ProvenanceMsg

### [v1.0.0-beta](https://github.com/provenance-io/provwasm/tree/v1.0.0-beta)

* Upgrade to cosmwasm v1.0.0-beta5

### [v0.16.0](https://github.com/provenance-io/provwasm/tree/v0.16.0)

* Upgrade to cosmwasm v0.16.0

### [v0.14.3](https://github.com/provenance-io/provwasm/tree/v0.14.3)

* Allow a supply of zero when creating a marker.

### [v0.14.2](https://github.com/provenance-io/provwasm/tree/v0.14.2)

* Add metadata module query support for scopes, sessions, and records.
* Add missing attribute value types to fix de-serialization errors.

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
