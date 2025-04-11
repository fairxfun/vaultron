ARG BASE_IMAGE=public.ecr.aws/amazonlinux/amazonlinux:2023

FROM $BASE_IMAGE as VAULTRON_ENCLAVE

RUN yum install -y unzip

RUN mkdir -p /fairx
COPY ./target/x86_64-unknown-linux-gnu/release/vaultron_enclave /fairx/vaultron_enclave
RUN chmod +x /fairx/vaultron_enclave

RUN mkdir -p /fairx/certs
RUN curl -O https://aws-nitro-enclaves.amazonaws.com/AWS_NitroEnclaves_Root-G1.zip
RUN unzip AWS_NitroEnclaves_Root-G1.zip
RUN mv root.pem /fairx/certs/aws-nitro-enclaves-root.pem
RUN rm AWS_NitroEnclaves_Root-G1.zip

WORKDIR /fairx
CMD ["/fairx/vaultron_enclave"]
