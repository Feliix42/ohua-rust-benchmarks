#!/usr/bin/env bash
set -euo pipefail

echo "Starting benchmarks at $(date)"
TODAY=`date +%Y-%m-%d`


echo -n "[info] Building."
cd labyrinth
cargo clean
echo -n "."
cargo build --release --quiet --bin sequential --features "cli"
cd ../labyrinth_new_compiler
cargo clean
echo -n "."
cargo build --release --quiet
cd ..

cd blackscholes
cargo clean
echo -n "."
cargo build --release --quiet --bin sequential --features "cli"
cd ../blackscholes_new_compiler
cargo clean
echo -n "."
cargo build --release --quiet
cd ..

cd kmeans
cargo clean
echo -n "."
cargo build --release --quiet --bin sequential --features "cli"
cd ../kmeans_new_compiler
cargo clean
echo -n "."
cargo build --release --quiet
cd ..

echo " - done!"


mkdir -p $TODAY-sequentials/labyrinth
mkdir -p $TODAY-sequentials/blackscholes
mkdir -p $TODAY-sequentials/kmeans

echo -n "[info] Running."
labyrinth/target/release/sequential labyrinth/inputs/random-x256-y256-z5-n256.txt --json --outdir "$TODAY-sequentials/labyrinth" --runs 30
echo -n "."
labyrinth_new_compiler/target/release/labyrinth_new_compiler labyrinth/inputs/random-x256-y256-z5-n256.txt --json --outdir "$TODAY-sequentials/labyrinth" --runs 30

echo -n "."
blackscholes/target/release/sequential blackscholes/inputs/in_20M.txt --json --outdir "$TODAY-sequentials/blackscholes" --runs 30 
echo -n "."
blackscholes_new_compiler/target/release/blackscholes_new_compiler blackscholes/inputs/in_20M.txt --json --outdir "$TODAY-sequentials/blackscholes" --runs 30

echo -n "."
kmeans/target/release/sequential kmeans/inputs/random-n65536-d32-c16.txt --json --outdir "$TODAY-sequentials/kmeans" --runs 30 -n 40 -t 0.00001
echo -n "."
kmeans_new_compiler/target/release/kmeans_new_compiler kmeans/inputs/random-n65536-d32-c16.txt --json --outdir "$TODAY-sequentials/kmeans" --runs 30 -n 40 -t 0.00001
echo " - done!"


echo "Finished at $(date)"
