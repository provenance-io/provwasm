#!/bin/bash -e
set -x

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
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

"$PROV_CMD" tx wasm store $WASM \
  --instantiate-anyof-addresses "$node0" \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

"$PROV_CMD" tx wasm instantiate 1 '{"name": "scope-itv2.sc.pb"}' \
  --admin "$node0" \
  --label metadata_module_integration_test_v2 \
  --from="$node0" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

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
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

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
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

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
  --yes \
  --testnet | "$PROV_CMD" q wait-tx

#"$PROV_CMD" tx wasm execute \
#  "$contract" \
#  '{
#    "write_scope": {
#      "scope": {
#        "scope_id": [0, 174, 30, 37, 47, 123, 130, 69, 19, 137, 241, 134, 241, 132, 253, 230, 1],
#        "specification_id": [4, 253, 43, 198, 76, 117, 80, 69, 128, 163, 161, 189, 231, 228, 228, 151, 171],
#        "owners": ["'"$node0"'"],
#        "value_owner_address": "'"$node0"'"
#      },
#    }
#  }' \
#  --from="$node0" \
#  --keyring-backend test \
#  --chain-id="testing" \
#  --gas=auto \
#  --gas-prices="1905nhash" \
#  --gas-adjustment=1.5 \
#  --yes \
#  --testnet

"$PROV_CMD" q wasm contract-state smart "$contract" '{"get_contract_spec":{"id":"contractspec1q0w6ys5g6jm509v2830374aprsrq260w62"}}' --testnet
"$PROV_CMD" q wasm contract-state smart "$contract" '{"get_scope":{"id":"scope1qzhpuff00wpy2yuf7xr0rp8aucqstsk0cn"}}' --testnet

echo "done with the scope contract"
