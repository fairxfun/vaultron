#!/bin/sh

dnf update -y

dnf install aws-nitro-enclaves-cli -y
dnf install aws-nitro-enclaves-cli-devel -y

systemctl enable nitro-enclaves-allocator.service
systemctl start nitro-enclaves-allocator.service


