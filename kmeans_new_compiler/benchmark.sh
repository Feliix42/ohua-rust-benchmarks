#!/usr/bin/env bash
set -e
echo "This benchmark runner will execute the sequential kmeans implementation from the old implementation (for now) as well as the new kmeans implementation."

echo "Starting benchmarks at $(date)"


# problem sizes
declare -a sizes=("random-n65536-d32-c16.txt")
TODAY=`date +%Y-%m-%d`

# reset
#sed -i "s/while runs < 200 { \/\//while runs < 500/" ../kmeans/src/bin/seq.rs
#sed -i -e 's/\/\/current_delta > threshold/current_delta > threshold/g' -e 's/runs < 200/\/\/ runs < 200/g' src/types.rs

# clean and build
cd ../kmeans
cargo clean
cargo build --release --quiet --bin sequential --features "cli"

cd -

for benchsize in "${sizes[@]}"
do
    IFS='.' read -ra foo <<< "$benchsize"
    sdir="${foo[0]}"
    mkdir -p $TODAY-kmeans-low/$sdir

    echo "Running benchmarks for $benchsize"

    echo -n "  sequential"
    ../kmeans/target/release/sequential ../kmeans/inputs/$benchsize --json --outdir "$TODAY-kmeans-low/$sdir" --runs 30 -n 40 -t 0.00001
    echo " - done!"

    echo -n "  new ohua version"
    
    for tcount in 1 2 3 4 5 6 7 8 9 10 11 12
    do
        sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tcount/" src/generated.rs
        echo -n "."
        cargo build --release --quiet
        target/release/kmeans_new_compiler ../kmeans/inputs/$benchsize --json --outdir "$TODAY-kmeans-low/$sdir" --runs 30 -n 40 -t 0.00001
    done

    echo " - done!"
done

####################### 200 iterations ####################

sed -i "s/while runs < 500/while runs < 200 { \/\//" ../kmeans/src/bin/seq.rs
sed -i -e 's/current_delta > threshold/\/\/current_delta > threshold/g' -e 's/\/\/ runs < 200/runs < 200/g' src/types.rs


# clean and build
cd ../kmeans
cargo clean
cargo build --release --quiet --bin sequential --features "cli"

cd -

for benchsize in "${sizes[@]}"
do
    IFS='.' read -ra foo <<< "$benchsize"
    sdir="${foo[0]}"
    mkdir -p $TODAY-kmeans-low-200/$sdir

    echo "Running benchmarks for $benchsize"

    echo -n "  sequential"
    ../kmeans/target/release/sequential ../kmeans/inputs/$benchsize --json --outdir "$TODAY-kmeans-low-200/$sdir" --runs 30 -n 40 -t 0.00001
    echo " - done!"

    echo -n "  new ohua version"
    
    for tcount in 1 2 3 4 5 6 7 8 9 10 11 12
    do
        sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tcount/" src/generated.rs
        echo -n "."
        cargo build --release --quiet
        target/release/kmeans_new_compiler ../kmeans/inputs/$benchsize --json --outdir "$TODAY-kmeans-low-200/$sdir" --runs 30 -n 40 -t 0.00001
    done

    echo " - done!"
done

# reset
sed -i "s/while runs < 200 { \/\//while runs < 500/" ../kmeans/src/bin/seq.rs
sed -i -e 's/\/\/current_delta > threshold/current_delta > threshold/g' -e 's/runs < 200/\/\/ runs < 200/g' src/types.rs

echo "Finished at $(date)"
