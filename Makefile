build-elf-install:
	./scripts/build_elf_install.sh

run-install:
	./scripts/run_install.sh

build-enclave:
	cargo build --release --bin vaultron_enclave --target x86_64-unknown-linux-gnu

build-enclave-agent:
	cargo build --release --bin vaultron_enclave_agent --target x86_64-unknown-linux-gnu

build-integration-tester:
	cargo build --release --bin integration_tester --target x86_64-unknown-linux-gnu

build: build-enclave build-enclave-agent build-integration-tester

img:
	docker buildx build \
	--build-arg SOURCE_DATE_EPOCH=$$(date -d "$$(date +%Y-%m-%d)" +%s) \
	--provenance=false \
	--output type=docker,name=ghcr.io/fairxfun/vaultron-enclave$(if $(DOCKER_TAG),:$(DOCKER_TAG),:latest),rewrite-timestamp=true \
	-f ./container/enclave/Dockerfile . \
	--no-cache

eif:
	nitro-cli build-enclave --docker-uri ghcr.io/fairxfun/vaultron-enclave$(if $(DOCKER_TAG),:$(DOCKER_TAG),:latest) --output-file vaultron_enclave.eif
	mkdir -p ./target/x86_64-unknown-linux-gnu/elf
	mv vaultron_enclave.eif ./target/x86_64-unknown-linux-gnu/elf/

checksums:
	sha256sum ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif | cut -d' ' -f1 > ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif.sha256
	cat ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif.sha256
	sha256sum ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave | cut -d' ' -f1 > ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave.sha256
	cat ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave.sha256
	sha256sum ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave_agent | cut -d' ' -f1 > ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave_agent.sha256
	cat ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave_agent.sha256

all: build img eif checksums

run-enclave:
	nitro-cli run-enclave \
		--enclave-name vaultron_enclave_1000 \
		--enclave-cid 1000 \
		--eif-path ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif \
		--cpu-count 2 \
		--memory 1024

run-enclave-debug:
	nitro-cli run-enclave \
		--enclave-name vaultron_enclave_1000 \
		--enclave-cid 1000 \
		--eif-path ./target/x86_64-unknown-linux-gnu/elf/vaultron_enclave.eif \
		--cpu-count 2 \
		--memory 1024 \
		--debug-mode

run-enclave-agent:
	./target/x86_64-unknown-linux-gnu/release/vaultron_enclave_agent

run-integration-tester: 
	./target/x86_64-unknown-linux-gnu/release/integration_tester

console:
	nitro-cli console --enclave-name vaultron_enclave_1000

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
