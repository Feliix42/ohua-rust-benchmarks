#!/bin/bash

set -e

echo "[INFO] Running the full benchmark suite for the genome benchmark."
echo "[INFO] Beginning cleanup of build artifacts."

rm Cargo.lock 
cargo --quiet clean

echo -n "[INFO] Building binaries..."

cargo --quiet build --release --bin sequential --features "cli"
cargo --quiet build --release --bin ohua_futures --features "cli ohua future"
cargo --quiet build --release --bin stm --features "cli transactional"

echo "done!"

TODAY=`date +%Y-%m-%d`

echo "[INFO] Running benchmark 'genome'"

target/release/sequential --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 24
echo -n "."

echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 12
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 13
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 14
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 15
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 16
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 17
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 18
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 19
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 20
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 21
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 22
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 23
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384 --threads 24
echo -n "."

echo " done."


# --------------------------------------------------- genome+ ---------------------------------------------------
echo "[INFO] Running benchmark 'genome+'"
target/release/sequential --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 24
echo -n "."

echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 12
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 13
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 14
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 15
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 16
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 17
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 18
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 19
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 20
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 21
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 22
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 23
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768 --threads 24
echo -n "."

echo " done."


# --------------------------------------------------- genome++ ---------------------------------------------------
echo "[INFO] Running benchmark 'genome++'"
target/release/sequential --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216


echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 24
echo -n "."


echo " done."
echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 12
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 13
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 14
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 15
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 16
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 17
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 18
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 19
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 20
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 21
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 22
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 23
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216 --threads 24
echo -n "."

echo " done."

echo "Finished benchmark run at $(date)."
