#!/usr/bin/env bash

set -euxo pipefail

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
LATEST_PROVENANCE_VERSION="v1.18.0"
PROVENANCE_REV=${1:-$LATEST_PROVENANCE_VERSION}
COMMIT=${2:-"skip"}

####################################
## Update and rebuild provwasm-std
####################################

# update revision in proto-build main.rs
PROTO_BUILD_MAIN_RS="$SCRIPT_DIR/../packages/proto-build/src/main.rs"

# use @ as a separator to avoid confusion on input like "origin/main"
sed -i -- "s@const PROVENANCE_REV: \&str = \".*\";@const PROVENANCE_REV: \&str = \"$PROVENANCE_REV\";@g" "$PROTO_BUILD_MAIN_RS"

#git diff

# rebuild provwasm-std
echo "[PROTO-BUILD] - generating provwasm-std types"
cd "$SCRIPT_DIR/../packages/proto-build/" && cargo run -- --update-deps
echo "[PROTO-BUILD] - completed generating provwasm-std types"

########################################
## Skip or update git revision if there is
## any change
## default: skip
########################################

#if [[ $COMMIT == "commit" ]]; then
#  if [[ $(git diff --stat) != '' ||  $(git ls-files  --exclude-standard  --others) ]]; then
#    # add, commit and push
#    git add "$SCRIPT_DIR/.."
#    git commit -m "rebuild with $(git rev-parse --short HEAD:dependencies/provenance)"
#
#    # remove "origin/"
#    PROVENANCE_REV=$(echo "$PROVENANCE_REV" | sed "s/^origin\///")
#    BRANCH="autobuild-$PROVENANCE_REV"
#
#    # force delete local "$BRANCH" if exists
#    git branch -D "$BRANCH" || true
#
#    git checkout -b "$BRANCH"
#    git push -uf origin "$BRANCH"
#  else
#    echo '[CLEAN] no update needed for this build'
#  fi
#  exit 0
#fi

echo '[SKIP] skipping git update'
exit 0

# if dirty or untracked file exists

