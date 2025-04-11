#!/bin/sh

# Update system
dnf update -y

# Install AWS Nitro Enclaves CLI
dnf install -y aws-nitro-enclaves-cli aws-nitro-enclaves-cli-devel 

# Install perl
dnf install -y perl git

# Install development tools
# dnf groupinstall -y "Development Tools"

