#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin simple_sequential --features "cli bench"
# cargo build --release --bin stm --features "cli bench transactional"
cargo build --release --bin ohua-frequency --features "cli bench ohua"

# ------ random-x32-y32-z3-n96.txt ------
echo "Running benchmarks for random-x32-y32-z3-n96.txt"

echo -n "Sequential..."
# run sequential version
# target/release/simple_sequential inputs/random-x32-y32-z3-n96.txt --json -o stm_freq --runs 30

echo " done."

echo -n "STM..."
# run stm version


echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 1 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 5 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 10 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 20 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 29 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 39 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 48 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 58 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 68 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 77 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 87 inputs/random-x32-y32-z3-n96.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 96 inputs/random-x32-y32-z3-n96.txt


echo " done ."


# ------ random-x48-y48-z3-n64.txt ------
echo "Running benchmarks for random-x48-y48-z3-n64.txt"

echo -n "Sequential..."
# run sequential version
# target/release/simple_sequential inputs/random-x48-y48-z3-n64.txt --json -o stm_freq --runs 30

echo " done."

echo -n "STM..."
# run stm version


echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 1 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 4 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 7 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 13 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 20 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 26 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 32 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 39 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 45 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 52 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 58 inputs/random-x48-y48-z3-n64.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 64 inputs/random-x48-y48-z3-n64.txt


echo " done ."


# ------ random-x128-y128-z5-n128.txt ------
echo "Running benchmarks for random-x128-y128-z5-n128.txt"

echo -n "Sequential..."
# run sequential version
# target/release/simple_sequential inputs/random-x128-y128-z5-n128.txt --json -o stm_freq --runs 30

echo " done."

echo -n "STM..."
# run stm version


echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 2 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 7 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 13 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 26 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 39 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 52 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 64 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 77 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 90 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 103 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 116 inputs/random-x128-y128-z5-n128.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 128 inputs/random-x128-y128-z5-n128.txt


echo " done ."


# ------ random-x256-y256-z5-n256.txt ------
echo "Running benchmarks for random-x256-y256-z5-n256.txt"

echo -n "Sequential..."
# run sequential version
# target/release/simple_sequential inputs/random-x256-y256-z5-n256.txt --json -o stm_freq --runs 30

echo " done."

echo -n "STM..."
# run stm version


echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 3 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 13 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 26 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 52 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 77 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 103 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 128 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 154 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 180 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 205 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 231 inputs/random-x256-y256-z5-n256.txt
target/release/ohua-frequency --json --outdir stm_freq --runs 30 --frequency 256 inputs/random-x256-y256-z5-n256.txt


echo " done ."


echo "Finished at $(date)"
