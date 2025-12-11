# provwasm-std Feature Flags

This document describes the feature flags available in `provwasm-std` and how to use them to optimize compile times.

## Overview

`provwasm-std` generates Rust bindings for protobuf types from Provenance, Cosmos SDK, IBC, and other blockchain modules. By default, this includes hundreds of types across many modules. To reduce compile times, you can use feature flags to include only the modules your contract needs.

## Benefits

- **Faster compile times**: Only compile the types you actually use
- **Smaller binaries**: Less code means smaller WASM files
- **Clearer dependencies**: Explicitly declare which blockchain modules your contract uses

## Important Notes

### Coin Conversion Utilities

The `cosmos-base` feature is required to use coin conversion utilities:
- `cosmwasm_to_proto_coins` - Convert CosmWasm `Coin` types to proto `Coin` types
- `try_proto_to_cosmwasm_coins` - Convert proto `Coin` types to CosmWasm `Coin` types

These functions are conditionally compiled and only available when `cosmos-base` is enabled. The `cosmos-base` feature is included by default, but can be disabled for minimal builds if you don't need coin conversions.

## Default Features

By default, `provwasm-std` includes the most commonly used modules:

```toml
default = ["provenance-all", "cosmos-core", "cosmos-base"]
```

This includes:
- **provenance-all**: All Provenance modules (`attribute`, `marker`, `name`, `metadata`, `msgfees`, `trigger`, `asset`, `exchange`, `hold`, `oracle`, `registry`, `ledger`, `ibchooks`, `ibcratelimit`, `flatfees`)
- **cosmos-core**: `auth`, `bank`, `base`, `tx` modules (with `tendermint`)
- **cosmos-base**: Included by default for coin conversion utilities (can be disabled for minimal builds)

## Feature Flags

### Top-Level Features

- **`full`**: Include all modules (previous behavior, for backward compatibility)
- **`default`**: Core modules commonly used by Provenance contracts

### Provenance Modules

**Feature Groups:**
- **`provenance-all`**: All Provenance modules (includes all individual modules listed below)

**Individual Modules:**
- `provenance-attribute` - Attribute management
- `provenance-marker` - Marker/coin management  
- `provenance-name` - Name service
- `provenance-metadata` - Metadata/scope management
- `provenance-msgfees` - Message fee assessment
- `provenance-trigger` - Event triggers
- `provenance-asset` - Asset classification
- `provenance-exchange` - Exchange functionality
- `provenance-hold` - Fund holds
- `provenance-oracle` - Oracle integration
- `provenance-registry` - Registry management
- `provenance-ledger` - Ledger functionality
- `provenance-ibchooks` - IBC hooks
- `provenance-ibcratelimit` - IBC rate limiting
- `provenance-flatfees` - Flat fee management

### Cosmos SDK Modules

**Feature Groups:**
- **`cosmos-core`**: Common Cosmos modules (`auth`, `bank`, `base`, `tx`)
- **`cosmos-all`**: All Cosmos SDK modules

**Individual Modules:**
- `cosmos-app` - Application framework (requires `cosmos-base`)
- `cosmos-auth` - Authentication (requires `cosmos-base`)
- `cosmos-authz` - Authorization (requires `cosmos-base`)
- `cosmos-bank` - Bank/coin management (requires `cosmos-base`)
- `cosmos-base` - Base types (required for coin conversion utilities: `cosmwasm_to_proto_coins`, `try_proto_to_cosmwasm_coins`)
- `cosmos-circuit` - Circuit breaker (requires `cosmos-base`)
- `cosmos-consensus` - Consensus
- `cosmos-crisis` - Crisis management (requires `cosmos-base`)
- `cosmos-crypto` - Cryptographic types
- `cosmos-distribution` - Staking rewards distribution (requires `cosmos-base`)
- `cosmos-evidence` - Evidence handling (requires `cosmos-base`)
- `cosmos-feegrant` - Fee grants (requires `cosmos-base`)
- `cosmos-genutil` - Genesis utilities
- `cosmos-gov` - Governance (requires `cosmos-base`)
- `cosmos-group` - Group accounts (requires `cosmos-base`)
- `cosmos-ics23` - ICS23 proofs
- `cosmos-mint` - Token minting
- `cosmos-msg` - Message types
- `cosmos-nft` - NFT support (requires `cosmos-base`)
- `cosmos-orm` - ORM functionality (requires `cosmos-base`)
- `cosmos-params` - Parameters
- `cosmos-quarantine` - Quarantine functionality (requires `cosmos-base`)
- `cosmos-query` - Query types
- `cosmos-sanction` - Sanctions (requires `cosmos-base`)
- `cosmos-slashing` - Slashing (requires `cosmos-base`)
- `cosmos-staking` - Staking (requires `cosmos-base`)
- `cosmos-store` - Store types
- `cosmos-tx` - Transaction types (requires `cosmos-crypto`, `cosmos-base`)
- `cosmos-upgrade` - Chain upgrades
- `cosmos-vesting` - Vesting accounts (requires `cosmos-auth`)

### IBC Modules

**Feature Groups:**
- **`ibc-all`**: All IBC modules
- **`ibc-core`**: Core IBC protocol (requires `cosmos-ics23`, `cosmos-upgrade`, `cosmos-base`)
- **`ibc-applications`**: IBC applications (transfer, interchain accounts, etc.) (requires `ibc-core`, `cosmos-auth`)
- **`ibc-lightclients`**: Light client implementations (requires `ibc-core`)

### Other Modules

