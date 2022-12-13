# IBC Remote Contract

This a CosmWasm smart contract that demonstrates the Inter-Blockchain Communication protocol capabilities.

**This contract does not have any direct user initiated messages. It can only receive messages from the [local](../local/README.md) contract.**


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
- [ibc-local](../local/README.md)
- [relayer](../relayer/README.md)

To simplify the setup and execution, several scripts are provided in the [scripts](scripts) directory.

1. [setup_remote_chain.sh](scripts/setup_remote_chain.sh)
  
    The `setup_remote_chain` script will start a Provenance chain with 3 funded accounts: `validator`, `remoteaccount`, and `relayer`.
    This "remote" chain will use ports that are slightly modified from default so that the two chains can run simultaneously.


3. [store_and_init_remote_contract.sh](scripts/store_and_init_remote_contract.sh)
    
    The `store_and_init_remote_contract` script stores and initializes the "remote" smart contract. **This step must be performed before starting the 
    `relayer`**

## IBC Messages

### Requests

1.
     ```rust
     #[cw_serde]
     pub enum PacketMsg {
         WhoAmI {},
     }
     ```

### Responses
1.
    ```rust
    #[cw_serde]
    pub struct WhoAmIResponse {
        pub account: String,
        pub block_info: BlockInfo,
    }
    ```