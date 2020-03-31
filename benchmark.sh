#!/bin/bash

set -e

# the list of benchmarks
# declare -a benches=("labyrinth" "intruder" "genome" "kmeans")
declare -a benches=("genome")
TODAY=`date +%Y-%m-%d`

echo "################################################################################"
echo "#               Ohua-STM benchmark suite               #"
echo "################################################################################"
echo ""

echo "This script will run the full ohua-stm benchmark suite."

echo -n "[init] Cleaning intermediate output files..."
for bench in "${benches[@]}"
do
    cd $bench
    cargo --quiet clean
    rm -f Cargo.lock
    cd ..
done
echo " done."

echo "[init] Building binaries"
for bench in "${benches[@]}"
do
    cd $bench
    echo -n "    $bench."
    cargo --quiet build --release --bin sequential --features "cli"
    echo -n "."
    cargo --quiet build --release --bin ohua_futures --features "cli ohua future"
    echo -n "."
    cargo --quiet build --release --bin stm --features "cli transactional"
    echo " done."
    cd ../
done

echo ""
echo ""
echo "[run] Commencing benchmark run"

# BENCH: labyrinth #########################################################################
cd labyrinth
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'labyrinth'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth" --runs 30 inputs/random-x32-y32-z3-n96.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 2 -s 2 --threads 1 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 4 -s 4 --threads 2 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 6 -s 6 --threads 3 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 8 -s 8 --threads 4 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 10 -s 10 --threads 5 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 12 -s 12 --threads 6 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 14 -s 14 --threads 7 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 16 -s 16 --threads 8 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 18 -s 18 --threads 9 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 20 -s 20 --threads 10 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 22 -s 22 --threads 11 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 24 -s 24 --threads 12 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 26 -s 26 --threads 13 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 28 -s 28 --threads 14 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 30 -s 30 --threads 15 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 32 -s 32 --threads 16 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 34 -s 34 --threads 17 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 36 -s 36 --threads 18 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 38 -s 38 --threads 19 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 40 -s 40 --threads 20 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 42 -s 42 --threads 21 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 44 -s 44 --threads 22 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 46 -s 46 --threads 23 inputs/random-x32-y32-z3-n96.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth" --runs 30 -f 48 -s 48 --threads 24 inputs/random-x32-y32-z3-n96.txt
echo -n "."
echo " done."


echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'labyrinth+'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth+" --runs 30 inputs/random-x48-y48-z3-n64.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 2 -s 2 --threads 1 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 4 -s 4 --threads 2 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 6 -s 6 --threads 3 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 8 -s 8 --threads 4 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 10 -s 10 --threads 5 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 12 -s 12 --threads 6 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 14 -s 14 --threads 7 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 16 -s 16 --threads 8 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 18 -s 18 --threads 9 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 20 -s 20 --threads 10 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 22 -s 22 --threads 11 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 24 -s 24 --threads 12 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 26 -s 26 --threads 13 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 28 -s 28 --threads 14 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 30 -s 30 --threads 15 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 32 -s 32 --threads 16 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 34 -s 34 --threads 17 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 36 -s 36 --threads 18 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 38 -s 38 --threads 19 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 40 -s 40 --threads 20 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 42 -s 42 --threads 21 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 44 -s 44 --threads 22 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 46 -s 46 --threads 23 inputs/random-x48-y48-z3-n64.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth+" --runs 30 -f 48 -s 48 --threads 24 inputs/random-x48-y48-z3-n64.txt
echo -n "."
echo " done."


echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'labyrinth++'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-labyrinth++" --runs 30 inputs/random-x512-y512-z7-n512.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 2 -s 2 --threads 1 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 4 -s 4 --threads 2 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 6 -s 6 --threads 3 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 8 -s 8 --threads 4 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 10 -s 10 --threads 5 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 12 -s 12 --threads 6 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 14 -s 14 --threads 7 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 16 -s 16 --threads 8 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 18 -s 18 --threads 9 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 20 -s 20 --threads 10 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 22 -s 22 --threads 11 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 24 -s 24 --threads 12 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 26 -s 26 --threads 13 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 28 -s 28 --threads 14 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 30 -s 30 --threads 15 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 32 -s 32 --threads 16 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 34 -s 34 --threads 17 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 36 -s 36 --threads 18 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 38 -s 38 --threads 19 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 40 -s 40 --threads 20 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 42 -s 42 --threads 21 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 44 -s 44 --threads 22 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 46 -s 46 --threads 23 inputs/random-x512-y512-z7-n512.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-labyrinth++" --runs 30 -f 48 -s 48 --threads 24 inputs/random-x512-y512-z7-n512.txt
echo -n "."
echo " done."
cd ..

