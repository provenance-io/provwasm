## Provenance CW721 Non Fungible Token Contract

This contract is a reference implementation of the [CW721](https://github.com/CosmWasm/cw-nfts/tree/main/packages/cw721)non fungible token standard utilizing [ProvWasm](../../README) for the [Provenance Blockchain](https://provenance.io)

---

### Concepts

The Provenance Blockchain provides a custom [Metadata module](https://developer.provenance.io/docs/pb/modules/metadata-module) which provides protocol-level attestations regarding authorization, facts, data, and provenance of user-configurable contract executions. While the module is complex, it is very powerful and expressive in it's abilities. Reading and understanding the capabilities of the module is encouraged, but not necessary, to create a smart contract-backed NFT on the Provenance Blockchain.

This contract manages a custom NFT, storing the metadata within the contract data storage, and uses the `metadata module` to store the ownership data and records and enforcing the requirements as specified in the Contract, Record, and Scope specifications. 

NFTs can be represented using the `metadata module` via the following mappings:

| Ethereum-based NFT                  | Provenance NFT                                                          |
|-------------------------------------|-------------------------------------------------------------------------|
| Managing Contract                   | ProvWasm contract instance and Contract Specification for enforcement   |
| NFT specification                   | Scope Specification                                                     |
| NFT instance                        | Scope                                                                   |
| NFT owner                           | Scope Value Owner                                                       |
| NFT data/owner change specification | Record Specification                                                    |
| NFT data/owner change events        | Scope Record                                                            |
| NFT metadata                        | Within contract storage (e.g. this implementation) or in a Scope Record |

---

### Supported CW721 Methods

#### [Execution](src/execute/README.md)
- `Approve`
- `ApproveAll`
- `Revoke`
- `RevokeAll`
- `SendNft`
- `TransferNft`
#### [Query](src/query/README.md)
- `OwnerOf`
- `Approval`
- `Approvals`
- `AllOperators`
- `NumTokens`
#### [Metadata Query](src/query/README.md#metadata-query-methods)
- `ContractInfo`
- `ContractVersion`
- `NftInfo`
- `AllNftInfo`
#### [Paginated Query](src/query/README.md#paginated-query-methods)
- `Tokens`
- `AllTokens`
### [Additional Execution](src/execute/README.md#additional-execution-methods)
- `Burn`
- `Mint` *in this nft example, the minter has already received funds OOB since they are the only one authorized to call the mint function*
- `Update Ownership`

---

### Project Structure

- [core](src/core) - constants, input/output/error messages
- [events](src/events)
- [execute](src/execute) - execute method handlers
- [instantiate](src/instantiate) - instantiate handler
- [query](src/query) - query method handlers
- [storage](src/storage) - nft and related data storage handlers
- [util](src/util) - miscellaneous utilities