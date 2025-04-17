#!/usr/bin/env bash

set -e

buf generate
cargo fmt

sed -i '' 's/pub enum Response {/#[allow(clippy::large_enum_variant)]\npub enum Response {/' src/gen/vaultron.agent.v1.rs
