# Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

ARCH := $(shell uname -m)
RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))

build:
# 	curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
# 	rustup target install ${ARCH}-unknown-linux-musl
	cargo build --manifest-path=${RUST_DIR}/Cargo.toml --target=${ARCH}-unknown-linux-musl --release
	cp ${RUST_DIR}/../target/${ARCH}-unknown-linux-musl/release/vaultron_enclave_agent ${RUST_DIR}

start:
	./vaultron_enclave_agent

.PHONY: clean
clean:
	rm -rf ${RUST_DIR}/../target ${RUST_DIR}/vaultron_enclave_agent
