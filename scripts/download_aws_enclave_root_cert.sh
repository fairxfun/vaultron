#!/bin/bash

curl -O https://aws-nitro-enclaves.amazonaws.com/AWS_NitroEnclaves_Root-G1.zip
unzip AWS_NitroEnclaves_Root-G1.zip
mkdir -p ./certs
mv root.pem ./certs/aws-nitro-enclaves-root.pem
rm AWS_NitroEnclaves_Root-G1.zip