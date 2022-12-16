#!/usr/bin/env bash

rm -rf ./target

provenanced -t --home target init --chain-id=remote remote
provenanced -t --home target keys add validator --keyring-backend test
provenanced -t --home target keys add remoteaccount --keyring-backend test
printf 'awesome eye lock unable enforce brand myth meadow flavor book energy borrow cloth field general enemy industry expose snow swim police vehicle naive neck' | provenanced -t --home target keys add relayer --recover
provenanced -t --home target add-genesis-root-name validator pio --keyring-backend test
provenanced -t --home target add-genesis-root-name validator io --restrict --keyring-backend test
provenanced -t --home target add-genesis-root-name validator provenance --keyring-backend test
provenanced -t --home target add-genesis-root-name validator sc --restrict=false --keyring-backend test
provenanced -t --home target add-genesis-account validator 100000000000000000000nhash --keyring-backend test
provenanced -t --home target add-genesis-account remoteaccount 10000000000000000nhash --keyring-backend test
provenanced -t --home target add-genesis-account relayer 50000000000000000000nhash --keyring-backend test
provenanced -t --home target add-genesis-marker 200000000000000000000nhash --manager validator --access mint,burn,admin,withdraw,deposit --activate --keyring-backend test
provenanced -t --home target gentx validator 1000000000000000nhash --keyring-backend test --chain-id=remote
provenanced -t --home target collect-gentxs

provenanced -t --home target --address tcp://0.0.0.0:36658 --rpc.laddr tcp://127.0.0.1:36657 --p2p.laddr tcp://0.0.0.0:36656 --grpc.address 0.0.0.0:9190 --grpc-web.address 0.0.0.0:9191 start
