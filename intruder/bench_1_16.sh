rm Cargo.lock 
cargo clean

cargo build --release --bin sequential --features "cli"
cargo build --release --bin ohua --features "cli ohua"
cargo build --release --bin ohua-split2 --features "ohua cli"
cargo build --release --bin ohua-split3 --features "ohua cli"
cargo build --release --bin ohua-split4 --features "ohua cli"
cargo build --release --bin ohua-split5 --features "ohua cli"
cargo build --release --bin ohua-split6 --features "ohua cli"
cargo build --release --bin ohua-split7 --features "ohua cli"
cargo build --release --bin ohua-split8 --features "ohua cli"
cargo build --release --bin ohua-split9 --features "ohua cli"
cargo build --release --bin ohua-split10 --features "ohua cli"
cargo build --release --bin ohua-split11 --features "ohua cli"
cargo build --release --bin ohua-split12 --features "ohua cli"
cargo build --release --bin ohua-split13 --features "ohua cli"
cargo build --release --bin ohua-split14 --features "ohua cli"
cargo build --release --bin ohua-split15 --features "ohua cli"
cargo build --release --bin ohua-split16 --features "ohua cli"


echo "Starting benchmarks at $(date)"

target/release/sequential --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split2 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split3 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split4 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split5 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split6 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split7 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split8 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split9 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split10 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split11 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split12 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split13 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split14 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split15 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
target/release/ohua-split16 --json --outdir  split_test_20191122 --runs 30 -n 262144
echo -n "."
echo " done!"

target/release/sequential --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split2 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split3 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split4 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split5 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split6 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split7 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split8 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split9 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split10 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split11 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split12 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split13 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split14 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split15 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."
target/release/ohua-split16 --json --outdir  split_test_20191122 --runs 30 -n 262144 -l 128
echo -n "."

echo "Finished at $(date)"
