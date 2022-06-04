#!/usr/bin/env bash
cp .env.example .env

cargo build --release
cargo install diesel_cli --no-default-features --features postgres
diesel database setup