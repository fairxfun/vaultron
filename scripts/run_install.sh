#!/bin/sh

dnf update -y

dnf install -y aws-nitro-enclaves-cli-1.3.4 aws-nitro-enclaves-cli-devel-1.3.4

systemctl enable nitro-enclaves-allocator.service
systemctl start nitro-enclaves-allocator.service


