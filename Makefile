lib:
	./scripts/build_kmstool_enclave_lib.sh

build:
	cargo build --release --features kmstool_aws_clib_feature

img:
	docker build -t vaultron_enclave -f ./container/enclave/Dockerfile .

eif:
	nitro-cli build-enclave --docker-uri vaultron_enclave --output-file vaultron_enclave.eif
	mkdir -p ./target/elf
	mv vaultron_enclave.eif ./target/elf/

run:
	nitro-cli run-enclave \
		--enclave-name vaultron_enclave_1000 \
		--enclave-cid 1000 \
		--eif-path ./target/elf/vaultron_enclave.eif \
		--cpu-count 2 \
		--memory 1024 \
		--debug-mode

run-tester: 
	./target/release/integration_tester

all: lib build img eif 

proxy:
	nohup vsock-proxy 8000 kms.ap-southeast-1.amazonaws.com 443 > /dev/null 2>&1 &

console:
	nitro-cli console --enclave-name vaultron_enclave_1000

test: run proxy sleep run-tester

test-all: all test

sleep:
	sleep 3

stop:
	nitro-cli terminate-enclave --all
	kill -9 $(shell pgrep -f "vsock-proxy")

.PHONY: clean
clean:
	rm -rf ./target
	docker rmi -f vaultron_enclave
	docker rmi -f kmstool-test
