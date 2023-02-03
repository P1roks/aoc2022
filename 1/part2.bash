#!/bin/sh

awk 'BEGIN {sum=0} $1=="" {print sum; sum =0; next} {sum+=$1}' input | sort -g | tail -n 3 | awk 'BEGIN {sum=0} {sum+=$1} END {print sum}'
