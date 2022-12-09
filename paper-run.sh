#!/usr/bin/env bash
set -euo pipefail

echo "Starting benchmarks at $(date)"
TODAY=`date +%Y-%m-%d`

threadrange=$(seq 2 2 10) # 1 - 10 threads
RUNS=30
#threadrange=$(seq 1 10) # 1 - 10 threads
#RUNS=20

RESPATH=/opt/benchmarks

echo "----------------------------------[ labyrinth ]----------------------------------"
echo "Building binaries"
cd labyrinth
./ohua-compile.sh

cargo build --quiet --release --bin sequential
cargo build --quiet --release --bin stm --features "transactional"
cargo build --quiet --release --bin dstm --features "transactional"
cargo build --quiet --release --bin ohua
cd ..

echo "Running labyrinth"
labyrinth/target/release/sequential labyrinth/inputs/random-x512-y512-z7-n512.txt --json --outdir $RESPATH/$TODAY-results/labyrinth/ --runs $RUNS

for tc in ${threadrange[@]}
do
    labyrinth/target/release/stm labyrinth/inputs/random-x512-y512-z7-n512.txt --json --outdir $RESPATH/$TODAY-results/labyrinth/ --runs $RUNS --threads $tc
    labyrinth/target/release/dstm labyrinth/inputs/random-x512-y512-z7-n512.txt --json --outdir $RESPATH/$TODAY-results/labyrinth/ --runs $RUNS --threads $tc
done

# optimal labyrinth frequencies
cd labyrinth
for tc in ${threadrange[@]}
do
    sed -i "s/data-parallelism: [0-9]\+/data-parallelism: ${labtcs[$i]}/" ohua-config.yaml
    BS=$(python -c "print(round(${labtcs[$i]} * 2.4))")
    echo "Frequency: $BS"
    sed -i "s/amorphous: [0-9]\+/amorphous: $BS/" ohua-config.yaml
    ./ohua-compile.sh

    cargo build --quiet --release --bin ohua

    target/release/ohua inputs/random-x512-y512-z7-n512.txt --json --outdir $RESPATH/$TODAY-results/labyrinth-non-opt/ --runs $RUNS --threads $tc --frequency $BS --runtime Ohua
    target/release/ohua inputs/random-x512-y512-z7-n512.txt --json --outdir $RESPATH/$TODAY-results/labyrinth/ --runs $RUNS --threads $tc --frequency $BS --runtime OhuaZeroClone
done
cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "-----------------------------------[ kmeans ]------------------------------------"
echo "Building binaries"
cd kmeans
cargo build --quiet --release --bin sequential --features "cli"
cargo build --quiet --release --bin stm --features "cli transactional"
cargo build --quiet --release --bin dstm --features "cli transactional"
cd ..

echo "Running kmeans"
kmeans/target/release/sequential --json --outdir $RESPATH/$TODAY-results/kmeans/ --runs $RUNS -n 40 -t 0.00001 kmeans/inputs/random-n65536-d32-c16.txt

for tc in ${threadrange[@]}
do
    kmeans/target/release/stm --json --outdir $RESPATH/$TODAY-results/kmeans/ --runs $RUNS -n 40 -t 0.00001 kmeans/inputs/random-n65536-d32-c16.txt --threads $tc
    kmeans/target/release/dstm --json --outdir $RESPATH/$TODAY-results/kmeans/ --runs $RUNS -n 40 -t 0.00001 kmeans/inputs/random-n65536-d32-c16.txt --threads $tc
done

cd kmeans_new_compiler
for tc in ${threadrange[@]}
do
    sed -i "s/data-parallelism: [0-9]\+/data-parallelism: $tc/" ohua-config.yaml
    ./ohua-compile.sh
    cargo build --release --quiet

    target/release/kmeans_new_compiler ../kmeans/inputs/random-n65536-d32-c16.txt --json --outdir $RESPATH/$TODAY-results/kmeans/ --runs $RUNS -n 40 -t 0.00001 --threadcount $tc
done
cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "-----------------------------------[ genome ]------------------------------------"
echo "Building binaries"
cd genome
./ohua-compile.sh

