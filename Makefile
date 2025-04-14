build-elf-install:
	./scripts/build_elf_install.sh

run-install:
	./scripts/run_install.sh

build-enclave:
	cargo build --release --bin vaultron_enclave --target x86_64-unknown-linux-gnu

build-enclave-agent:
	cargo build --release --bin vaultron_enclave_agent --target x86_64-unknown-linux-gnu

build: build-enclave build-enclave-agent

img:
	docker build -t vaultron-enclave:latest -f ./container/enclave/Dockerfile .

eif:
	nitro-cli build-enclave --docker-uri vaultron-enclave:latest --output-file vaultron_enclave.eif
	mkdir -p ./target/x86_64-unknown-linux-gnu/elf
	mv vaultron_enclave.eif ./target/x86_64-unknown-linux-gnu/elf/

all: build img eif 

run:
	nitro-cli run-enclave \
		--enclave-name vaultron_enclave_1000 \
		--enclave-cid 1000 \
		--eif-path ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif \
		--cpu-count 2 \
		--memory 1024

run-debug:
	nitro-cli run-enclave \
		--enclave-name vaultron_enclave_1000 \
		--enclave-cid 1000 \
		--eif-path ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif \
		--cpu-count 2 \
		--memory 1024 \
		--debug-mode

run-tester: 
	./target/release/integration_tester

console:
	nitro-cli console --enclave-name vaultron_enclave_1000

test: run-debug sleep run-tester

test-all: all test

sleep:
	sleep 5

stop-tester:
	pkill -x "integration_tester" || true

stop-enclave:
	nitro-cli terminate-enclave --all

stop: stop-tester stop-enclave

.PHONY: clean
clean:
	rm -rf ./target
	docker rmi -f vaultron_enclave
