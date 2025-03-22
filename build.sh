#!/usr/bin/env bash

set -e  # Exit script if any command fails

# Check if cargo-leptos is installed, install if missing
if ! command -v cargo-leptos &>/dev/null; then
    echo "🚀 Installing cargo-leptos..."
    cargo install cargo-leptos
fi

# Ensure Diesel CLI is installed
if ! command -v diesel &>/dev/null; then
    echo "🛠 Installing Diesel CLI..."
    cargo install diesel_cli --no-default-features --features postgres
fi

# Run Diesel migrations only if the `migrations/` folder exists
if [ -d "migrations" ]; then
    echo "📦 Running Diesel migrations..."
    diesel migration run
else
    echo "⚠️ No 'migrations/' folder found. Skipping migrations."
fi

# Check if Bun is installed, use it for package installation
if command -v bun &>/dev/null; then
    echo "⚡ Bun detected! Running 'bun install'..."
    bun install
else
    echo "📦 Bun not found. Falling back to 'npm install'..."
    npm install
fi

# Build Leptos app
echo "🚀 Building Leptos project..."
cargo leptos build --release

echo "✅ Setup completed successfully!"
