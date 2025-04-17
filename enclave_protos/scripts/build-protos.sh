#!/usr/bin/env bash

set -e

buf generate
cargo fmt

if [ -f "src/gen/vaultron.agent.v1.rs" ]; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        sed -i '' 's/pub enum Response {/#[allow(clippy::large_enum_variant)]\npub enum Response {/' src/gen/vaultron.agent.v1.rs
    else
        # Linux
        sed -i 's/pub enum Response {/#[allow(clippy::large_enum_variant)]\npub enum Response {/' src/gen/vaultron.agent.v1.rs
    fi
fi
