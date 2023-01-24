use std::collections::HashMap;
use std::ops as o;
use std::time::Instant;
#[derive(Debug, Clone)]
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
        use Oper::*;
        match self {
            Add => item_1 + item_2,
            Sub => item_1 - item_2,
            Div => item_1 / item_2,
            Mul => item_1 * item_2,
        }
    }
    //gotta love math
    fn do_oper_right<F, U>(&self, item_1: F, item_2: U) -> F
    where
        F: o::Sub<U, Output = F> + o::Div<U, Output = F>,
        U: o::Sub<F, Output = F> + o::Div<F, Output = F>,
    {
        use Oper::*;
        match self {
            Add => item_2 - item_1,
            Sub => item_1 - item_2,
            Div => item_1 / item_2,
            Mul => item_2 / item_1,
        }
    }
    fn from(og: &str) -> Self {
        use Oper::*;
        match og {
            "+" => Add,
            "-" => Sub,
            "/" => Div,
            "*" => Mul,
            _ => panic!("Non supported operation!"),
        }
    }
    fn opposite(&self) -> Self {
        use Oper::*;
        match self {
            Add => Sub,
            Sub => Add,
            Mul => Div,
            Div => Mul,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    first_monkey: String,
    second_monkey: String,
    oper: Oper,
}

//djb2, bcs imagine using Strings as keys
fn hash(val: &str) -> String {
    /* let mut hash: u64 = 5381;

    for c in val.as_bytes() {
        hash = ((hash << 5) + hash) + *c as u64;
    } */

    // hash
    val.to_owned()
}

fn parse_input(file: &str) -> (HashMap<String, Monkey>, HashMap<String, u64>) {
    //solved and unsolved monkeys
    let mut s_monkeys = HashMap::new();
    let mut us_monkeys = HashMap::new();

    for line in file.lines() {
        let mut mnk = line.split(": ");
        let name = hash(mnk.next().unwrap());

        //job of a given monkey
        let job = mnk.next().unwrap();
        if let Ok(val) = job.parse::<_>() {
            us_monkeys.insert(name, val);
        } else {
            let mut val = job.split(' ');
            let name1 = hash(val.next().unwrap());
            let oper = Oper::from(val.next().unwrap());
            let name2 = hash(val.next().unwrap());

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
        if let Some(root) = s_monkeys.get(&hash("root")) {
            println!("{root}");
            return;
        }
    }
}

fn solve_part2(s_monkeys: &mut HashMap<String, u64>, us_monkeys: &mut HashMap<String, Monkey>) {
    let unsolvable = &find_humn(us_monkeys, "root").unwrap();
    s_monkeys.remove(&"humn".to_owned());

    let mut keys_to_remove = Vec::new();
    let (first, second) = {
        let root = us_monkeys.get("root").unwrap();
        (root.first_monkey.to_owned(), root.second_monkey.to_owned())
    };
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
        //humn is not in us_monkeys
        if us_monkeys.len() == (unsolvable.len() - 1) {
            //need to reverse from the solved value in root
            let mut val = *s_monkeys.get(&first).or(s_monkeys.get(&second)).unwrap();
            for mnk in unsolvable
                .iter()
                .skip(1)
                .rev()
                .skip(1)
                .map(|name| us_monkeys.get(name).unwrap())
            {
                //lhs is known
                if let Some(mnk_val) = s_monkeys.get(&mnk.first_monkey) {
                    val = mnk.oper.opposite().do_oper_right(val, mnk_val);
                }
                //rhs is known
                else if let Some(mnk_val) = s_monkeys.get(&mnk.second_monkey) {
                    val = mnk.oper.opposite().do_oper(val, mnk_val);
                }
            }

            return;
        }
    }
}

fn find_humn(us_monkeys: &HashMap<String, Monkey>, name: &str) -> Option<Vec<String>> {
    if name == "humn" {
        return Some(vec![name.to_owned()]);
    }

    if let Some(mnk) = us_monkeys.get(&name.to_owned()) {
        if let Some(mut vec) =
            find_humn(us_monkeys, &mnk.first_monkey).or(find_humn(us_monkeys, &mnk.second_monkey))
        {
            vec.push(name.to_owned());
            return Some(vec);
        }
    }

    None
}

fn main() {
    // let time = Instant::now();
    let (mut us_monkeys, mut s_monkeys) = parse_input(include_str!("input"));
    solve_part1(&mut s_monkeys, &mut us_monkeys);
    // println!("{:?}", time.elapsed());
}
