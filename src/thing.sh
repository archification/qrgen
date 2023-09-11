#!/bin/bash

target_file="random_file.txt"
#target_size=1073741824  # 1G
target_size=4096  # 4K
current_size=0

# Empty the file if it exists
> "$target_file"

# Loop until the file reaches the target size
while [ $current_size -lt $target_size ]; do
  head -c 100 /dev/urandom | tr -dc 'a-zA-Z0-9' >> "$target_file"
  current_size=$(stat -c %s "$target_file")
done

# Trim the file to exactly 4K
truncate -s $target_size "$target_file"
