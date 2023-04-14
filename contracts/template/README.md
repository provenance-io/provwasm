# Template Contract
This contract aims to equip developers with a pre-configured smart contract that streamlines implementation and conforms to the standard Provenance layout. With only the need to add or modify endpoints and integrate their logic, developers can focus on their contract's core functionalities. Additionally, the accompanying documentation and tests provide clear examples for proper documentation practices.

This contract was inspired by the [metadata-bilateral-exchange](https://github.com/provenance-io/metadata-bilateral-exchange) and similar contracts.

## Concepts
In the template contract, two crucial concepts that every developer should aim to comprehend are the incorporation of Fees to monetize their contract and the standard structure of the contract.

### Fee
Fees offer a straightforward method for contract developers to receive funds. If a fee lacks a recipient, it will be routed directly to Provenance foundation. However, if a recipient is specified, the fee will be split evenly, with 50% going to the foundation.

### Module Structure
Our contract has been carefully crafted with a modular design, allowing developers to seamlessly integrate their own functionalities without compromising the core components of the smart contract. All message handling follows a consistent pattern: messages enter through the entry points specified in contract.rs, then handed off to their message router, and ultimately are processed by their respective module's handler.

#### Contract
Within this module, you can find all the entrypoints that are invoked by wasmd. Each entrypoint first validates the incoming message before forwarding it to its corresponding message router.

#### Core
This set of modules comprises the fundamental components that every contract utilizes to define types and messages.

#### Events
A module used for defining events that will be emitted during contract state modification.

#### Execute, Instantiate, Migrate, Query, Reply
These modules encompass the complete implementation for the various endpoints corresponding to each message variant.

#### Storage
Acting as an intermediary between the contract and its stored state, this module offers a layer that streamlines storage for developers.

#### Testing
A set of modules to assist in testing the smart contract.

#### Util
This group of modules is used to provide additional tools and helpers for the contract.

## Transactions
The template contract enables all three transaction entry points. These transactions allow the user to manipulate contract and blockchain state with messages. 

### [Instantiation]
These message variants are utilized in the construction of the contract and to activate the instantiate entry point.

#### [Default]
A default instantiation message that provides and demonstrates commonly used setup functionality.

#### Request Parameters
- owner: The address of the account that will own the contract.
- fee: A fee to be applied when instantiating the contract.

#### Emitted Attributes
- action: This will always have a value of "instantiate".

#### Emitted Events
This transaction does not emit any events.

#### Request Sample
```
{
    "default": {
        "owner": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr",
        "fee": {
            "recipient": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr",
            "amount": {
                "denom": "nhash",
                "amount": "1000000000"
            }
        }
    }
}
```

### [Execute]
The execute message variants encompass the various methods through which users can interact with and modify the contract. These variants activate the execute entry point.

#### [Change Owner]
This transaction will change the owner of the contract to the provided address.

##### Note
This message variant will fail if the sender is not the current owner of the contract.

#### Request Parameters
- new_owner: The address of the contract's new owner.

#### Emitted Attributes
- action: This will always have the value of "change_owner".

#### Emitted Events
- change_owner:
    - previous_owner: The address of the previous owner.
    - new_owner: The address of the new owner.

#### Request Sample
```
{
    "change_owner": {
        "new_owner": "tp1ek77wghn0n9lc7x2uycgh697sjc7fvy995keun"
    }
}
```

### [Migrate]
The migrate message variants enable the developer to make their contract future-proof by providing the ability to upgrade it in multiple ways.

#### [Default]
A default instantiation message that provides and demonstrates commonly used migration functionality.

#### Request Parameters
- None

#### Emitted Attributes
- action: This will always have a value of "migrate".

#### Emitted Events
- None

#### Request Sample
```
{
    "default": {}
}
```

### [Query]
The migrate message variants allow the users to quickly obtain contract information.

### [Query Version]
The QueryVersion message will return the contract's current version.

#### Request Parameters
- None

#### Request Sample
```
{
    "query_version": {}
}
```

#### Response Sample
```
{
    "data": {
        "contract_version": {
            "contract": "template",
            "version": "1.0.0"
        }
    }
}
```

### [Query Owner]
The QueryVersion message will return the contract's current owner.

#### Request Parameters
- None

#### Request Sample
```
{
    "query_owner": {}
}
```

#### Response Sample
```
{
    "data": {
        "owner": "tp1y8e0lvgek8em05mpxk8veqfyz9tzwphylq7hxr"
    }
}
```