- **`cosmwasm-all`**: CosmWasm-specific types (requires `cosmos-base`)
- **`tendermint-all`**: Tendermint consensus types (automatically included with Cosmos/Provenance modules)
- **`capability`**: Capability module
- **`provlabs-all`**: Provenance Labs extensions (requires `cosmos-base`, `cosmos-bank`, `cosmos-auth`)

## Usage Examples

### Minimal Contract (Name Service Only)

```toml
[dependencies]
provwasm-std = { workspace = true, default-features = false, features = [
    "provenance-name",
    # Note: provenance-name automatically includes tendermint-all and cosmos-base
]}
```

### Attribute Contract

```toml
[dependencies]
provwasm-std = { workspace = true, default-features = false, features = [
    "provenance-attribute",
    "provenance-name",
    # Note: Both modules automatically include tendermint-all and cosmos-base
]}
```

### Marker Contract with IBC

```toml
[dependencies]
provwasm-std = { workspace = true, default-features = false, features = [
    "provenance-marker",
    "provenance-name",
    # Note: provenance-marker automatically includes cosmos-bank and ibc-applications
    # Both provenance modules include tendermint-all and cosmos-base
]}
```

### Message Fee Contract

```toml
[dependencies]
provwasm-std = { workspace = true, default-features = false, features = [
    "provenance-msgfees",
    "cosmos-bank",
    # Note: Both modules automatically include tendermint-all and cosmos-base
]}
```

### Full Access (Testing/Integration)

```toml
[dependencies]
provwasm-std = { workspace = true, features = ["full"] }
```

## Feature Dependencies

Some modules automatically include their dependencies:

- All **Provenance modules** include:
  - `tendermint-all` (Provenance types use Tendermint types)
  - `cosmos-base` (for pagination types)
  
- **`provenance-exchange`** additionally includes:
  - `cosmos-auth` (exchange requires authentication types)
  
- **`provenance-marker`** additionally includes:
  - `cosmos-bank` (marker uses bank metadata)
  - `ibc-applications` (for IBC transfers)
  
- All **Cosmos modules** include:
  - `tendermint-all` (Cosmos types use Tendermint types)
  
- Many **Cosmos modules** also include:
  - `cosmos-base` (for base types like Coin, pagination, etc.)
  - Modules that require `cosmos-base`: `app`, `auth`, `authz`, `bank`, `circuit`, `crisis`, `distribution`, `evidence`, `feegrant`, `gov`, `group`, `nft`, `orm`, `quarantine`, `sanction`, `slashing`, `staking`, `tx`
  
- **`cosmos-tx`** includes:
  - `cosmos-crypto` (transaction signing)
  - `cosmos-base` (base types)
  
- **`cosmos-vesting`** includes:
  - `cosmos-auth` (vesting accounts extend base accounts)
  
- **`ibc-core`** includes:
  - `cosmos-ics23` (IBC proofs)
  - `cosmos-upgrade` (chain upgrades)
  - `cosmos-base` (base types)
  
- **`ibc-applications`** includes:
  - `ibc-core` (applications depend on core protocol)
  - `cosmos-auth` (for account authentication)
  
- **`ibc-lightclients`** includes:
  - `ibc-core` (light clients depend on core protocol)
  
- **`cosmwasm-all`** includes:
  - `cosmos-base` (for base types)
  
- **`provlabs-all`** includes:
  - `cosmos-base` (base types)
  - `cosmos-bank` (bank functionality)
  - `cosmos-auth` (authentication)

## Migration Guide

### From Implicit Dependencies (Pre-Features)

If you're migrating from a version without feature flags:

**Old:**
```toml
provwasm-std = { workspace = true }
```

**New (recommended):**
```toml
provwasm-std = { workspace = true, default-features = false, features = [
    "provenance-<your-modules>",
    # Add other modules as needed
]}
```

**Or keep everything (backward compatible):**
```toml
provwasm-std = { workspace = true, features = ["full"] }
```

### Finding Required Features

1. Look at your imports:
   ```rust
   use provwasm_std::types::provenance::marker::v1::...;
   ```
   → Need `provenance-marker`

2. Check compiler errors for missing modules:
   ```
   error: could not find `marker` in `provenance`
   ```
   → Add `provenance-marker` feature

3. Start minimal and add features as needed based on compiler feedback

## Performance Impact

Typical compile time improvements when using specific features instead of `full`:

- **Name-only contract**: ~40-50% faster
- **Attribute contract**: ~35-45% faster  
- **Marker contract**: ~30-40% faster
- **Complex multi-module contract**: ~20-30% faster

Results vary based on contract complexity and enabled features.

## Troubleshooting

### "could not find X in Y"

Add the missing feature for module X:
```toml
features = ["provenance-<module>", ...]
```

### "could not find `cosmos`"

Make sure you have at least one cosmos or provenance feature enabled. Cosmos modules are auto-included as dependencies.

### "could not find `base` in `cosmos`"

Add the `cosmos-base` feature. Many modules require it, and it's needed for coin conversion utilities (`cosmwasm_to_proto_coins`, `try_proto_to_cosmwasm_coins`).

### Still Slow Compile Times?

- Use `cargo build --timings` to identify slow dependencies
- Consider splitting large contracts into smaller ones
- Use `sccache` or similar tools to cache compiled dependencies

## Contributing

When adding new protobuf modules, remember to:
1. Add feature flags to `Cargo.toml`
2. Add `#[cfg(feature = "...")]` to module declarations
3. Document feature dependencies
4. Update this README