# BENCH: intruder ##########################################################################
cd intruder
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'intruder'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-intruder" --runs 30 -n 2048 -l 4 -a 10 -s 1
echo " - done!"

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
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'intruder+'"
echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-intruder+" --runs 30 -n 4096 -l 16 -a 10 -s 1
echo " - done!"

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
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'intruder++'"
echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1
echo " - done!"


echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-intruder++" --runs 30 -n 262144 -l 128 -a 10 -s 1 --threads 24
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
cd ..

# BENCH: genome ############################################################################

cd genome
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'genome'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-genome" --runs 30 -g 256 -s 16 -n 16384
echo " - done!"

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
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'genome+'"
echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-genome+" --runs 30 -g 510 -s 32 -n 32768
echo " - done!"

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
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'genome++'"
echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-genome++" --runs 30 -g 16384 -s 64 -n 16777216
echo " - done!"


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
cd ..


# BENCH: kmeans ############################################################################

cd kmeans
echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'kmeans-high'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-kmeans-high" --runs 30 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high" --runs 30  -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 1 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 2 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 3 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 4 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 5 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 6 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 7 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 8 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 9 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 10 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 11 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 12 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 13 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 14 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 15 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 16 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 17 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 18 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 19 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 20 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 21 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 22 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 23 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high" --runs 30 --threads 24 -n 15 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
echo " done."


echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'kmeans-high+'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-kmeans-high+" --runs 30 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high+" --runs 30  -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 1 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 2 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 3 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 4 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 5 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 6 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 7 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 8 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 9 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 10 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 11 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 12 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 13 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 14 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 15 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 16 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 17 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 18 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 19 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 20 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 21 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 22 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 23 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high+" --runs 30 --threads 24 -n 15 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
echo " done."

echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'kmeans-high++'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-kmeans-high++" --runs 30 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-high++" --runs 30  -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 1 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 2 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 3 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 4 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 5 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 6 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 7 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 8 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 9 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 10 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 11 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 12 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 13 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 14 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 15 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 16 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 17 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 18 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 19 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 20 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 21 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 22 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 23 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-high++" --runs 30 --threads 24 -n 15 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
echo " done."

echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'kmeans-low'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-kmeans-low" --runs 30 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low" --runs 30  -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 1 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 2 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 3 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 4 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 5 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 6 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 7 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 8 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 9 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 10 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 11 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 12 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 13 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 14 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 15 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 16 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 17 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 18 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 19 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 20 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 21 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 22 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 23 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low" --runs 30 --threads 24 -n 40 -t 0.05 inputs/random-n2048-d16-c16.txt
echo -n "."
echo " done."


echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'kmeans-low+'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-kmeans-low+" --runs 30 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low+" --runs 30  -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 1 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 2 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 3 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 4 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 5 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 6 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 7 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 8 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 9 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 10 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 11 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 12 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 13 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 14 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 15 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 16 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 17 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 18 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 19 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 20 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 21 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 22 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 23 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low+" --runs 30 --threads 24 -n 40 -t 0.05 inputs/random-n16384-d24-c16.txt
echo -n "."
echo " done."

echo "[info] Current time: $(date)."
echo "[run] Running benchmark 'kmeans-low++'"

echo -n "    Measuring sequential baseline"
target/release/sequential --json --outdir "$TODAY-kmeans-low++" --runs 30 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo " - done!"

echo -n "    Running STM"
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 1
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 2
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 3
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 4
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 5
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 6
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 7
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 8
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 9
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 10
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 11
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 12
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 13
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 14
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 15
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 16
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 17
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 18
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 19
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 20
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 21
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 22
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 23
echo -n "."
target/release/stm --json --outdir "$TODAY-kmeans-low++" --runs 30  -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads 24
echo -n "."
echo " done."


echo -n "    Running ohua-futures"
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 1 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 2 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 3 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 4 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 5 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 6 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 7 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 8 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 9 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 10 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 11 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 12 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 13 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 14 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 15 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 16 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 17 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 18 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 19 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 20 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 21 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 22 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 23 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
target/release/ohua_futures --json --outdir "$TODAY-kmeans-low++" --runs 30 --threads 24 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt
echo -n "."
echo " done."
cd ..


echo "[done] Current time: $(date)."
