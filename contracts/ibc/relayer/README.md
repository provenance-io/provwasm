# IBC Relayer

This project contains the configuration and start script for the relayer that demonstrates the Inter-Blockchain Communication protocol capabilities.

**This contract does not have any direct user initiated messages. It can only receive messages from the [local](../local/README.md) contract.**

## Requirements

- [hermes](https://github.com/informalsystems/hermes/releases) cli

## Workflow

***See the [Overview](../README.md) for end-to-end steps***

This relayer configuration is intended to be used with the other two projects in the `ibc` directory:
- [ibc-local](../local/README.md)
- [ibc-remote](../remote/README.md)

To simplify the setup and execution, there is a script in the [scripts](scripts) directory.

1. [init_and_start_relayer.sh](init_and_start_relayer.sh)
  
    The `init_and_start_relayer` script will import both relayer keys from the `local` and `remote` chain `relayer` account.
    These accounts were created with enough funds for the hermes relayer to create transactions on the respective chains.
    

**It is important to note that the contract addresses "may" change and require you to modify the script with the correct addresses.**

e.g.
```bash
# on local chain
LOCAL_CONTRACT_ADDRESS=tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8
# on remote chain
REMOTE_CONTRACT_ADDRESS=tp14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s96lrg8
```