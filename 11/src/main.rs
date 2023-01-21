mod monkey;

use std::str::{FromStr, Lines};

use monkey::*;

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
        let items: Vec<u16> = lines
            .next()
            .unwrap()
            .split(' ')
            .skip(4)
            .map(|x| {
                x.chars()
                    .take_while(|x| *x != ',')
                    .collect::<String>()
                    .parse::<u16>()
                    .unwrap()
            })
            .collect();

        //operation & change
        let (oper, change) = {
            let mut operation = lines.next().unwrap().split(' ').rev();
            let change = match operation.next().unwrap() {
                "old" => Change::Old,
                number @ _ => Change::Number(number.parse::<u16>().unwrap()),
            };
            let oper = Oper::from_str(operation.next().unwrap());
            (oper, change)
        };

        let test = get_last_number::<u16>(&mut lines);
        let true_idx = get_last_number::<usize>(&mut lines);
        let false_idx = get_last_number::<usize>(&mut lines);

        monkeys.push(Monkey {
            items,
            oper,
            change,
            test,
            true_idx,
            false_idx,
        });
    }
    monkeys
}

fn main() {
    let file = include_str!("input");
    let mut monkeys = parse_input(file);

    for _ in 0..20 {
        Monkey::do_round(&mut monkeys);
    }
}
