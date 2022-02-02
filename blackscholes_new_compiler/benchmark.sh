#!/usr/bin/env bash
set -e
echo "This benchmark runner will execute the sequential blackscholes implementation from the old implementation (for now) as well as the new blackscholes implementation."

echo "Starting benchmarks at $(date)"


# problem sizes
declare -a sizes=("in_4K_simsmall.txt" "in_16K_simmedium.txt" "in_64K_simlarge.txt" "in_10M_native.txt" "in_20M.txt")
TODAY=`date +%Y-%m-%d`

# clean and build
cd ../blackscholes
cargo clean
cargo build --release --quiet --bin sequential --features "cli"

cd -

for benchsize in "${sizes[@]}"
do
    IFS='.' read -ra foo <<< "$benchsize"
    sdir="${foo[0]}"
    mkdir -p $TODAY-blackscholes/$sdir

    echo "Running benchmarks for $benchsize"

    echo -n "  sequential"
    ../blackscholes/target/release/sequential ../blackscholes/inputs/$benchsize --json --outdir "$TODAY-blackscholes/$sdir" --runs 30 
    echo " - done!"

    echo -n "  new ohua version"
    
    for tcount in 1 2 3 4 5 6 7 8 9 10 11 12
    do
        sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tcount/" src/generated.rs
        echo -n "."
        cargo build --release --quiet
        target/release/blackscholes_new_compiler ../blackscholes/inputs/$benchsize --json --outdir "$TODAY-blackscholes/$sdir" --runs 30
    done

    echo " - done!"
done

echo "Finished at $(date)"
