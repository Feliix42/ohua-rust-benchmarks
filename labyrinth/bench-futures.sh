#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# # clean and build
# cargo clean
# cargo build --release --bin ohua_futures --features "future cli bench ohua"


# # ------ random-x128-y128-z5-n128.txt ------
# echo "Running benchmarks for random-x128-y128-z5-n128.txt"
# 
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 16 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 24 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 32 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 40 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 48 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 56 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 64 inputs/random-x128-y128-z5-n128.txt
# 
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 32 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 48 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 64 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 80 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 96 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 112 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 128 inputs/random-x128-y128-z5-n128.txt
# 
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 32 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 48 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 64 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 80 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 96 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 112 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 128 inputs/random-x128-y128-z5-n128.txt
# 
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 64 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 96 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 128 inputs/random-x128-y128-z5-n128.txt
# 
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 64 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 96 inputs/random-x128-y128-z5-n128.txt
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 128 inputs/random-x128-y128-z5-n128.txt
# 
# target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 24 --tasks 48 --frequency 96 inputs/random-x128-y128-z5-n128.txt
# 
# echo "Done."


# ------ random-x256-y256-z5-n256.txt ------
echo "Running benchmarks for random-x256-y256-z5-n256.txt"

target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 16 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 24 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 32 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 40 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 48 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 56 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 8 --frequency 64 inputs/random-x256-y256-z5-n256.txt

target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 32 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 48 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 64 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 80 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 96 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 112 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 4 --tasks 16 --frequency 128 inputs/random-x256-y256-z5-n256.txt

target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 32 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 48 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 64 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 80 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 96 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 112 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 16 --frequency 128 inputs/random-x256-y256-z5-n256.txt

target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 64 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 96 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 128 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 160 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 192 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 224 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 8 --tasks 32 --frequency 256 inputs/random-x256-y256-z5-n256.txt

target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 64 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 96 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 128 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 160 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 192 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 224 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 16 --tasks 32 --frequency 256 inputs/random-x256-y256-z5-n256.txt

target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 24 --tasks 48 --frequency 96 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 24 --tasks 48 --frequency 144 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 24 --tasks 48 --frequency 192 inputs/random-x256-y256-z5-n256.txt
target/release/ohua_futures --json --outdir future_benches --runs 30 --threads 24 --tasks 48 --frequency 240 inputs/random-x256-y256-z5-n256.txt

echo "Done."


echo "Finished at $(date)"
