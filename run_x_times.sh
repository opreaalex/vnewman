#!/bin/bash
for run in {0..20}
do
  printf "%s" "------------------------------"
  cargo run
done
