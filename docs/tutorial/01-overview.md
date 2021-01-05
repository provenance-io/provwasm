# Provenance Smart Contract Tutorial

## NOTICE

This tutorial is a work in progress. All sections are subject to frequent change without further
notice.

--

This tutorial will walk developers through a step-by-step process of creating and deploying a
web assembly smart contract to the provenance blockchain.

## Overview

This tutorial is broken out into three parts. The first will cover developing smart contracts.
The second will deal with deploying smart contracts to the provenance blockchain.
The final part will show how to execute and query smart contracts from Kotlin using the
provenance blockchain simple-client.

### Part 1: Development

- [Introduction](02-intro.md) - What is CosmWasm?
- [Getting Started](03-getting-started.md) - Set up a development environment.
- [Project](04-project.md) - Generate a smart contract project from template.
- [Requirements](05-requirements.md) - Define smart contract functionality.
- [Develop](06-develop.md) - Code, compile, and test the smart contract.
- [Optimize](07-optimize.md) - Create optimized contract WASM for deployment.

### Part 2: Deployment

- [Setup](08-setup.md) - Start a provenance localnet cluster, create accounts and mint fpcoin.
- [Store](09-store.md) - Upload optimized contract WASM.
- [Instantiate](10-instantiate.md) - Create an instance of the contract.
- [Query](11-query.md) - Get the smart contract configuration state.
- [Execute](12-execute.md) - Execute the contract, sending funds and collecting fees.
- [Migrate](13-migrate.md) - Notes on upgrading smart contracts.

### Part 3: Integration

- [Integration](14-integration.md): Use the Kotlin simple client to execute the contract.

### Up Next

Continue to the [Introduction](02-intro.md) for a high-level background of smart contracts.

To dive right in, proceed to the [Getting Startred](03-getting-started.md) section.
