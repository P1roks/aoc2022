mod monkey;

use std::{
    cmp::Reverse,
    str::{FromStr, Lines},
};

use monkey::*;
use num_bigint::BigUint;

fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    fn get_last_number<F>(lines: &mut Lines) -> F
    where
        F: FromStr,
    {
        match lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<F>()
        {
            Ok(val) => val,
            Err(_) => panic!("Couldn't parse"),
        }
    }

    for monkey in input.split("\n\n") {
        let mut lines = monkey.lines();
        //First line is not important
        lines.next();

        //items
        let items: Vec<_> = lines
            .next()
            .unwrap()
            .split(' ')
            .skip(4)
            .map(|x| {
                x.chars()
                    .take_while(|x| *x != ',')
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect();

        //operation & change
        let (oper, change) = {
            let mut operation = lines.next().unwrap().split(' ').rev();
            let change = match operation.next().unwrap() {
                "old" => Change::Old,
                number @ _ => Change::Number(number.parse::<u64>().unwrap()),
            };
            let oper = Oper::from_str(operation.next().unwrap());
            (oper, change)
        };

        let test = get_last_number::<u64>(&mut lines);
        let true_idx = get_last_number::<usize>(&mut lines);
        let false_idx = get_last_number::<usize>(&mut lines);

        monkeys.push(Monkey {
            items,
            oper,
            change,
            test,
            true_idx,
            false_idx,
            operations: 0,
        });
    }
    monkeys
}

fn part1(mut monkeys: Vec<Monkey>) {
    for _ in 0..20 {
        Monkey::do_round(&mut monkeys);
    }

    let mut operations_count = monkeys.iter().map(|x| x.operations).collect::<Vec<_>>();

    operations_count.sort_by_key(|&key| Reverse(key));
    let result = operations_count.iter().take(2).product::<u32>();
    println!("{result}");
}

fn part2(mut monkeys: Vec<Monkey>) {
    let common_div = monkeys.iter().map(|x| x.test).product();
    for _ in 0..10000 {
        Monkey::do_round_part2(&mut monkeys, common_div);
    }

    let mut operations_count = monkeys.iter().map(|x| x.operations).collect::<Vec<_>>();

    operations_count.sort_by_key(|&key| Reverse(key));
    let result = operations_count.iter().take(2).product::<BigUint>();
    println!("{result}");
}
fn main() {
    let file = include_str!("input");
    let mut monkeys = parse_input(file);
    //part1(monkeys);
    part2(monkeys);
}
