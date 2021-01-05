# Provenance Smart Contract Tutorial

## Requirements

For this tutorial, the smart contract shall provide the following functionality.

**Create a smart contract that handles purchases - the transfer of coin from a consumer to a
merchant. Transfers incur fees, calculated as a percentage of the amount sent. Fees shall be
sent to a fee collection account, the remainder to the merchant.**

## Overview

Smart contracts are similar to [actors](https://en.wikipedia.org/wiki/Actor_model).

In response to messages received, a smart contract can modify internal state and emit events,
but can only affect other contracts and modules by passing further messages.

Because of this, it is useful to define functionality in terms of the types of messages the smart
conctract can handle. There are four types of message handlers.

- Init: called when contracts are instantiated; setting the initial contract state.
- Handle: performs actions (change state, send funds, send more messages, generate events, etc).
- Query: access the state of the contract, other contracts, and blockchain modules.
- Migrate(optional): called when a contract is migrated to a new code instance.

The tutorial contract requires the following handler functions and messages.

## Init

```rust
pub fn init(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> ...
```

Params

- `DepsMut` - Holds external contract dependencies - mutable storage, address, and querier APIs.
- `Env` - Contains blockchain environment info - block height, block time, and the contract address.
- `MessageInfo` - Contains info for authorization - identity of the caller, and payment.
- `InitMsg` - The user-defined message sent to initialize the contract

The following fields must be provided in the initialization message

- a name for the contract
- the merchant required denomination
- the merchant address
- the fee collection address (the sender from the info message)
- the fee percentage

The following validations must be performed during initialization.

- there should not be any funds sent
- the fee percentage must be > 0% and <= 25%
- the merchant address cannot equal the fee collection address
- the contract name must be available (not bound)

## Handle

```rust
pub fn handle(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> ...
```

Params

- `DepsMut` - Holds external contract dependencies - mutable storage, address, and querier APIs.
- `Env` - Contains blockchain environment info - block height, block time, and the contract address.
- `MessageInfo` - Contains info for authorization - identity of the caller, and payment.
- `HandleMsg` - The user-defined message for purchases.

The following fields are required for purchases

- The purchase ID (handle message)
- The sent funds (message info)
- The sender address (message info)

The following validations must be performed.

- The purchase amount is > 0
- The purchase demon matches the required merchant denom
- The sender account has sufficient funds (implied)

Since the purchase ID isn't stored, or part of the bank transfers, it shall be emitted as an event
attribute, along with the block time. This is important so downstream event listeners can take
action when a purchase has been completed.

## Query

```rust
pub fn query(
    deps: Deps,
    env: Env,
    msg: QueryMsg,
) -> ...
```

Params

- `Deps` - Holds external contract dependencies - _immutable_ storage, address, and querier APIs.
- `Env` - Contains blockchain environment info - block height, block time, and the contract address.
- `QueryMsg` - The user-defined parms sent to query the contract

There are no params for the tutorial query, so `QueryMsg` shall contain an empty request object.

Querying the contract shall return a JSON object with the following data

- The contract name
- The merchant address
- The merchant required denomination
- The fee collection address
- The fee percentage

## Migrate

Migrations are left as an exercise for the reader. Suggested modifications

1) Change the allowed transfer fee percentage range.
1) Update the transfer fee percentage to a new value in the migrate message.

## Up Next

Proceed to the next [section](06-develop.md) to implement the smart contract functionality defined
here.
