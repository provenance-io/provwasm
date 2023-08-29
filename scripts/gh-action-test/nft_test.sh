#!/bin/bash -e

# This script stores and instantiates the nft smart contract
PROV_CMD="provenanced"
WASM="./contracts/nft/artifacts/nft.wasm"

# setup all of the necessary keys
"$PROV_CMD" keys add owner_A --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"
"$PROV_CMD" keys add owner_B --keyring-backend test --testnet --hd-path "44'/1'/0'/0/0"

# setup key variables
export node0=$("$PROV_CMD" keys show -a validator --keyring-backend test --testnet )
export owner_A=$("$PROV_CMD" keys show -a owner_A --keyring-backend test --testnet )
export owner_B=$("$PROV_CMD" keys show -a owner_B --keyring-backend test --testnet )

echo "Sending coins to different keys"

"$PROV_CMD" tx bank send \
  "$node0" \
  "$owner_A" \
  200000000000nhash \
  --from="$node0" \
  --keyring-backend=test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode=block \
  --yes \
  --testnet \
  --output json


"$PROV_CMD" tx bank send \
  "$node0" \
  "$owner_B" \
  200000000000nhash \
  --from="$node0" \
  --keyring-backend=test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode=block \
  --yes \
  --testnet \
  --output json

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

"$PROV_CMD" tx wasm instantiate 1 \
 '{
    "default":{
      "contract_spec_uuid":"9fe17f9a-56e1-4158-a8af-450680ac9e60",
      "scope_spec_uuid":"7a65b199-66bc-4d7d-af46-7321b3b017f1"
    }
  }' \
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

 "$PROV_CMD" tx authz grant $contract generic \
   --msg-type=/provenance.metadata.v1.MsgWriteScopeRequest \
   --from="$owner_A" \
   --keyring-backend test \
   --chain-id="testing" \
   --gas=auto \
   --gas-prices="1905nhash" \
   --gas-adjustment=1.5 \
   --broadcast-mode block \
   --yes \
   --testnet

  "$PROV_CMD" tx authz grant $contract generic \
   --msg-type=/provenance.metadata.v1.MsgWriteSessionRequest \
   --from="$owner_A" \
   --keyring-backend test \
   --chain-id="testing" \
   --gas=auto \
   --gas-prices="1905nhash" \
   --gas-adjustment=1.5 \
   --broadcast-mode block \
   --yes \
   --testnet

"$PROV_CMD" tx wasm execute $contract \
 '{
    "mint":{
      "scope_uuid": "fe8a2073-1284-421f-9e85-34edd18dec85",
      "session_uuid":"bbd5b2df-5adb-4557-9f87-ed678281bef8"
    }
  }' \
  --from="$owner_A" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet

"$PROV_CMD" tx wasm execute $contract \
 '{
    "transfer_nft":{
      "id": "fe8a2073-1284-421f-9e85-34edd18dec85",
      "recipient":"'"$owner_B"'",
      "session_uuid":"bbd5b2df-5adb-4557-9f87-ed678281bef8"
    }
  }' \
  --from="$owner_A" \
  --keyring-backend test \
  --chain-id="testing" \
  --gas=auto \
  --gas-prices="1905nhash" \
  --gas-adjustment=1.5 \
  --broadcast-mode block \
  --yes \
  --testnet
  
# "$PROV_CMD" tx wasm execute \
#  "$contract" \
#  '{
#    "burn":{
#        "id":"fe8a2073-1284-421f-9e85-34edd18dec85"
#     }
#   }' \
#  --from="$owner_B" \
#  --keyring-backend test \
#  --chain-id="testing" \
#  --gas=auto \
#  --gas-prices="1905nhash" \
#  --gas-adjustment=1.5 \
#  --broadcast-mode block \
#  --yes \
#  --testnet
