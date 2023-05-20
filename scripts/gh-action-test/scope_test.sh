#!/bin/bash -e

# This script stores and instantiates the scope smart contract for the metadata module
PROV_CMD="provenanced"
WASM="./contracts/scope/artifacts/scope.wasm"

export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet )

"$PROV_CMD" tx name bind \
  "sc" \
  "$node0" \
  "pb" \
  --unrestrict \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

"$PROV_CMD" tx wasm store $WASM \
  --instantiate-anyof-addresses "$node0" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

"$PROV_CMD" tx wasm instantiate 1 '{"name": "scope-itv2.sc.pb"}' \
  --admin "$node0" \
  --label metadata_module_integration_test_v2 \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

export contract=$("$PROV_CMD" query wasm list-contract-by-code 1 --testnet --output json  | jq -r ".contracts[0]")

$PROV_CMD tx metadata write-contract-specification \
  contractspec1q0w6ys5g6jm509v2830374aprsrq260w62 \
  "$node0" \
  "owner" \
  "hashvalue" \
  "myclassname" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

$PROV_CMD tx metadata write-scope-specification \
  scopespec1qn7jh3jvw4gytq9r5x770e8yj74s9t479r \
  "$node0" \
  "OWNER" \
  contractspec1q0w6ys5g6jm509v2830374aprsrq260w62 \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

$PROV_CMD tx metadata write-scope \
  scope1qzhpuff00wpy2yuf7xr0rp8aucqstsk0cn \
  scopespec1qn7jh3jvw4gytq9r5x770e8yj74s9t479r \
  "$node0" \
  "$node0" \
  "$node0" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

#"$PROV_CMD" tx wasm execute \
#  "$contract" \
#  '{
#    "write_scope":{
#      "scope":{
#        "scope_id":"scopeid",
#        "specification_id":"scopeid",
#        "owners":['"$node0"'],
#        "data_access":"access",
#        "value_owner_address":"",
#        "require_party_rollup":false,
#      },
#      "signers": ["one"],
#    }
#  }' \
#  --from="$node0" \
#  --keyring-backend test \
#  --chain-id="testing" \
#  --gas=auto \
#  --gas-prices="1905nhash" \
#  --gas-adjustment=1.5 \
#  --broadcast-mode block \
#  --yes \
#  --testnet

"$PROV_CMD" q wasm contract-state smart "$contract" '{"get_contract_spec":{"id":"contractspec1q0w6ys5g6jm509v2830374aprsrq260w62"}}' --testnet
"$PROV_CMD" q wasm contract-state smart "$contract" '{"get_scope":{"id":"scope1qzhpuff00wpy2yuf7xr0rp8aucqstsk0cn"}}' --testnet

echo "done with the scope contract"
