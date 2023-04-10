# Template Contract
This contract aims to equip developers with a pre-configured smart contract that streamlines implementation and conforms to the standard Provenance layout. With only the need to add or modify endpoints and integrate their logic, developers can focus on their contract's core functionalities. Additionally, the accompanying documentation and tests provide clear examples for proper documentation practices.

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
These message variants are used to construct the contract and are dispatched to the instantiate entry point.

#### [Default]
The default instantiation message that provides commonly used setup functionality. This is primarily an example, but can be used as a good starting point.

#### Request Parameters

#### Emitted Attributes

#### Emitted Events

#### Request Sample

### [Execute]

#### [Change Owner]

#### Request Parameters

#### Emitted Attributes

#### Emitted Events

#### Request Sample

### [Migrate]

#### [Default]

#### Request Parameters

#### Emitted Attributes

#### Emitted Events

#### Request Sample

## Queries

### [Querry Version]

#### Request Sample

#### Response Sample

### [Querry Owner]

#### Request Parameters

#### Request Sample

#### Response Sample