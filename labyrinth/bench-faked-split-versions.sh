#!/bin/bash
set -e
echo "Starting benchmarks at $(date)"

# clean and build
cargo clean
cargo build --release --bin simple_sequential --features "cli bench"
cargo build --release --bin ohua --features "cli bench ohua"
cargo build --release --bin threads-data-par --features "cli bench"
cargo build --release --bin ohua-split1 --features "cli bench ohua"
cargo build --release --bin ohua-split2 --features "cli bench ohua"
cargo build --release --bin ohua-split3 --features "cli bench ohua"
cargo build --release --bin ohua-split4 --features "cli bench ohua"
cargo build --release --bin ohua-split5 --features "cli bench ohua"
cargo build --release --bin ohua-split6 --features "cli bench ohua"
cargo build --release --bin ohua-split7 --features "cli bench ohua"
cargo build --release --bin ohua-split8 --features "cli bench ohua"
cargo build --release --bin ohua-split9 --features "cli bench ohua"
cargo build --release --bin ohua-split10 --features "cli bench ohua"
cargo build --release --bin ohua-split11 --features "cli bench ohua"
cargo build --release --bin ohua-split12 --features "cli bench ohua"
cargo build --release --bin ohua-split13 --features "cli bench ohua"
cargo build --release --bin ohua-split14 --features "cli bench ohua"
cargo build --release --bin ohua-split15 --features "cli bench ohua"
cargo build --release --bin ohua-split16 --features "cli bench ohua"
cargo build --release --bin ohua-split17 --features "cli bench ohua"
cargo build --release --bin ohua-split18 --features "cli bench ohua"
cargo build --release --bin ohua-split19 --features "cli bench ohua"
cargo build --release --bin ohua-split20 --features "cli bench ohua"


# ------ faked/x32-y32-z3-n96.txt ------
echo "Running benchmarks for faked/x32-y32-z3-n96.txt"

echo -n "Sequential..."
# run sequential version
target/release/simple_sequential inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30

echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/ohua-split1 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 1
target/release/ohua-split2 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 2
target/release/ohua-split3 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 3
target/release/ohua-split4 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 4
target/release/ohua-split5 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 5
target/release/ohua-split6 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 6
target/release/ohua-split7 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 7
target/release/ohua-split8 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 8
target/release/ohua-split9 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 9
target/release/ohua-split10 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 10
target/release/ohua-split11 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 11
target/release/ohua-split12 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 12
target/release/ohua-split13 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 13
target/release/ohua-split14 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 14
target/release/ohua-split15 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 15
target/release/ohua-split16 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 16
target/release/ohua-split17 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 17
target/release/ohua-split18 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 18
target/release/ohua-split19 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 19
target/release/ohua-split20 inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x32-y32-z3-n96.txt --json -o data_par_no_overhead --runs 30 -s 20
echo " done."


# ------ faked/x48-y48-z3-n64.txt ------
echo "Running benchmarks for faked/x48-y48-z3-n64.txt"

echo -n "Sequential..."
# run sequential version
target/release/simple_sequential inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30

echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/ohua-split1 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 1
target/release/ohua-split2 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 2
target/release/ohua-split3 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 3
target/release/ohua-split4 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 4
target/release/ohua-split5 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 5
target/release/ohua-split6 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 6
target/release/ohua-split7 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 7
target/release/ohua-split8 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 8
target/release/ohua-split9 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 9
target/release/ohua-split10 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 10
target/release/ohua-split11 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 11
target/release/ohua-split12 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 12
target/release/ohua-split13 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 13
target/release/ohua-split14 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 14
target/release/ohua-split15 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 15
target/release/ohua-split16 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 16
target/release/ohua-split17 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 17
target/release/ohua-split18 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 18
target/release/ohua-split19 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 19
target/release/ohua-split20 inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x48-y48-z3-n64.txt --json -o data_par_no_overhead --runs 30 -s 20
echo " done."


# ------ faked/x128-y128-z5-n128.txt ------
echo "Running benchmarks for faked/x128-y128-z5-n128.txt"

echo -n "Sequential..."
# run sequential version
target/release/simple_sequential inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30

echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/ohua-split1 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 1
target/release/ohua-split2 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 2
target/release/ohua-split3 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 3
target/release/ohua-split4 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 4
target/release/ohua-split5 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 5
target/release/ohua-split6 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 6
target/release/ohua-split7 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 7
target/release/ohua-split8 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 8
target/release/ohua-split9 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 9
target/release/ohua-split10 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 10
target/release/ohua-split11 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 11
target/release/ohua-split12 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 12
target/release/ohua-split13 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 13
target/release/ohua-split14 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 14
target/release/ohua-split15 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 15
target/release/ohua-split16 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 16
target/release/ohua-split17 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 17
target/release/ohua-split18 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 18
target/release/ohua-split19 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 19
target/release/ohua-split20 inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x128-y128-z5-n128.txt --json -o data_par_no_overhead --runs 30 -s 20
echo " done."


# ------ faked/x256-y256-z5-n256.txt ------
echo "Running benchmarks for faked/x256-y256-z5-n256.txt"

echo -n "Sequential..."
# run sequential version
target/release/simple_sequential inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30

echo " done."

echo -n "Ohua..."
# run ohua
target/release/ohua inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/ohua-split1 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 1
target/release/ohua-split2 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 2
target/release/ohua-split3 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 3
target/release/ohua-split4 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 4
target/release/ohua-split5 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 5
target/release/ohua-split6 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 6
target/release/ohua-split7 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 7
target/release/ohua-split8 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 8
target/release/ohua-split9 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 9
target/release/ohua-split10 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 10
target/release/ohua-split11 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 11
target/release/ohua-split12 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 12
target/release/ohua-split13 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 13
target/release/ohua-split14 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 14
target/release/ohua-split15 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 15
target/release/ohua-split16 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 16
target/release/ohua-split17 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 17
target/release/ohua-split18 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 18
target/release/ohua-split19 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 19
target/release/ohua-split20 inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30
target/release/threads-data-par inputs/faked/x256-y256-z5-n256.txt --json -o data_par_no_overhead --runs 30 -s 20
echo " done."




echo "Finished benchmarks at $(date)!"
