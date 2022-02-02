#!/usr/bin/env bash
set -e
echo "This benchmark runner will execute the sequential labyrinth implementation from the old implementation (for now) as well as the new labyrinth implementation."

echo "Starting benchmarks at $(date)"


# problem sizes
declare -a sizes=("random-x32-y32-z3-n96.txt" "random-x48-y48-z3-n64.txt" "random-x128-y128-z5-n128.txt" "random-x256-y256-z5-n256.txt" "random-x512-y512-z7-n512.txt")
TODAY=`date +%Y-%m-%d`

# clean and build
cd ../labyrinth
cargo clean
cargo build --release --quiet --bin sequential --features "cli"

cd -

for benchsize in "${sizes[@]}"
do
    IFS='-' read -ra foo <<< "$benchsize"
    sdir="${foo[1]}"
    mkdir -p $TODAY-labyrinth/$sdir

    echo "Running benchmarks for $benchsize"

    echo -n "  sequential"
    ../labyrinth/target/release/sequential ../labyrinth/inputs/$benchsize --json --outdir "$TODAY-labyrinth/$sdir" --runs 30 
    echo " - done!"

    echo -n "  new ohua version"
    
    for tcount in 1 2 3 4 5 6 7 8 9 10 11 12
    do
        sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tcount/" src/generated.rs
        for frequency in 1 2 3 4
        do
            mkdir -p $TODAY-labyrinth/$sdir/freq$frequency
            echo -n "."
            sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = $(($tcount * $frequency))/" src/generated.rs
            cargo build --release --quiet
            target/release/labyrinth_new_compiler ../labyrinth/inputs/$benchsize --json --outdir "$TODAY-labyrinth/$sdir/freq$frequency" --runs 30
        done
    done

    echo " - done!"
done

echo "Finished at $(date)"