cargo build --quiet --release --bin sequential
cargo build --quiet --release --bin stm
cargo build --quiet --release --bin dstm
cargo build --quiet --release --bin ohua
cd ..

echo "Running genome"
genome/target/release/sequential --json --outdir $RESPATH/$TODAY-results/genome/ --runs $RUNS -g 16384 -s 64 -n 16777216

for tc in ${threadrange[@]}
do
    genome/target/release/stm --json --outdir $RESPATH/$TODAY-results/genome/ --runs $RUNS --threads $tc -g 16384 -s 64 -n 16777216
    genome/target/release/dstm --json --outdir $RESPATH/$TODAY-results/genome/ --runs $RUNS --threads $tc -g 16384 -s 64 -n 16777216

    cd genome
    sed -i "s/data-parallelism: [0-9]\+/data-parallelism: $tc/" ohua-config.yaml
    ./ohua-compile.sh
    cargo --quiet build --release --bin ohua
    cd ..

    genome/target/release/ohua --json --outdir $RESPATH/$TODAY-results/genome/ --runs $RUNS --threads $tc -g 16384 -s 64 -n 16777216
done


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "-----------------------------------[ canneal ]-----------------------------------"
echo "Building binaries"
cd canneal
cargo build --quiet --release --bin sequential --features "cli"
cargo build --quiet --release --bin stm_small_tx --features "cli transactional less_tx"
cargo build --quiet --release --bin dstm --features "cli transactional less_tx"
cd ..

echo "Running canneal"
canneal/target/release/sequential --runs $RUNS --json --outdir $RESPATH/$TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128 canneal/inputs/400000.nets

for tc in ${threadrange[@]}
do
    canneal/target/release/stm_small_tx --runs $RUNS --json --outdir $RESPATH/$TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128 canneal/inputs/400000.nets --threads $tc
    canneal/target/release/dstm --runs $RUNS --json --outdir $RESPATH/$TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128 canneal/inputs/400000.nets --threads $tc
done

cd canneal_new_compiler
sed -i "s/amorphous: [0-9]\+/amorphous: 100/" ohua-config.yaml
for tc in ${threadrange[@]}
do
    BATCHSIZE=$(($tc * 100))
    sed -i "s/data-parallelism: [0-9]\+/data-parallelism: $tc/" ohua-config.yaml
    sed -i "s/amorphous: [0-9]\+/amorphous: $BATCHSIZE/" ohua-config.yaml
    ./ohua-compile.sh
    cargo build --release --quiet

    target/release/canneal_new_compiler ../canneal/inputs/400000.nets --runs $RUNS --json --outdir $RESPATH/$TODAY-results/canneal/ --swaps 15000 --temperature 2000 --max-steps 128

    mv $RESPATH/$TODAY-results/canneal/ohua-r$RUNS-log.json $RESPATH/$TODAY-results/canneal/ohua-t$tc-r$RUNS-log.json
    sed -i "s/insert_threadcount/$tc/g" $RESPATH/$TODAY-results/canneal/ohua-t$tc-r$RUNS-log.json
    sed -i "s/insert_freq/$BATCHSIZE/g" $RESPATH/$TODAY-results/canneal/ohua-t$tc-r$RUNS-log.json
done
cd ..


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
#echo "----------------------------------[ delaunay ]-----------------------------------"
#echo "Building binaries"
#cd yada
#cd seq
#cargo build --quiet --release
#cd ..
#cd stm
#cargo build --quiet --release
#cd ..

#echo "Running delaunay"
#target/release/seq-yada inputs/ttimeu10000.2 --json --runs $RUNS --outdir ../$TODAY-results/delaunay/ 

## ohua first here
#cd ohua
#declare -a delaunayfreq=(100 300 500 700 1000 3000 5000 7000 10000)

