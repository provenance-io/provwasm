# Provenance CosmWasm Bindings

## NOTICE

This is currently the development repo for provwasm on the stargate(0.40.x) release of the Provenance Blockchain.

All code is subject to frequent change without further notice.

--

This crate provides bindings to enable CosmWasm smart contracts to interact with custom provenance
blockchain modules.

## Compatibility

The following table shows provwasm version compatibility and network availability for smart
contract development and testing.

| provwasm | wasmd | cosmos-sdk | provenance | localnet | testnet | mainnet | module support        |
| -------- | ----- | ---------- | ---------- | -------- | ------- | ------- | --------------------- |
| v0.13.2  | 0.14  | v0.40.1    | v0.1.x     | yes      | yes     | N/A     | attribute,marker,name |

## Getting Started

Start with the [tutorial](docs/tutorial/01-overview.md) for a complete guide to developing
smart contracts for the provenance blockchain.

## Example Usage

See the [examples](docs/examples/README.md) for usage.

## Contents

In addition to the core CosmWasm functionality, the provenance bindings include

### Query Support

- Names
  - Resolve the address for a name.
  - Lookup all names bound to an address.
- Account Metadata
  - Get all attributes for an account.
  - Get attributes for an account by name.
- Scope Metadata
  - Get scope by ID.
- Markers (in progress)
  - Get marker by denom.
  - Get marker by address.

### Message Encoding

- Names
  - Bind name
  - Delete name
- Account Metadata
  - Add attribute
  - Delete attributes
- Markers (in progress)
  - Create
  - Grant access
  - Revoke access
  - Finalize
  - Activate
  - Cancel
  - Destroy
  - Mint supply
  - Burn supply
  - Withdraw coins
  - Transfer coins
