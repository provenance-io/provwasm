# Provenance Smart Contract Tutorial

In this section we will execute a purchase by sending funds to the contract account. The contract
will then peform bank transfers to the merchant and fee collection accounts.

## Execute Contract

To execute a `100fpcoin` purchase with an ID of `12345`, run

```bash
provenance tx wasm execute \
    tp18vd8fpwxzck93qlwghaj6arh4p7c5n89x8kskz \
    '{"purchase":{"id":"12345"}}' \
    --amount 100fpcoin \
    --from consumer \
    --keyring-backend test \
    --home build/node0/provenance \
    --chain-id chain-local \
    --gas auto \
    --fees 4000vspn \
    --broadcast-mode block \
    --yes
```

To ensure the transfers were sent successfully, first query the `merhcant` account

```bash
provenance query auth account \
    $(provenance keys show -a merchant --home build/node0/provenance --keyring-backend test)
```

This should show that the merchant has increased by `90fpcoin`

```yaml
|
  address: tp168zfahluza55e2vxrdta0c5c0dx5asdpvlz7pw
  coins:
  - denom: fpcoin
    amount: "90"
  - denom: vspn
    amount: "100000"
  public_key: ""
  account_number: 12
  sequence: 0
```

Then, query the `feebucket` account

```bash
provenance query auth account \
    $(provenance keys show -a feebucket --home build/node0/provenance --keyring-backend test)
```

This should show that the feebucket account has increased by `10fpcoin`

```yaml
|
  address: tp1clx2v0ze5wmckerm3wx9c2r7wcaf05ktwyaedj
  coins:
  - denom: fpcoin
    amount: "10"
  - denom: vspn
    amount: "71500"
  public_key: tppub1addwnpepqt9pw0ygrr09g3wpvgch68ydfxvmclzyd4ag6z9hn9qg9zvzjeh5xgl5hrw
  account_number: 13
  sequence: 2
```

Finally, query the `consumer` account

```bash
provenance query auth account \
    $(provenance keys show -a consumer --home build/node0/provenance --keyring-backend test)
```

This should show that it has decreased by `100fpcoin`

```yaml
|
  address: tp1s2nzrmekarkeckerxnmc6p2pd90yh2spzej0vg
  coins:
  - denom: fpcoin
    amount: "99900"
  - denom: vspn
    amount: "96000"
  public_key: tppub1addwnpepqtggdc7hdy9tgjzner6872txeegjlmukqzulfgh7qru5s987cus25stta60
  account_number: 14
  sequence: 1
```

## Up Next

The smart contract has been verified to be deployed and working. Proceed to the
[Migrate](13-migrate.md) section for notes on how to upgrade smart contracts. Or, proceed to Part 3
to see how to [Integrate](14-integration) Kotlin applications.
