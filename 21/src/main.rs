use std::collections::HashMap;
use std::ops as o;

#[derive(Debug)]
enum Oper {
    Add,
    Sub,
    Div,
    Mul,
}
impl Oper {
    fn do_oper<F, U>(&self, item_1: F, item_2: U) -> F
    where
        F: o::Add<U, Output = F>
            + o::Sub<U, Output = F>
            + o::Div<U, Output = F>
            + o::Mul<U, Output = F>,
    {
        match self {
            Self::Add => item_1 + item_2,
            Self::Sub => item_1 - item_2,
            Self::Div => item_1 / item_2,
            Self::Mul => item_1 * item_2,
        }
    }
    fn from(og: &str) -> Self {
        match og {
            "+" => Self::Add,
            "-" => Self::Sub,
            "/" => Self::Div,
            "*" => Self::Mul,
            _ => panic!("Non supported operation!"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    first_monkey: String,
    second_monkey: String,
    oper: Oper,
}

fn parse_input(file: &str) -> (HashMap<String, Monkey>, HashMap<String, u64>) {
    //solved and unsolved monkeys
    let mut s_monkeys = HashMap::new();
    let mut us_monkeys = HashMap::new();

    for line in file.lines() {
        let mut mnk = line.split(": ");
        let name = mnk.next().unwrap().to_string();

        //job of a given monkey
        let job = mnk.next().unwrap();
        if let Ok(val) = job.parse::<u64>() {
            us_monkeys.insert(name, val);
        } else {
            let mut val = job.split(' ');
            let name1 = val.next().unwrap().to_owned();
            let oper = Oper::from(val.next().unwrap());
            let name2 = val.next().unwrap().to_owned();

            let monkey = Monkey {
                first_monkey: name1,
                second_monkey: name2,
                oper,
            };
            s_monkeys.insert(name, monkey);
        }
    }

    (s_monkeys, us_monkeys)
}

fn solve_part1(s_monkeys: &mut HashMap<String, u64>, us_monkeys: &mut HashMap<String, Monkey>) {
    let mut keys_to_remove = Vec::new();
    loop {
        for mnk_p in &*us_monkeys {
            let mnk = &mnk_p.1;
            if let Some((val1, val2)) = s_monkeys
                .get(&mnk.first_monkey)
                .zip(s_monkeys.get(&mnk.second_monkey))
            {
                let mnk_val = mnk.oper.do_oper(*val1, *val2);
                s_monkeys.insert(mnk_p.0.to_owned(), mnk_val);
                keys_to_remove.push(mnk_p.0.to_owned());
            }
        }
        for key in keys_to_remove.drain(..) {
            us_monkeys.remove(&key.to_owned());
        }
        if let Some(root) = s_monkeys.get("root") {
            println!("{root}");
            return;
        }
    }
}

fn main() {
    let (mut us_monkeys, mut s_monkeys) = parse_input(include_str!("input"));
    solve_part1(&mut s_monkeys, &mut us_monkeys);
}
