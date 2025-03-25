#!/bin/bash

set -e

rm -f /usr/lib64/libkmstool-enclave-lib.so
rm -f /usr/lib64/libnsm.so
if [ -d ./target/kmstool-enclave-lib ]; then
    rm -f ./target/kmstool-enclave-lib/*
else
    mkdir -p ./target/kmstool-enclave-lib
fi

cd enclave_kmstool/lib/aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/
chmod +x build.sh
./build.sh

echo "run build.sh done"

cd ../../../../../
cp ./enclave_kmstool/lib/aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/libkmstool-enclave-lib.so /usr/lib64/
cp ./enclave_kmstool/lib/aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/libnsm.so /usr/lib64/
mv ./enclave_kmstool/lib/aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/libkmstool-enclave-lib.so ./target/kmstool-enclave-lib/
mv ./enclave_kmstool/lib/aws-nitro-enclaves-sdk-c/lib/kmstool-enclave-lib/libnsm.so ./target/kmstool-enclave-lib/

echo "build kmstool-enclave-lib done"