#for tc in ${threadrange[@]}
#do
    #sed -i "s/THREADCOUNT: usize = [0-9]\+/THREADCOUNT: usize = $tc/" src/generated.rs
    #for fre in ${delaunayfreq[@]}
    #do
        #sed -i "s/FREQUENCY: usize = [0-9]\+/FREQUENCY: usize = $fre/" src/generated.rs
        #cargo build --release --quiet

        #../target/release/ohua-yada ../inputs/ttimeu10000.2 --json --runs $RUNS --outdir ../../$TODAY-results/delaunay/$fre/
    #done
#done
#cd ..


#echo "Current time: $(date)\n\n"
#echo "STM Time"
#for tc in ${threadrange[@]}
#do
    #target/release/stm-yada inputs/ttimeu10000.2 --json --runs $RUNS --outdir ../$TODAY-results/delaunay/ --threads $tc
#done
#cd ..

#echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
echo "----------------------------------[ intruder ]-----------------------------------"
echo "Building binaries"
cd intruder
./ohua-compile.sh
cargo build --quiet --release --bin bench
cd ..

echo "Running intruder"
# TODO: Change
##intruder/target/release/sequential --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 128 -n 262144 -s 1
intruder/target/release/bench --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 16 -n 4096 -s 1 --runtime Seq

for tc in ${threadrange[@]}
do
    ###intruder/target/release/stm --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 128 -n 262144 -s 1 --threads $tc
    intruder/target/release/bench --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 16 -n 4096 -s 1 --threads $tc --runtime STM
    intruder/target/release/bench --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 16 -n 4096 -s 1 --threads $tc --runtime DSTM
done

cd intruder
for tc in ${threadrange[@]}
do
    sed -i "s/data-parallelism: [0-9]\+/data-parallelism: $tc/" ohua-config.yaml
    ./ohua-compile.sh
    cargo build --release --quiet --bin bench

    #target/release/intruder_new_compiler --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 128 -n 262144 -s 1
    #target/release/bench --json --outdir $RESPATH/$TODAY-results/intruder++/ --runs $RUNS -a 10 -l 128 -n 262144 -s 1 -r Ohua --threads $tc
    target/release/bench --json --outdir $RESPATH/$TODAY-results/intruder/ --runs $RUNS -a 10 -l 16 -n 4096 -s 1 -r Ohua --threads $tc
done

cd ..


echo "Current time: $(date)\n\n"
    
########################################################################################
########################################################################################
########################################################################################
echo "--------------------------------[ blackscholes ]---------------------------------"

# parameters
sdir="in_40M"
mkdir -p $RESPATH/$TODAY-blackscholes/$sdir

echo "Building binaries"
# clean and build
cd blackscholes_new_compiler
./ohua-compile.sh
#cargo --quiet clean
cd ../blackscholes
#cargo --quiet clean
cargo build --release --quiet --bin sequential --features "cli"
cargo build --release --quiet --bin par --features "cli"
cargo build --release --quiet --bin locked --features "cli"
cargo build --release --quiet --bin parlist --features "cli"

echo "Running benchmarks"

echo -n "  sequential"
target/release/sequential inputs/$sdir.txt --json --outdir "$RESPATH/$TODAY-blackscholes/$sdir" --runs $RUNS 
echo " - done!"

echo -n "  threaded version"
for tcount in ${threadrange[@]}
do
    echo -n "."
    target/release/par inputs/$sdir.txt --json --outdir "$RESPATH/$TODAY-blackscholes/$sdir" --runs $RUNS --threads $tcount
    target/release/locked inputs/$sdir.txt --json --outdir "$RESPATH/$TODAY-blackscholes/$sdir" --runs $RUNS --threads $tcount
    target/release/parlist inputs/$sdir.txt --json --outdir "$RESPATH/$TODAY-blackscholes/$sdir" --runs $RUNS --threads $tcount
done
echo " - done!"

cd ../blackscholes_new_compiler

echo -n "  new ohua version"
for tcount in ${threadrange[@]}
do
    sed -i "s/data-parallelism: [0-9]\+/data-parallelism: $tcount/" ohua-config.yaml
    ./ohua-compile.sh
    cargo build --release --quiet

    target/release/blackscholes_new_compiler ../blackscholes/inputs/$sdir.txt --json --outdir "$RESPATH/$TODAY-blackscholes/$sdir" --runs $RUNS
