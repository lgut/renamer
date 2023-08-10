#!/bin/bash
# ts=$(date +%s%N)
# sleep 1s 
# echo $((($(date +%s%N) - $ts)/1000))
# echo $((($(date +%s%N) - $ts)/1000000))

# SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
# makes working directory the folder above the script
# so that logs always end up in the project's log folder
cd "${0%/*}/../"

bench_id=$(date +%s)
bench_machine=$(hostname)

for run in {001..100}; do
    rm -rf ~/tmp/renamer_output/* 2> /dev/null
    echo "Run: $run"
    ts=$(date +%s%N)
    ./scripts/file_renamer.sh -o ~/tmp/renamer_output ~/tmp/renamer_test_files > /dev/null
    echo $((($(date +%s%N) - $ts)/1000)) >> "logs/${bench_machine}_renamer_$bench_id.log"
done