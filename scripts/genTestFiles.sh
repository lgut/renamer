#!/bin/bash

p="$HOME/tmp/renamer_test_files"
out="$HOME/tmp/renamer_output"

mkdir $p 2> /dev/null
mkdir $out 2> /dev/null

echo "Creating dummy RAW files"
for run in {001..100}; do
    dd if=/dev/urandom bs=1M count=11 status=none > "$p/DSC_$run.DEF"
done

echo "Creating sub directory"
subdir="$p/vacation"
mkdir $subdir 2> /dev/null

cp $p/DSC_00*.DEF $subdir
