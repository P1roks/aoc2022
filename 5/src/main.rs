use std::fs;

fn parse_boxes(boxes: &str) -> Vec<Vec<char>> {
    let mut parsed: Vec<Vec<char>> = Vec::with_capacity(9);
    unsafe {
        parsed.set_len(9);
    }
    for line in boxes.split('\n') {
        for (idx, supp) in line.bytes().skip(1).step_by(4).enumerate() {
            let supp = supp as char;
            //check for last row
            if supp.is_digit(10) {
                break;
            }
            //only want to add real boxes
            if supp != ' ' {
                parsed[idx].push(supp);
            }
        }
    }
    parsed.iter_mut().for_each(|x| x.reverse());
    parsed
}

struct Oper {
    quant: u8,
    from: u8,
    to: u8,
}

fn parse_move(mov: &str) -> Option<Oper> {
    // for line in moves.split('\n') {
    let mut split = mov.split(' ').skip(1).step_by(2);
    if let Some((quant, (from, to))) = split.next().zip(split.next().zip(split.next())) {
        let quant = quant.parse().unwrap();
        let from = from.parse().unwrap();
        let to = to.parse().unwrap();
        return Some(Oper { quant, from, to });
    }
    // }
    None
}

fn solve_part_1(moves: &str, boxes: &mut Vec<Vec<char>>) {
    for line in moves.split('\n') {
        if let Some(movement) = parse_move(line) {
            for _ in 1..=movement.quant {
                if let Some(mv_box) = boxes[(movement.from - 1) as usize].pop() {
                    boxes[(movement.to - 1) as usize].push(mv_box);
                }
            }
        }
    }
}

fn solve_part_2(moves: &str, boxes: &mut Vec<Vec<char>>) {
    for line in moves.split('\n') {
        if let Some(movement) = parse_move(line) {
            let moved = {
                let curr_box = &mut boxes[(movement.from - 1) as usize];
                let quant = curr_box.len() - movement.quant as usize;
                curr_box.drain(quant..curr_box.len()).collect::<Vec<char>>()
            };
            boxes[(movement.to - 1) as usize].extend(moved);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string("./input")?;
    let (boxes, moves) = file.split_once("\n\n").unwrap();
    let mut boxes = parse_boxes(boxes);
    let mut boxes2 = boxes.clone();

    solve_part_1(moves, &mut boxes);
    solve_part_2(moves, &mut boxes2);

    for single in boxes.iter() {
        print!("{}", single.last().unwrap());
    }
    println!();
    for single in boxes2.iter() {
        print!("{}", single.last().unwrap());
    }
    Ok(())
}
