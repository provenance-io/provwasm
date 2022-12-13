#!/usr/bin/env bash

rm -rf ./target

provenanced -t --home target init --chain-id=local local
provenanced -t --home target keys add validator --keyring-backend test
provenanced -t --home target keys add localaccount --keyring-backend test
printf 'remind produce consider glory ill bind surge organ over brisk weekend portion size nasty object mention supreme someone brisk evidence short library scissors bone' | provenanced -t --home target keys add relayer --recover
provenanced -t --home target add-genesis-root-name validator pio --keyring-backend test
provenanced -t --home target add-genesis-root-name validator io --restrict --keyring-backend test
provenanced -t --home target add-genesis-root-name validator provenance --keyring-backend test
provenanced -t --home target add-genesis-root-name validator sc --restrict=false --keyring-backend test
provenanced -t --home target add-genesis-account validator 100000000000000000000nhash --keyring-backend test
provenanced -t --home target add-genesis-account localaccount 10000000000000000nhash --keyring-backend test
provenanced -t --home target add-genesis-account relayer 50000000000000000000nhash --keyring-backend test
provenanced -t --home target add-genesis-marker 200000000000000000000nhash --manager validator --access mint,burn,admin,withdraw,deposit --activate --keyring-backend test
provenanced -t --home target gentx validator 1000000000000000nhash --keyring-backend test --chain-id=local
provenanced -t --home target collect-gentxs

provenanced -t --home target --address tcp://0.0.0.0:26658 --rpc.laddr tcp://127.0.0.1:26657 --p2p.laddr tcp://0.0.0.0:26656 --grpc.address 0.0.0.0:9090 --grpc-web.address 0.0.0.0:9091 start
