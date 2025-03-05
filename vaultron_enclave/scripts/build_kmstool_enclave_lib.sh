#!/bin/bash

set -e

rm -rf aws-nitro-enclaves-sdk-c
rm -f libkmstool-enclave-lib.so
rm -f libnsm.so

git clone git@github.com:fairxfun/aws-nitro-enclaves-sdk-c.git
cd aws-nitro-enclaves-sdk-c

cd ./lib/kmstool-enclave-lib/
chmod +x build.sh
./build.sh

cd ../../../
cp ./aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/libkmstool-enclave-lib.so .
cp ./aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/libnsm.so .
