#!/bin/sh

cat input | awk 'BEGIN {sum=0} $1=="" {print sum; sum =0; next} {sum+=$1}' | sort -g | tail -n 1
