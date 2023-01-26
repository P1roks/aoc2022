import * as fs from 'fs';
let pairs = fs.readFileSync("./input").toString().split('\n\n')

function cmp_pair(left: number[]|number,right: number[]|number): boolean|null{
	//both numbers
	if (typeof left === "number" && typeof right === "number"){
		if (left < right) { return true;}
		else if (left > right) { return false;}
		return null;
	}
	//both arrays
	else if(Array.isArray(left) && Array.isArray(right)){
		let minLen = Math.min(left.length,right.length);
		for(let idx = 0; idx < minLen; ++idx){
			const val = cmp_pair(left[idx], right[idx]);
			if(val !== null){return val;}
		}

		return cmp_pair(left.length, right.length);
	}

	//mixed
	if (typeof left === "number"){
		return cmp_pair([left], right);
	}
	else if (typeof right === "number"){
		return cmp_pair(left, [right]);
	}

	console.log(left,right)
	throw new Error("Something really bad happened: left and right is nor number[], number or mixed")
}

const part1 = () => {
	let sum = 0;
	for(let idx = 0; idx < pairs.length; ++idx){
		let vals = pairs[idx].split('\n');
		let [left,right] = [JSON.parse(vals[0]),JSON.parse(vals[1])];
		sum += cmp_pair(left, right) ? (idx + 1) : 0;
	}
	console.log("Part 1 solution:",sum);
}

const part2 = () => {
	let packets = [];
	//to sort we will want them as actual arrays
	for(let idx = 0; idx < pairs.length; ++idx){
		let vals = pairs[idx].split('\n');
		let [left,right] = [JSON.parse(vals[0]),JSON.parse(vals[1])];
		packets.push(left,right)
	}

	let divs = [[[2]],[[6]]];
	packets.push(...divs)

	let decoder_key = packets
	.sort((left,right) => Number(cmp_pair(right,left)) - Number(cmp_pair(left,right)))
	.reduce((accum,elem,idx) => divs.includes(elem) ? accum * (idx + 1) : accum,1);

	console.log("Part 2 solution:",decoder_key);
}

function main(){
	part1();
	part2();
}
main();
