#!/bin/sh
set -e

# shellcheck disable=SC2154
sed -i.bak "s/version = \".*\" # generated/version = \"$npm_package_version\" \# generated/" Cargo.toml
cargo build
git add Cargo.toml Cargo.lock
