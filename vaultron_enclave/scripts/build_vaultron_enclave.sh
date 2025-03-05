#!/bin/bash

set -e

cargo build --release
cp ../target/release/vaultron_enclave .
