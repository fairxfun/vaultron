build:
	cargo build

start-enclave:
	cd ../enclave && make test

start-tester:
	../target/debug/vaultron_enclave_agent_tester

start-proxy:
	cd ../enclave && make start-proxy

stop-enclave:
	cd ../enclave && make stop

stop-proxy:
	cd ../enclave && make stop-proxy

integration-test: start-enclave start-proxy start-tester

.PHONY: clean
clean:
	rm -rf ${RUST_DIR}/../target ${RUST_DIR}/vaultron_enclave_agent_tester
	cd ../enclave && make clean
