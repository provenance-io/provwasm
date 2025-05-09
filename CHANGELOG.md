# Changelog for provwasm

## Unreleased changes

* Update to Provenance v1.22.0 and CosmWasm v2.2.0 ([#166](https://github.com/provenance-io/provwasm/pull/166))

## Releases

### [v2.5.0](https://github.com/provenance-io/provwasm/tree/v2.5.0)

* Update to Provenance 1.20.0 ([#163](https://github.com/provenance-io/provwasm/issues/163))
* Update to cosmwasm 2.1.4 ([#163](https://github.com/provenance-io/provwasm/issues/163))

### [v2.4.0](https://github.com/provenance-io/provwasm/tree/v2.4.0)

* migrate to GRPC queries ([#157](https://github.com/provenance-io/provwasm/issues/157))
* add `MetadataAddress` type for encoding/decoding metadata
  addresses ([#161](https://github.com/provenance-io/provwasm/pull/161))

### [v2.3.0](https://github.com/provenance-io/provwasm/tree/v2.3.0)

* test tube integration tests ([#150](https://github.com/provenance-io/provwasm/pull/150))
* fix code generation ([#150](https://github.com/provenance-io/provwasm/issues/150))
* refactor `Any` to support Trigger messages ([#123](https://github.com/provenance-io/provwasm/issues/123))
* Update to cosmwasm 2.x ([#141](https://github.com/provenance-io/provwasm/issues/141))
* minimize generated types ([#152](https://github.com/provenance-io/provwasm/issues/152))
* update to provenance 1.19.0-rc5 ([#155](https://github.com/provenance-io/provwasm/issues/155))

### [v2.2.0](https://github.com/provenance-io/provwasm/tree/v2.2.0)

* Add wasm message types ([#133](https://github.com/provenance-io/provwasm/issues/133))
* Bump to Provenance 1.17.1 ([#135](https://github.com/provenance-io/provwasm/issues/135))
* Add ibc message types ([#136](https://github.com/provenance-io/provwasm/issues/136))
* Update contracts to use `cosmwasm/optimzer` ([#140](https://github.com/provenance-io/provwasm/issues/140))
* Create security contract to link assets with a security
  type ([#138](https://github.com/provenance-io/provwasm/issues/138))
* Bump to Provenance 1.18.0-rc2 ([#143](https://github.com/provenance-io/provwasm/issues/143))
* remove nft, security, and template contracts ([#145](https://github.com/provenance-io/provwasm/pull/145))
* Bump to Provenance 1.18.0 and add new exchange types ([#146](https://github.com/provenance-io/provwasm/issues/146))
* Update dependencies, including cosmwasm 1.5 ([#147](https://github.com/provenance-io/provwasm/issues/147))

### [v2.1.0](https://github.com/provenance-io/provwasm/tree/v2.1.0)

* Add trigger module message types ([#120](https://github.com/provenance-io/provwasm/issues/120))
* Add exchange module message types ([#127](https://github.com/provenance-io/provwasm/issues/127))

### [v2.0.0](https://github.com/provenance-io/provwasm/tree/v2.0.0)

* Use protos instead of json for bindings ([#101](https://github.com/provenance-io/provwasm/issues/101))

### [v1.2.0](https://github.com/provenance-io/provwasm/tree/v1.2.0)

* Update tutorial documentation ([#102](https://github.com/provenance-io/provwasm/issues/102))
* Add Fee struct to make it easier to implement message
  fees ([#106](https://github.com/provenance-io/provwasm/issues/106))
* Added ProvenanceMsg From implementation ([#108](https://github.com/provenance-io/provwasm/issues/102))
* Add forced transfer support for restricted markers ([#110](https://github.com/provenance-io/provwasm/issues/110))

### [v1.1.2](https://github.com/provenance-io/provwasm/tree/v1.1.2)

* Upgrade to CosmWasm 1.1.9 ([#97](https://github.com/provenance-io/provwasm/issues/97))
* Create example IBC contract ([#74](https://github.com/provenance-io/provwasm/issues/74))

### [v1.1.1](https://github.com/provenance-io/provwasm/tree/v1.1.1)

* Upgrade to latest cosmwasm v1.1.5 ([#95](https://github.com/provenance-io/provwasm/issues/95))

### [v1.1.0](https://github.com/provenance-io/provwasm/tree/v1.1.0)

* Smart Contract MsgFee Support ([#87](https://github.com/provenance-io/provwasm/issues/87))
* Add `UpdateAttribute` and `DeleteDistinctAttribute`
  bindings ([#86](https://github.com/provenance-io/provwasm/issues/86))

### [v1.0.0](https://github.com/provenance-io/provwasm/tree/v1.0.0)

* Upgrade to CosmWasm v1.0.0 ([#88](https://github.com/provenance-io/provwasm/issues/88))

### [v1.0.0-rc.0](https://github.com/provenance-io/provwasm/tree/v1.0.0-rc.0)

* Upgrade to CosmWasm v1.0.0-rc.0 ([#83](https://github.com/provenance-io/provwasm/issues/83))

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
