ARG BASE_IMAGE=public.ecr.aws/amazonlinux/amazonlinux:2023.7.20250414.0

FROM $BASE_IMAGE

RUN mkdir -p /fairx/certs
COPY ./certs/aws-nitro-enclaves-root.pem /fairx/certs/aws-nitro-enclaves-root.pem
COPY ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave /fairx/vaultron_enclave
RUN chmod +x /fairx/vaultron_enclave

WORKDIR /fairx
CMD ["/fairx/vaultron_enclave"]
