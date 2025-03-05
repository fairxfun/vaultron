#!/bin/sh

sudo yum update -y

sudo dnf install aws-nitro-enclaves-cli -y
sudo yum install -y aws-nitro-enclaves-cli-devel

sudo systemctl enable nitro-enclaves-allocator.service
sudo systemctl start nitro-enclaves-allocator.service

sudo dnf install -y docker
sudo systemctl enable --now docker

sudo yum install -y llvm clang

sudo yum groupinstall "Development Tools" -y

