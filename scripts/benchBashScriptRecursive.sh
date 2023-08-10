#!/bin/bash
# ts=$(date +%s%N)
# sleep 1s 
# echo $((($(date +%s%N) - $ts)/1000))
# echo $((($(date +%s%N) - $ts)/1000000))
cd "${0%/*}/../"

bench_id=$(date +%s)
bench_machine=$(hostname)

for run in {001..100}; do
    rm -rf ~/tmp/renamer_output/* 2> /dev/null
    echo "Run: $run"
    ts=$(date +%s%N)
    ./scripts/file_renamer.sh -r -o ~/tmp/renamer_output ~/tmp/renamer_test_files > /dev/null
    echo $((($(date +%s%N) - $ts)/1000)) >> "logs/${bench_machine}_renamer_recursive_$bench_id.log"
done