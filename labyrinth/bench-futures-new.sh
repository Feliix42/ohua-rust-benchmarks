#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin simple_sequential --features "cli bench"
cargo build --release --bin stm --features "cli bench transactional"
cargo build --release --bin ohua_futures --features "future cli bench ohua"

# ------ random-x256-y256-z5-n256.txt ------
echo "Running benchmarks for random-x256-y256-z5-n256.txt"

# echo -n "simple_sequential"
# target/release/simple_sequential inputs/random-x256-y256-z5-n256.txt --json --outdir future_benches_cmp --runs 30
# echo " done."
# 
# echo -n "stm ."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 4 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 6 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 8 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 10 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 12 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 14 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 16 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 18 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 20 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 22 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/stm --json --outdir future_benches_cmp --runs 30 --threads 24 inputs/random-x256-y256-z5-n256.txt
# echo " done."
# 
# 
# echo -n "ohua-futures [2 tasks/thread, 2 items/task] ."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 4 --tasks 8 --frequency 16 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 6 --tasks 12 --frequency 24 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 8 --tasks 16 --frequency 32 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 10 --tasks 20 --frequency 40 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 12 --tasks 24 --frequency 48 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 14 --tasks 28 --frequency 56 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 16 --tasks 32 --frequency 64 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 18 --tasks 36 --frequency 72 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 20 --tasks 40 --frequency 80 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 22 --tasks 44 --frequency 88 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 24 --tasks 48 --frequency 96 inputs/random-x256-y256-z5-n256.txt
# echo " done."


echo -n "ohua-futures [2 tasks/thread, 1 item/task] ."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 4 --tasks 8 --frequency 8 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 6 --tasks 12 --frequency 12 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 8 --tasks 16 --frequency 16 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 10 --tasks 20 --frequency 20 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 12 --tasks 24 --frequency 24 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 14 --tasks 28 --frequency 28 inputs/random-x256-y256-z5-n256.txt
echo -n "."
target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 16 --tasks 32 --frequency 32 inputs/random-x256-y256-z5-n256.txt
echo -n "."
target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 18 --tasks 36 --frequency 36 inputs/random-x256-y256-z5-n256.txt
echo -n "."
target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 20 --tasks 40 --frequency 40 inputs/random-x256-y256-z5-n256.txt
echo -n "."
target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 22 --tasks 44 --frequency 44 inputs/random-x256-y256-z5-n256.txt
echo -n "."
target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 24 --tasks 48 --frequency 48 inputs/random-x256-y256-z5-n256.txt
echo " done."


# echo -n "ohua-futures [1 task/thread, 2 items/task] ."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 4 --tasks 4 --frequency 8 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 6 --tasks 6 --frequency 12 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 8 --tasks 8 --frequency 16 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 10 --tasks 10 --frequency 20 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 12 --tasks 12 --frequency 24 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 14 --tasks 14 --frequency 28 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 16 --tasks 16 --frequency 32 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 18 --tasks 18 --frequency 36 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 20 --tasks 20 --frequency 40 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 22 --tasks 22 --frequency 44 inputs/random-x256-y256-z5-n256.txt
# echo -n "."
# target/release/ohua_futures --json --outdir future_benches_cmp --runs 30 --threads 24 --tasks 24 --frequency 48 inputs/random-x256-y256-z5-n256.txt
# echo " done."


echo "Finished at $(date)"
