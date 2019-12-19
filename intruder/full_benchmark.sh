#!/bin/bash

set -e

echo "[INFO] Running the full benchmark suite for the intruder benchmark."
echo "[INFO] Beginning cleanup of build artifacts."

rm Cargo.lock 
cargo --quiet clean

echo -n "[INFO] Building binaries..."

cargo --quiet build --release --bin sequential --features "cli"
cargo --quiet build --release --bin ohua_futures --features "cli ohua future"
cargo --quiet build --release --bin stm --features "cli transactional"

cargo --quiet build --release --bin ohua --features "cli ohua"
cargo --quiet build --release --bin ohua-split2 --features "ohua cli"
cargo --quiet build --release --bin ohua-split3 --features "ohua cli"
cargo --quiet build --release --bin ohua-split4 --features "ohua cli"
cargo --quiet build --release --bin ohua-split5 --features "ohua cli"
cargo --quiet build --release --bin ohua-split6 --features "ohua cli"
cargo --quiet build --release --bin ohua-split7 --features "ohua cli"
cargo --quiet build --release --bin ohua-split8 --features "ohua cli"
cargo --quiet build --release --bin ohua-split9 --features "ohua cli"
cargo --quiet build --release --bin ohua-split10 --features "ohua cli"
cargo --quiet build --release --bin ohua-split11 --features "ohua cli"
cargo --quiet build --release --bin ohua-split12 --features "ohua cli"
cargo --quiet build --release --bin ohua-split13 --features "ohua cli"
cargo --quiet build --release --bin ohua-split14 --features "ohua cli"
cargo --quiet build --release --bin ohua-split15 --features "ohua cli"
cargo --quiet build --release --bin ohua-split16 --features "ohua cli"
cargo --quiet build --release --bin ohua-split17 --features "ohua cli"
cargo --quiet build --release --bin ohua-split18 --features "ohua cli"
cargo --quiet build --release --bin ohua-split19 --features "ohua cli"
cargo --quiet build --release --bin ohua-split20 --features "ohua cli"
cargo --quiet build --release --bin ohua-split21 --features "ohua cli"
cargo --quiet build --release --bin ohua-split22 --features "ohua cli"
cargo --quiet build --release --bin ohua-split23 --features "ohua cli"
cargo --quiet build --release --bin ohua-split24 --features "ohua cli"

echo "done!"

TODAY=`date +%Y-%m-%d`

echo "[INFO] Running benchmark 'intruder'"

target/release/sequential --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 24
echo -n "."

echo " done."


echo -n "    Running ohua-split"
target/release/ohua --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split2 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split3 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split4 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split5 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split6 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split7 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split8 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split9 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split10 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split11 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split12 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split13 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split14 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split15 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split16 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split17 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split18 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split19 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split20 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split21 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split22 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split23 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."
target/release/ohua-split24 --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo -n "."

echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 12
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 13
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 14
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 15
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 16
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 17
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 18
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 19
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 20
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 21
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 22
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 23
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1 --threads 24
echo -n "."

echo " done."


# --------------------------------------------------- intruder+ ---------------------------------------------------
echo "[INFO] Running benchmark 'intruder+'"
target/release/sequential --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 24
echo -n "."

echo " done."


echo -n "    Running ohua-split"
target/release/ohua --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split2 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split3 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split4 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split5 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split6 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split7 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split8 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split9 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split10 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split11 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split12 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split13 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split14 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split15 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split16 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split17 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split18 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split19 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split20 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split21 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split22 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split23 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."
target/release/ohua-split24 --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo -n "."

echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 12
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 13
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 14
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 15
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 16
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 17
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 18
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 19
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 20
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 21
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 22
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 23
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1 --threads 24
echo -n "."

echo " done."


# --------------------------------------------------- intruder++ ---------------------------------------------------
echo "[INFO] Running benchmark 'intruder++'"
target/release/sequential --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1

echo -n "    Running ohua-split"
target/release/ohua --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split2 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split3 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split4 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split5 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split6 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split7 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split8 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split9 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split10 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split11 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split12 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split13 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split14 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split15 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split16 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split17 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split18 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split19 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split20 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split21 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split22 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split23 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."
target/release/ohua-split24 --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo -n "."

echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 12
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 13
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 14
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 15
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 16
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 17
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 18
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 19
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 20
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 21
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 22
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 23
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 24
echo -n "."

echo " done."

echo "Finished benchmark run at $(date)."
