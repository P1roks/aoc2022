#!/bin/sh

cat input | awk '
BEGIN {cycle = 0; x=1;}
function draw(){
	if (cycle % 40 == 0){
		print ""
		cycle = 0;
		return;
	}
	if (cycle == x || cycle == x + 1 || cycle == x+2)
		printf "#";
	else
		printf ".";
}
$2=="" {++cycle;draw(); next;}
{++cycle;draw();++cycle;draw();x+=$2}
'
