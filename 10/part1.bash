#!/bin/sh

cat input | awk '
	BEGIN {cycle = 0; x=1;sum=0;}
	function print_cycl(){
	if (cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220)
	sum += cycle * x;
	}
	$2=="" {++cycle;print_cycl(); next;}
	{++cycle;print_cycl();++cycle;print_cycl();x+=$2}
	END {print sum;}
'
