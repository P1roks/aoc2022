use std::ops as op;

#[derive(Debug)]
pub enum Change {
    Number(u16),
    Old,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<u16>,
    pub oper: Oper,
    pub change: Change,
    pub test: u16,
    pub true_idx: usize,
    pub false_idx: usize,
}

impl Monkey {
    pub fn do_round(monkeys: &mut Vec<Monkey>) //, monkeys: &mut Vec<Monkey>) {
    {
        for monkey in monkeys {
            for item in monkey.items.drain(..) {
                let change = match monkey.change {
                    Change::Number(x) => x,
                    Change::Old => item,
                };

                let new_worry = monkey.oper.do_oper(item, change) / 3;

                let idx = if new_worry % monkey.test == 0 {
                    monkey.true_idx
                } else {
                    monkey.false_idx
                };

                monkeys[idx].items.push(new_worry);
            }
        }
    }
}
