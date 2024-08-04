# Intro

This repository contains a simplified version of [Spire-VM](https://github.com/spire-labs/spvm-1) written in Rust.

It consists of two parts:

- SDK: The Solidity to Rust port
- CLI: Command line interface for interacting with the SDK

> Note: This is not meant to be production worthy and thus it has no tests, has not been reviewed and it likely contains bugs.

## Overview

### Spire-VM

This port is focused on the following functionality

- Interacting with user balances for specified token tickers
- Executing transactions in `SPVM` format or directly in `Bytes`
- Validating user signatures

It implements `rocksdb` for its backend to store state, specifically:

- User balance
- User nonce
- Token Tickers

### CLI

The CLI consumes the SDK and exposes an API for the following:

- Querying and setting the balance of a user
- Validating signature recovery
- Executing a transaction to update state, specifically:
  - Minting for a single user
  - Transfering and managing the balance between two users
  - Handling user nonce

## Usage

> Note: This has only been run on linux

### Build the project

In the root of the repository `/path/to/Spire-VM` run the following command

```bash
cargo build
```

### SDK

> Note: We'll use the binary from the repository without installing it on the system.

To interact with the `VM` use the `CLI` to submit commands.

#### Set Balance

> Note: That's a USDC address \
> Note: Only balance has been tested hence the concrete command examples for it

```bash
./target/debug/spire-cli balance \
    set \
    --ticker eth \
    --balance 13 \
    --account 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
```

#### Get Balance

```bash
./target/debug/spire-cli balance \
    view \
    --ticker eth \
    --account 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
```

#### Check Signature Validity

```bash
./target/debug/spire-cli signature \
    validate \
    --message-hash <MESSAGE_HASH> \
    --signature <SIGNATURE> \
    --signer <SIGNER>
```

#### Execute Transaction

```bash
./target/debug/spire-cli transaction \
    execute \
    --from <FROM> \
    --tx-type <TX_TYPE> \
    --tx-param <TX_PARAM> \
    --nonce <NONCE> \
    --transaction-hash <TRANSACTION_HASH> \
    --signature <SIGNATURE>
```

#### Execute Raw Transaction

```bash
./target/debug/spire-cli transaction \
    execute-raw \
    --transaction <TRANSACTION>
```