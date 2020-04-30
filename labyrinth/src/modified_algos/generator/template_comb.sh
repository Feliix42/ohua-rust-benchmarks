#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin sequential --features "cli"
# cargo build --release --bin ohua-frequency --features "cli ohua"
{build_calls}

{executions}

echo "Finished benchmarks at $(date)!"
