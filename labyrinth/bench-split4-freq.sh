#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin split4-frequency --features "cli bench ohua"
cargo build --release --bin ohua-frequency --features "cli bench ohua"
cargo build --release --bin ohua-split1 --features "cli bench ohua"
cargo build --release --bin ohua-split2 --features "cli bench ohua"
cargo build --release --bin ohua-split3 --features "cli bench ohua"
cargo build --release --bin ohua-split4 --features "cli bench ohua"
cargo build --release --bin ohua-split5 --features "cli bench ohua"
cargo build --release --bin ohua-split6 --features "cli bench ohua"
cargo build --release --bin ohua-split7 --features "cli bench ohua"
cargo build --release --bin ohua-split8 --features "cli bench ohua"
cargo build --release --bin ohua-split9 --features "cli bench ohua"
cargo build --release --bin ohua-split10 --features "cli bench ohua"
cargo build --release --bin ohua-split11 --features "cli bench ohua"
cargo build --release --bin ohua-split12 --features "cli bench ohua"

# ------ random-x128-y128-z5-n128.txt ------
echo "Running benchmarks for random-x128-y128-z5-n128.txt"

echo -n "Ohua (split4-frequency)..."
# run ohua
target/release/split4-frequency --json --outdir split_freq_mixed --runs 30 --frequency 4 inputs/random-x128-y128-z5-n128.txt
target/release/split4-frequency --json --outdir split_freq_mixed --runs 30 --frequency 8 inputs/random-x128-y128-z5-n128.txt
target/release/split4-frequency --json --outdir split_freq_mixed --runs 30 --frequency 12 inputs/random-x128-y128-z5-n128.txt
target/release/split4-frequency --json --outdir split_freq_mixed --runs 30 --frequency 16 inputs/random-x128-y128-z5-n128.txt
target/release/split4-frequency --json --outdir split_freq_mixed --runs 30 --frequency 20 inputs/random-x128-y128-z5-n128.txt
target/release/split4-frequency --json --outdir split_freq_mixed --runs 30 --frequency 32 inputs/random-x128-y128-z5-n128.txt

echo " done ."


echo -n "Ohua (frequency)..."
# run ohua
target/release/ohua-frequency --json --outdir split_freq_mixed --runs 30 --frequency 4 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir split_freq_mixed --runs 30 --frequency 8 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir split_freq_mixed --runs 30 --frequency 12 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir split_freq_mixed --runs 30 --frequency 16 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir split_freq_mixed --runs 30 --frequency 20 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir split_freq_mixed --runs 30 --frequency 32 inputs/random-x128-y128-z5-n128.txt

echo " done ."


echo -n "Ohua (split)..."
# run ohua
target/release/ohua-split1 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split2 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split3 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split4 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split5 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split6 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split7 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split8 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split9 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split10 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split11 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-split12 --json --outdir split_freq_mixed --runs 30 inputs/random-x128-y128-z5-n128.txt


echo " done ."
