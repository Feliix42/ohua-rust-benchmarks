#!/usr/bin/env bash
set -e
echo "This benchmark runner will execute the sequential canneal implementation from the old implementation (for now) as well as the new canneal implementation."

echo "Starting benchmarks at $(date)"


# problem sizes
declare -a sizes=("400000.nets")
TODAY=`date +%Y-%m-%d`

# clean and build
cd ../canneal
cargo clean
cargo build --release --quiet --bin sequential --features "cli"

cd -

for benchsize in "${sizes[@]}"
do
    IFS='.' read -ra foo <<< "$benchsize"
    sdir="${foo[0]}"
    mkdir -p $TODAY-canneal/$sdir

    echo "Running sequential Ohua"
    mkdir -p $TODAY-canneal/$sdir/seq
    cargo build --release --quiet
    target/release/canneal_new_compiler ../canneal/inputs/$benchsize --json --outdir "$TODAY-canneal/$sdir/seq" --runs 30 --swaps 15000 --temperature 2000 --max-steps 128 -s

    echo "Running benchmarks for $benchsize"

    echo -n "  sequential"
    ../canneal/target/release/sequential ../canneal/inputs/$benchsize --json --outdir "$TODAY-canneal/$sdir" --runs 30 --swaps 15000 --temperature 2000 --max-steps 128
    echo " - done!"

    echo -n "  new ohua version"
    
    for tcount in 1 2 3 4 5 6 7 8 9 10 11 12
    do
        sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tcount/" src/generated.rs
        for frequency in 10 20 30 40 50 60 70 80 90 100 120 130 140 150
        do
            mkdir -p $TODAY-canneal/$sdir/freq$frequency
            echo -n "."
            # sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = $(($tcount * $frequency))/" src/generated.rs
            sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = $frequency/" src/generated.rs
            cargo build --release --quiet
            target/release/canneal_new_compiler ../canneal/inputs/$benchsize --json --outdir "$TODAY-canneal/$sdir/freq$frequency" --runs 30 --swaps 15000 --temperature 2000 --max-steps 128
        done
    done

    echo " - done!"
done

echo "Finished at $(date)"
