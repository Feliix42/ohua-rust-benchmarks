#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin simple_sequential --features "cli bench"
cargo build --release --bin ohua --features "cli bench ohua"
{build_calls}

{executions}

echo "Finished benchmarks at $(date)!"
