#!/bin/sh

# Update system
dnf update -y

# Install AWS Nitro Enclaves CLI
dnf install -y aws-nitro-enclaves-cli-1.3.4 aws-nitro-enclaves-cli-devel-1.3.4 

# Install perl
dnf install -y perl git jq coreutils

# Install development tools
# dnf groupinstall -y "Development Tools"

