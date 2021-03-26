#!/usr/bin/env bash

set -euo pipefail

RUNS=20
TODAY=`date +%Y-%m-%d`

# clean everything
echo "[build] Cleaning old output data"
cargo --quiet clean
rm -rf Cargo.lock

# build benchmarks
echo -n "[build] Building binaries."
cargo --quiet build --release --bin sequential --features "cli"
echo -n "."
cargo --quiet build --release --bin ohua_futures --features "cli ohua future"
echo -n "."
cargo --quiet build --release --bin ohua_unsafe --features "cli ohua future"
echo -n "."
cargo --quiet build --release --bin stm --features "cli transactional"
echo -n "."
cargo --quiet build --release --bin stm_small_tx --features "cli transactional less_tx"
echo " done."

# run simsmall
echo "[run] Running simsmall"
echo -n "    SEQ"
target/release/sequential --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 inputs/100000.nets
echo " - done."

echo -n "    STM"
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 1 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 2 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 3 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 4 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 5 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 6 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 7 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 8 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 9 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 10 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 11 inputs/100000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 12 inputs/100000.nets
echo " - done."

echo -n "    STM (small tx)"
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 1 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 2 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 3 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 4 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 5 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 6 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 7 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 8 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 9 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 10 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 11 inputs/100000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 12 inputs/100000.nets
echo " - done."

echo -n "    OHUA"
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 1 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 2 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 3 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 4 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 5 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 6 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 7 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 8 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 9 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 10 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 11 inputs/100000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 12 inputs/100000.nets
echo " - done."

echo -n "    OHUA (unsafe)"
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 1 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 2 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 3 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 4 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 5 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 6 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 7 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 8 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 9 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 10 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 11 inputs/100000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simsmall" --swaps 10000 --temperature 2000 --max-steps 32 --threads 12 inputs/100000.nets
echo " - done."


# run simmedium
echo "[run] Running simmedium"
echo -n "    SEQ"
target/release/sequential --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 inputs/200000.nets
echo " - done."

echo -n "    STM"
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 1 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 2 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 3 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 4 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 5 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 6 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 7 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 8 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 9 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 10 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 11 inputs/200000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 12 inputs/200000.nets
echo " - done."

echo -n "    STM (small tx)"
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 1 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 2 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 3 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 4 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 5 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 6 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 7 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 8 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 9 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 10 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 11 inputs/200000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 12 inputs/200000.nets
echo " - done."

echo -n "    OHUA"
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 1 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 2 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 3 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 4 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 5 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 6 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 7 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 8 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 9 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 10 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 11 inputs/200000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 12 inputs/200000.nets
echo " - done."

echo -n "    OHUA (unsafe)"
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 1 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 2 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 3 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 4 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 5 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 6 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 7 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 8 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 9 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 10 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 11 inputs/200000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simmedium" --swaps 15000 --temperature 2000 --max-steps 64 --threads 12 inputs/200000.nets
echo " - done."


# run simlarge
echo "[run] Running simlarge"
echo -n "    SEQ"
target/release/sequential --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 inputs/400000.nets
echo " - done."

echo -n "    STM"
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 1 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 2 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 3 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 4 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 5 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 6 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 7 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 8 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 9 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 10 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 11 inputs/400000.nets
echo -n "."
target/release/stm --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 12 inputs/400000.nets
echo " - done."

echo -n "    STM"
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 1 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 2 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 3 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 4 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 5 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 6 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 7 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 8 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 9 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 10 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 11 inputs/400000.nets
echo -n "."
target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 12 inputs/400000.nets
echo " - done."

echo -n "    OHUA"
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 1 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 2 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 3 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 4 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 5 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 6 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 7 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 8 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 9 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 10 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 11 inputs/400000.nets
echo -n "."
target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 12 inputs/400000.nets
echo " - done."

echo -n "    OHUA (unsafe)"
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 1 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 2 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 3 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 4 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 5 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 6 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 7 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 8 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 9 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 10 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 11 inputs/400000.nets
echo -n "."
target/release/ohua_unsafe --runs $RUNS --json --outdir "$TODAY-simlarge" --swaps 15000 --temperature 2000 --max-steps 128 --threads 12 inputs/400000.nets
echo " - done."


## run native
#echo "[run] Running native"
#echo -n "    SEQ"
#target/release/sequential --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 inputs/2500000.nets
#echo " - done."

#echo -n "    STM"
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 1 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 2 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 3 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 4 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 5 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 6 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 7 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 8 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 9 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 10 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 11 inputs/2500000.nets
#echo -n "."
#target/release/stm --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 12 inputs/2500000.nets
#echo " - done."

#echo -n "    STM (small tx)"
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 1 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 2 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 3 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 4 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 5 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 6 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 7 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 8 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 9 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 10 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 11 inputs/2500000.nets
#echo -n "."
#target/release/stm_small_tx --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 12 inputs/2500000.nets
#echo " - done."

#echo -n "    OHUA"
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 1 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 2 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 3 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 4 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 5 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 6 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 7 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 8 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 9 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 10 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 11 inputs/2500000.nets
#echo -n "."
#target/release/ohua_futures --runs $RUNS --json --outdir "$TODAY-native" --swaps 15000 --temperature 2000 --max-steps 6000 --threads 12 inputs/2500000.nets
#echo " - done."


