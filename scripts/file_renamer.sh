#!/bin/bash

OPTIND=1 # reset getops

# command variables
quite=0
output_path="."
in_place=0
recursive=0

while getopts ":hriqo:" opt; do
    case ${opt} in
        h )
            echo "Help needs implementing"
            exit 1
            ;;
        q ) 
            quite=1
            ;;
        r ) 
            recursive=1
            ;;
        o )
            output_path=$OPTARG
            ;;
        i ) 
            in_place=1
            ;;
        \?) 
            echo "Help needs implementing"
            exit 1
            ;;
        esac
    done
shift $((OPTIND -1))

function get_parent() {
    echo "$(dirname "$(readlink -f "$1")" | awk -F"/" ' { print $NF } ')"
}

function get_rand(){
    echo "$(cat /dev/urandom | tr -cd 'a-f0-9' | head -c 33)"
}


function walk_dir() {
    local rand=$(get_rand)
    local count=1
    for item in "${1}"/*; do
        
        if [ -d "$item" ] && [ $recursive == 1 ]; then
            walk_dir "$item"
        elif [ -d "$item" ] && [ $recursive == 0 ]; then
             >&2 echo "Recusive flag not set. Skipping $item"
        elif [ -f $item ]; then
            local namespace="$(get_parent "$item")_$rand"
            rename_file "$namespace" "$item" "$((count++))"
        else
            >&2 echo "$item is not a directory or regular file. Skipping"
        fi
    done 
}


function rename_file(){
    local namespace=$1        
    local file=$2
    local ext=${file##*.}
    local filenum=$(printf "%05d" $3)
    local parent=$(get_parent "$file")
    local destination=0
    if [ $in_place == 1 ];then
        if [ "$input_dir" = "$parent" ]; then
            destination="${parent}/${namespace}_${filenum}.${ext}"
            mv $file "$destination"
        else
            destination="${input_dir}/${parent}/${namespace}_${filenum}.${ext}"
            mv $file "$destination"
        fi
    else
        if [ $recursive == 1 ]; then
            if [ "$output_path" =  "$parent" ]; then
                destination="${output_path}/${namespace}_${filenum}.${ext}"
                cp $file "$destination"
            else
                mkdir -p "${output_path}/${parent}"
                destination="${output_path}/${parent}/${namespace}_${filenum}.${ext}"
                cp $file "$destination"
            fi
        else
            destination="${output_path}/${namespace}_${filenum}.${ext}"
            cp $file "$destination"
        fi
    fi
    
    if [ $quite = 0 ]; then
        echo "$file -> $destination"
    fi
}

# rename given file(s)
if [ -t 0 ] && [ ! -z "$1" ]; then
    # input is not being piped in
    rand="$(get_rand)"
    if [ -d "$1" ]; then
        input_dir="$1"
        walk_dir "$1"
    elif [ -f $1 ]; then
        namespace="$(get_parent "$1")_$rand"
        count=1
        input_dir="$(get_parent "$1")"
        rename_file "$namespace" "$1" $count
    else
        >&2 echo "$1 is not a directory or regular file. Skipping"
        exit 1
    fi
else
    >&2 echo "Piping is currently broken"
    exit 1
    local rand=$(get_rand)
    local count=1
    while read line; do
        if [ -d "$1" ] && [ $recursive == 1 ]; then
            input_dir="$1"
            walk_dir "$1"
        elif [ -d "$1" ] && [ $recursive == 0 ]; then
             >&2 echo "Recusive flag not set. Skipping $1"
        elif [ -f $1 ]; then
            namespace="$(get_parent "$1")_$rand"
            input_dir="$(get_parent "$1")"
            rename_file "$namespace" "$1" "$((count++))"
        else
            >&2 echo "$1 is not a directory or regular file. Skipping"
        fi
    done
fi
exit 0
