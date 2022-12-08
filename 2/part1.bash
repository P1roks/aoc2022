#!/bin/sh

cat input |
awk 'BEGIN {ur_score = 0;}
$2=="X" {ur_score+=1;
if($1=="A") ur_score+= 3; if($1=="C") ur_score+=6}
$2=="Y" {ur_score+=2;
if($1=="B")ur_score+=3; if($1=="A") ur_score+=6;}
$2=="Z" {ur_score += 3; if($1=="C") ur_score+=3; if($1=="B") ur_score+=6;}
END {print ur_score}'
