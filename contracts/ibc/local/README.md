# IBC Local Contract

This a CosmWasm smart contract that demonstrates the Inter-Blockchain Communication protocol capabilities.

This contract has the following functionality:

- Messages
  - Send a `WhoAmI` message to a remote chain contract such as [ibc-remote](../remote/README.md)
  - Change the admin account of the contract
- Queries
  - Get contract admin
  - Get test information received from remote chain contract on specified channel
  - Get test information received from all remote chain contract on all channels

## Build

Compile and optimize the smart contract Wasm.

```bash
make && make optimize
```

## Requirements

- [provenanced](https://github.com/provenance-io/provenance/releases) cli

## Workflow

***See the [Overview](../README.md) for end-to-end steps***

This contract is intended to be used with the other two projects in the `ibc` directory:
- [ibc-remote](../remote/README.md)
- [relayer](../relayer/README.md)

To simplify the setup and execution, several scripts are provided in the [scripts](scripts) directory.

1. [setup_local_chain.sh](scripts/setup_local_chain.sh)
  
    The `setup_local_chain` script will start a Provenance chain with 3 funded accounts: `validator`, `localaccount`, and `relayer`.
    This "local" chain will use the default ports.


3. [store_and_init_local_contract.sh](scripts/store_and_init_local_contract.sh)
    
    The `store_and_init_local_contract` script stores and initializes the "local" smart contract. **This step must be performed before starting the 
    `relayer`**


3. [execute_contract.sh](scripts/execute_contract.sh)

    The `execute_contract` script will send a `WhoAmI` request to the `remote` chain contract. The remote contract will
    respond with data that includes the current block height, timestamp, and chain-id of the remote chain as well as the
    remote contract's address. It subsequently monitors the current state data of the local contract so that the
    response can be visualized. It normally requires around 10-12 blocks before the response is received.