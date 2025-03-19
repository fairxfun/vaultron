# Copyright 2021 Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
ARG BASE_IMAGE=public.ecr.aws/amazonlinux/amazonlinux:2023

FROM $BASE_IMAGE as VAULTRON_ENCLAVE

#RUN yum install -y ca-certificates && update-ca-trust

RUN mkdir /fairx

#RUN rpm -e python python-libs python-urlgrabber python2-rpm pygpgme pyliblzma python-iniparse pyxattr python-pycurl amazon-linux-extras yum yum-metadata-parser yum-plugin-ovl yum-plugin-priorities
COPY ./target/kmstool-enclave-lib/libnsm.so /usr/lib64/libnsm.so
COPY ./target/kmstool-enclave-lib/libkmstool-enclave-lib.so /usr/lib64/libkmstool-enclave-lib.so
COPY ./target/release/vaultron_enclave /fairx/vaultron_enclave

WORKDIR /fairx
CMD ["/fairx/vaultron_enclave"]
