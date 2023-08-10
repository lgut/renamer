#!/bin/bash
cd "${0%/*}/../"
cargo build --release

bench_id=$(date +%s)
bench_machine=$(hostname)

for run in {001..100}; do
    rm -rf ~/tmp/renamer_output/* 2> /dev/null
    echo "Run: $run"
    ts=$(date +%s%N)
    ./target/release/renamer -q -r /home/luis/tmp/renamer_test_files /home/luis/tmp/renamer_output
    echo $((($(date +%s%N) - $ts)/1000)) >> "logs/${bench_machine}_rust_cli_$bench_id".log
done