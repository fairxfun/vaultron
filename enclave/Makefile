# Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

ARCH := $(shell uname -m)
RUST_DIR := $(shell readlink -m $(shell dirname $(firstword $(MAKEFILE_LIST))))

buildLib:
	./scripts/build_kmstool_enclave_lib.sh

build:
	./scripts/build_vaultron_enclave.sh

buildLibTest:
	./scripts/build_kmstool_enclave_lib.sh
	./scripts/build_kmstool_enclave_lib_test.sh

docker:
	docker build -t vaultron_enclave -f ./container/Dockerfile .

dockerLibTest:
	docker build -t vaultron_enclave -f ./container/DockerfileLibTest .

eif:
	nitro-cli build-enclave --docker-uri vaultron_enclave --output-file vaultron_enclave.eif

start:
	nitro-cli run-enclave \
		--enclave-name vaultron_enclave_1000 \
		--enclave-cid 1000 \
		--eif-path vaultron_enclave.eif \
		--cpu-count 2 \
		--memory 1024 \
		--debug-mode


start-proxy:
	vsock-proxy 8000 kms.ap-southeast-1.amazonaws.com 443

console:
	nitro-cli console --enclave-name vaultron_enclave_1000

stop:
	nitro-cli terminate-enclave --all

test: buildLib build docker eif stop start

testLib: buildLibTest dockerLibTest eif stop start

.PHONY: clean
clean:
	rm -rf ${RUST_DIR}/../target ${RUST_DIR}/fair_enclave_*.eif ${RUST_DIR}/vaultron_enclave ${RUST_DIR}/libkmstool-enclave-lib.so ${RUST_DIR}/libnsm.so vaultron_enclave.eif kmstool_enclave_lib_test
	docker rmi -f vaultron_enclave
