#!/usr/bin/env bash
set -euo pipefail

echo "Starting benchmarks at $(date)"
TODAY=`date +%Y-%m-%d`

echo "----------------------------------[ labyrinth ]----------------------------------"
echo "Building binaries"
cd labyrinth
cargo build --quiet --release --bin sequential --features "cli"
cargo build --quiet --release --bin stm --features "cli transactional"
cd ..

echo "Running labyrinth"
labyrinth/target/release/sequential labyrinth/inputs/random-x256-y256-z5-n256.txt --json --outdir $TODAY-results/labyrinth/ --runs 30

for tc in {1..12}
do
    labyrinth/target/release/stm labyrinth/inputs/random-x256-y256-z5-n256.txt --json --outdir $TODAY-results/labyrinth/ --runs 30 --threads $tc
done

# optimal labyrinth frequencies
declare -a labtcs=(1 2 3 4 5 6 7 8 9 10 11 12)
declare -a labfre=(1 3 4 6 7 8 10 11 13 14 15 17)
cd labyrinth_new_compiler
for ((i=0; i < ${#labtcs[@]}; i++))
do
    sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = ${labtcs[$i]}/" src/generated.rs
    sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = ${labfre[$i]}/" src/generated.rs
    cargo build --quiet --release

    target/release/labyrinth_new_compiler ../labyrinth/inputs/random-x256-y256-z5-n256.txt --json --outdir ../$TODAY-results/labyrinth/ --runs 30
done
cd ..
# labyrinth w/o pipelining
#cd labyrinth_no_pipelining
#for ((i=0; i < ${#labtcs[@]}; i++))
#do
    #sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = ${labtcs[$i]}/" src/generated.rs
    #sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = ${labfre[$i]}/" src/generated.rs
    #cargo build --quiet --release

    #target/release/labyrinth_no_pipelining ../labyrinth/inputs/random-x256-y256-z5-n256.txt --json --outdir ../$TODAY-results/labyrinth/ --runs 30
#done
#cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "-----------------------------------[ kmeans ]------------------------------------"
echo "Building binaries"
cd kmeans
cargo build --quiet --release --bin sequential --features "cli"
cargo build --quiet --release --bin stm --features "cli transactional"
cd ..

echo "Running kmeans"
kmeans/target/release/sequential --json --outdir $TODAY-results/kmeans/ --runs 30 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt

for tc in {1..12}
do
    kmeans/target/release/stm --json --outdir $TODAY-results/kmeans/ --runs 30 -n 40 -t 0.00001 inputs/random-n65536-d32-c16.txt --threads $tc
done

cd kmeans_new_compiler
for tc in {1..12}
do
    sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tc/" src/generated.rs
    cargo build --release --quiet
    target/release/kmeans_new_compiler ../kmeans/inputs/random-n65536-d32-c16.txt --json --outdir ../$TODAY-results/kmeans/ --runs 30 -n 40 -t 0.00001
done
cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
#echo "----------------------------------[ intruder ]-----------------------------------"
#echo "Building binaries"
#cd intruder
#cargo build --quiet --release --bin sequential --features "cli"
#cargo build --quiet --release --bin stm --features "cli transactional"
#cd ..

#echo "Running intruder"
#intruder/target/release/sequential --json --outdir $TODAY-results/intruder/ --runs 30 -a 10 -l 128 -n 262144 -s 1

#for tc in {1..12}
#do
    #intruder/target/release/stm --json --outdir $TODAY-results/intruder/ --runs 30 -a 10 -l 128 -n 262144 -s 1 --threads $tc
#done

#cd intruder_new_compiler
#for tc in {1..12}
#do
    #sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tc/" src/generated.rs
    #cargo build --release --quiet
    #target/release/intruder_new_compiler --json --outdir ../$TODAY-results/intruder/ --runs 30 -a 10 -l 128 -n 262144 -s 1
#done
#cd ..


#echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "-----------------------------------[ genome ]------------------------------------"
echo "Building binaries"
cd genome
cargo build --quiet --release --bin sequential --features "cli"
cargo build --quiet --release --bin stm --features "cli transactional"
cd ..

echo "Running genome"
genome/target/release/sequential --json --outdir $TODAY-results/genome/ --runs 30 -g 510 -s 32 -n 32768

for tc in {1..12}
do
    genome/target/release/stm --json --outdir $TODAY-results/genome/ --runs 30 --threads $tc -g 510 -s 32 -n 32768
done

cd genome_new_compiler
for tc in {1..12}
do
    sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tc/" src/generated.rs
    cargo build --release --quiet
    target/release/genome_new_compiler --json --outdir ../$TODAY-results/genome/ --runs 30 -g 510 -s 32 -n 32768
done
cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "-----------------------------------[ canneal ]-----------------------------------"
echo "Building binaries"
cd canneal
cargo build --quiet --release --bin sequential --features "cli"
cargo build --quiet --release --bin stm_small_tx --features "cli transactional less_tx"
cd ..

echo "Running canneal"
canneal/target/release/sequential --runs 30 --json --outdir $TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128 canneal/inputs/400000.nets

for tc in {1..12}
do
    canneal/target/release/stm_small_tx --runs 30 --json --outdir $TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128 canneal/inputs/400000.nets --threads $tc
done

cd canneal_new_compiler
sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = 100/" src/generated.rs
for tc in {1..12}
do
    sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tc/" src/generated.rs
    cargo build --release --quiet
    target/release/canneal_new_compiler ../canneal/inputs/400000.nets -runs 30 --json --outdir ../$TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128
done
cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "----------------------------------[ delaunay ]-----------------------------------"
echo "Building binaries"
cd yada
cargo build --quiet --release

echo "Running delaunay"
target/release/seq-yada inputs/ttimeu10000.2 --json --runs 30 --outdir ../$TODAY-results/delaunay/ 

# ohua first here
cd ohua
declare -a delaunayfreq=(100 300 500 700 1000 3000 5000 7000 10000)

for tc in {1..12}
do
    sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tc/" src/generated.rs
    for fre in ${delaunayfreq[@]}
    do
        sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = $fre/" src/generated.rs
        cargo build --release --quiet

        ../target/release/ohua-yada ../inputs/ttimeu10000.2 --json --runs 30 --outdir ../../$TODAY-results/delaunay/$fre/
    done
done
cd ..


echo "Current time: $(date)\n\n"
echo "STM Time"
for tc in {1..12}
do
    target/release/stm-yada inputs/ttimeu10000.2 --json --runs 30 --outdir ../$TODAY-results/delaunay/ --threads $tc
done
    

cd ..
echo "Finished at $(date)"


