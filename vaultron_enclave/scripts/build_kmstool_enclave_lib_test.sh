#!/bin/bash

set -e

rm -rf aws-nitro-enclaves-sdk-c
rm -f kmstool_enclave_lib_test
rm -f libnsm.so

git clone git@github.com:fairxfun/aws-nitro-enclaves-sdk-c.git
cd aws-nitro-enclaves-sdk-c

cd ./lib/kmstool-enclave-lib-test/
chmod +x build.sh
./build.sh

cd ../../../
cp ./aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib-test/libkmstool-enclave-lib.so .
cp ./aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib-test/kmstool_enclave_lib_test .
cp ./aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib-test/libnsm.so .

