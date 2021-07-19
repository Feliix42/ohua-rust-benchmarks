#!/usr/bin/env bash

set -euo pipefail

TODAY=`date +%Y-%m-%d`

echo "[init] Cleaning intermediate files"
cargo --quiet clean
rm -f Cargo.lock

echo "[init] Building binaries"
cargo --quiet build --release --bin sequential --features "cli" 
cargo --quiet build --release --bin par --features "cli"
cargo --quiet build --release --bin ohua_futures --features "cli ohua future"

echo "[run] Commencing benchmark run"
echo "[run] Running benchmark 'blackscholes'"


## small
#echo "[run] Input Set: small"
#echo -n "    Measuring sequential baseline"
#target/release/sequential --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt
#echo " - done!"


#echo -n "    Running par"
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 1
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 2
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 3
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 4
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 5
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 6
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 7
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 8
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 9
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 10
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 11
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 12
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 13
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 14
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 15
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 16
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 17
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 18
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 19
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 20
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 21
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 22
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 23
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 24
#echo " done!"


#echo -n "    Running ohua"
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 1
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 2
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 3
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 4
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 5
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 6
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 7
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 8
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 9
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 10
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 11
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 12
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 13
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 14
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 15
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 16
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 17
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 18
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 19
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 20
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 21
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 22
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 23
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-small" --runs 30 inputs/in_4K_simsmall.txt --threads 24
#echo " done!"

## medium
#echo "[run] Input Set: medium"
#echo -n "    Measuring sequential baseline"
#target/release/sequential --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt
#echo " - done!"


#echo -n "    Running par"
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 1
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 2
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 3
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 4
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 5
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 6
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 7
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 8
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 9
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 10
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 11
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 12
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 13
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 14
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 15
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 16
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 17
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 18
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 19
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 20
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 21
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 22
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 23
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 24
#echo " done!"


#echo -n "    Running ohua"
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 1
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 2
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 3
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 4
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 5
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 6
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 7
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 8
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 9
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 10
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 11
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 12
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 13
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 14
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 15
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 16
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 17
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 18
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 19
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 20
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 21
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 22
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 23
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-medium" --runs 30 inputs/in_16K_simmedium.txt --threads 24
#echo " done!"


## large
#echo "[run] Input Set: large"
#echo -n "    Measuring sequential baseline"
#target/release/sequential --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt
#echo " - done!"


#echo -n "    Running par"
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 1
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 2
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 3
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 4
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 5
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 6
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 7
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 8
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 9
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 10
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 11
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 12
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 13
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 14
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 15
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 16
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 17
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 18
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 19
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 20
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 21
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 22
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 23
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 24
#echo " done!"


#echo -n "    Running ohua"
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 1
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 2
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 3
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 4
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 5
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 6
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 7
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 8
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 9
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 10
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 11
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 12
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 13
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 14
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 15
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 16
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 17
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 18
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 19
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 20
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 21
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 22
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 23
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-large" --runs 30 inputs/in_64K_simlarge.txt --threads 24
#echo " done!"



# native
echo "[run] Input Set: native"
echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt
echo " - done!"


echo -n "    Running par"
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 1
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 2
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 3
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 4
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 5
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 6
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 7
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 8
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 9
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 10
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 11
echo -n "."
target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 12
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 13
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 14
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 15
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 16
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 17
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 18
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 19
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 20
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 21
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 22
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 23
#echo -n "."
#target/release/par --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 24
echo " done!"


echo -n "    Running ohua"
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 1
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 2
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 3
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 4
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 5
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 6
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 7
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 8
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 9
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 10
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 11
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 12
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 13
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 14
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 15
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 16
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 17
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 18
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 19
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 20
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 21
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 22
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 23
#echo -n "."
#target/release/ohua_futures --json --outdir "$TODAY-blackscholes-native" --runs 30 inputs/in_20M.txt --threads 24
echo " done!"

echo "[done] Current time: $(date)."
