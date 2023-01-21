use std::ops as op;

#[derive(Debug, Clone)]
pub enum Change {
    Number(u32),
    Old,
}
#[derive(Debug, Clone)]
pub enum Oper {
    Plus,
    Minus,
    Div,
    Mul,
}

impl Oper {
    pub fn do_oper<F, U>(&self, item_1: F, item_2: U) -> F
    where
        F: op::Add<U, Output = F>
            + op::Mul<U, Output = F>
            + op::Sub<U, Output = F>
            + op::Div<U, Output = F>,
    {
        match self {
            Self::Plus => item_1 + item_2,
            Self::Minus => item_1 - item_2,
            Self::Mul => item_1 * item_2,
            Self::Div => item_1 / item_2,
        }
    }
    pub fn from_chr(oper: char) -> Self {
        match oper {
            '*' => Self::Mul,
            '+' => Self::Plus,
            '-' => Self::Minus,
            '/' => Self::Div,
            _ => panic!("Invalid opearator"),
        }
    }
    pub fn from_str(oper: &str) -> Self {
        match oper {
            "*" => Self::Mul,
            "+" => Self::Plus,
            "-" => Self::Minus,
            "/" => Self::Div,
            _ => panic!("Invalid opearator"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<u32>,
    pub oper: Oper,
    pub change: Change,
    pub test: u32,
    pub true_idx: usize,
    pub false_idx: usize,
    pub operations: u32,
}

impl Monkey {
    pub fn do_round(monkeys: &mut Vec<Monkey>) //, monkeys: &mut Vec<Monkey>) {
    {
        for i in 0..monkeys.len() {
            let monkey_clone = monkeys[i].clone();
            monkeys[i].operations += monkey_clone.items.len() as u32;

            for item in monkey_clone.items {
                let change = match monkey_clone.change {
                    Change::Number(x) => x,
                    Change::Old => item,
                };

                let new_worry = monkey_clone.oper.do_oper(item, change) / 3;

                let idx = if new_worry % monkey_clone.test == 0 {
                    monkey_clone.true_idx
                } else {
                    monkey_clone.false_idx
                };

                monkeys[idx].items.push(new_worry);
            }
            monkeys[i].items.clear();
        }
    }
}
