# Contributing to provwasm

Thanks for taking the time to contribute! :+1:

The following is a set of guidelines for contributing to the CosmWasm bindings for the Provenance
Blockchain, which are hosted in the [Provenance Organization](https://github.com/provenance-io) on
GitHub. These are guidelines, not rules. Use your best judgment, and feel free to propose changes
to this document in a pull request.

## Table Of Contents

[Code of Conduct](#code-of-conduct)

[What is all this malarke...I just have a question!!!](#i-just-have-a-question)

[What should I know before I get started?](#what-should-i-know-before-i-get-started)

* [Design Decisions](#design-decisions)
* [Packages](#provwasm-packages)

[How Can I Contribute?](#how-can-i-contribute)

* [Reporting Bugs](#reporting-bugs)
* [Suggesting Enhancements](#suggesting-enhancements)
* [Pull Requests](#pull-requests)

[Styleguides](#styleguides)

* [Git Commit Messages](#git-commit-messages)
* [Rust Styleguide](#rust-styleguide)
* [Documentation Styleguide](#documentation-styleguide)

## Code of Conduct

This project and everyone participating in it is governed by the [Code of Conduct](CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior to
community@provenance.io.

## I just have a question

**NOTE:** Please don't file an issue to ask a question. You'll get faster results by using the
resources below.

Currently, we have a Slack team you can join:

* [Join the Provenance Slack Team](https://provenanceio.slack.com)

  * Even though Slack is a chat service, sometimes it takes several hours for community members to
    respond &mdash; please be patient!
  * Start with the `#general` channel for questions/discussions, or to find a more specific channel.

## What should I know before I get started?

### Design Decisions

The design of `provwasm` was based on the
[wasmd integration guide](https://github.com/CosmWasm/wasmd/blob/master/INTEGRATION.md#extending-the-contract-interface).

In the future, when we make a significant decision in how we maintain the project and what we can
or cannot support, we will document it. For now, if you have a question around how we do things,
please ask a question in Slack.

### Provwasm Packages

#### Bindings

This [package](https://github.com/provenance-io/provwasm/blob/main/packages/bindings/README.md)
provides bindings that enable CosmWasm smart contracts to interact &mdash; encode handler
messages or query keepers &mdash; with custom modules in the Provenance Blockchain.

#### Mocks

This [package](https://github.com/provenance-io/provwasm/blob/main/packages/mocks/README.md)
provides mocks that enable unit testing of CosmWasm smart contracts that interact with custom
modules in the Provenance Blockchain. The functionality provided here should NOT be used directly
in the `instantiate`, `execute`, or `query` functions of smart contracts (ie only use them in tests).

## How Can I Contribute?

### Reporting Bugs

Bugs are tracked as GitHub issues.

Before creating bug reports, perform a cursory search to see if the problem has already been
reported. If it has and the issue is still open, add a comment to the existing issue instead of
opening a new one.

After you've determined it is a new bug, create an issue and provide the following information:

* Use a clear and descriptive title for the issue to identify the problem.
* Describe the exact steps which reproduce the problem in as many details as possible. When listing
  steps, don't just say what you did, but explain how you did it.
* Describe the behavior you observed after following the steps and point out what exactly is the
  problem with that behavior. Explain which behavior you expected to see instead and why.
* Provide specific examples to demonstrate the steps. Include links to files or GitHub projects, or
  copy/pasteable snippets, which you use in those examples. When providing snippets in the issue,
  use markdown.

### Suggesting Enhancements

Like bugs, enhancements are tracked as GitHub issues.

Before suggesting an enhancement, perform a cursory search to see if the feature has already been
requested. If it has and the issue is still open, add a comment to the existing issue instead of
opening a new one.

After you've determined it is a new feature, create an issue and provide the following information:

* Use a clear and descriptive title for the issue to identify the suggestion.
* Provide a step-by-step description of the suggested enhancement in as many details as possible.
* Provide specific examples to demonstrate the steps. Include copy/pasteable markdown snippets
  which you use in those examples.
* Describe the current behavior and explain which behavior you expected to see instead and why.
* If possible, include screenshots which help you demonstrate your point.
* Explain why this enhancement would be useful to most smart contract users.

### Pull Requests

The process described here has several goals:

* Maintain or improve code quality.
* Fix problems that are important to smart contract developers.
* Engage the community in working toward the best possible API.
* Enable a sustainable system for `provwasm` maintainers to review contributions.

Please follow these steps to have your contribution considered:

* Format and lint all code.
* Follow the [Rust Styleguide](#rust-styleguide).
* Include unit tests.
* Update integration test contracts (add a contract for new module integrations).
* If necessary, update the
  [tutorial](https://github.com/provenance-io/provwasm/tree/main/docs/tutorial).
* Verify correct operation against the Provenance Blockchain.
* Add a link to close the referenced issue (PRs without an issue will be rejected).

Optional:

Before you submit a PR, we suggest you run the GitHub actions locally against your branch
by using our
[act image](https://github.com/provenance-io/provwasm/blob/main/docker/act/README.md).

## Styleguides

### Git Commit Messages

* Limit the first line to 72 characters or less.
* Use the present tense ("Add feature" not "Added feature").
* Reference issues liberally after the first line.

### Rust Styleguide

Whenever possible, use the standards set by the
[Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

### Documentation Styleguide

* Use [Markdown](https://daringfireball.net/projects/markdown)
* Use a Markdown [linter](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)

## References

The contributing document from the
[Atom](https://github.com/atom/atom/blob/master/CONTRIBUTING.md) editor project was used as a
template for this document.