done
echo " - done!"


echo "Current time: $(date)\n\n"
########################################################################################
########################################################################################
########################################################################################
#echo "----------------------------------[ vacation ]-----------------------------------"
#echo "Building binaries"
#cd vacation
#./ohua-compile.sh
#cd seq
#cargo build --quiet --release
#cd ../stm
#cargo build --quiet --release
#cd ../dstm
#cargo build --quiet --release
#cd ../..

#echo "Running vacation"
##vacation/target/release/seq-vacation --json --outdir $TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 16384 -t 4096
##vacation/target/release/seq-vacation --json --outdir $TODAY-results/vacation-high/ --runs $RUNS -n 2 -q 60 -u 90 -r 16384 -t 4096
## +
##vacation/target/release/seq-vacation --json --outdir $TODAY-results/vacation-low+/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4096
##vacation/target/release/seq-vacation --json --outdir $TODAY-results/vacation-high+/ --runs $RUNS -n 2 -q 60 -u 90 -r 1048576 -t 4096
## ++
#vacation/target/release/seq-vacation --json --outdir $TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4194304
##vacation/target/release/seq-vacation --json --outdir $TODAY-results/vacation-high/ --runs $RUNS -n 2 -q 60 -u 90 -r 1048576 -t 4194304

#for tc in ${threadrange[@]}
#do
    ##vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 16384 -t 4096 --threads $tc
    ##vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-high/ --runs $RUNS -n 4 -q 60 -u 90 -r 16384 -t 4096 --threads $tc
    ## +
    ##vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-low+/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4096 --threads $tc
    ##vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-high+/ --runs $RUNS -n 4 -q 60 -u 90 -r 1048576 -t 4096 --threads $tc
    ## ++
    #vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4194304 --threads $tc -b Naive
    #vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-low-prime/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4194304 --threads $tc -b Prime
    ##vacation/target/release/stm-vacation --json --outdir $TODAY-results/vacation-high/ --runs $RUNS -n 4 -q 60 -u 90 -r 1048576 -t 4194304 --threads $tc
#done

## dstm
#for tc in ${threadrange[@]}
#do
    ##vacation/target/release/dstm-vacation --json --outdir $TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 16384 -t 4096 --threads $tc
    ##vacation/target/release/dstm-vacation --json --outdir $TODAY-results/vacation-high/ --runs $RUNS -n 4 -q 60 -u 90 -r 16384 -t 4096 --threads $tc
    ## +
    ##vacation/target/release/dstm-vacation --json --outdir $TODAY-results/vacation-low+/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4096 --threads $tc
    ##vacation/target/release/dstm-vacation --json --outdir $TODAY-results/vacation-high+/ --runs $RUNS -n 4 -q 60 -u 90 -r 1048576 -t 4096 --threads $tc
    ## ++
    #vacation/target/release/dstm-vacation --json --outdir $TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4194304 --threads $tc
    ##vacation/target/release/dstm-vacation --json --outdir $TODAY-results/vacation-high/ --runs $RUNS -n 4 -q 60 -u 90 -r 1048576 -t 4194304 --threads $tc
#done

#cd vacation
#for tc in ${threadrange[@]}
#do
    #sed -i "s/data-parallelism: [0-9]\+/data-parallelism: $tc/" ohua-config.yaml
    #./ohua-compile.sh
    #cargo build --release --quiet --bin seq-vacation
    ###target/release/vacation_new_compiler --json --outdir ../$TODAY-results/vacation/ --runs $RUNS -a 10 -l 128 -n 262144 -s 1
    ##target/release/vacation_new_compiler --json --outdir ../$TODAY-results/vacation/ --runs $RUNS -a 10 -l 16 -n 4096 -s 1
    #target/release/seq-vacation --json --outdir ../$TODAY-results/vacation-low/ --runs $RUNS -n 2 -q 90 -u 98 -r 1048576 -t 4194304 --clients $tc -b OhuaNaive
#done
#cd ..


#echo "Current time: $(date)\n\n"
    


echo "Finished at $(date)"


