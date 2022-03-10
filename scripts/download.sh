#!/bin/bash -e

export Provenance_Version="v1.7.6"

wget "https://github.com/provenance-io/provenance/releases/download/$Provenance_Version/provenance-linux-amd64-v1.7.6.zip"

# this will create a folder with both provenance and libwasm
unzip provenance-linux-amd64-v1.7.6.zip
