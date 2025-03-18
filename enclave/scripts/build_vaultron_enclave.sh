#!/bin/bash

set -e

cargo build --release --features kmstool_aws_clib_feature
cp ../target/release/vaultron_enclave .
