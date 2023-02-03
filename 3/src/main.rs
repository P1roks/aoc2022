use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

trait Priority {
    fn to_priority(&self) -> Result<u8, &str>;
}
impl Priority for char {
    fn to_priority(&self) -> Result<u8, &str> {
        if !self.is_ascii() {
            return Err("Only ASCII allowed");
        }
        let self_bytes = *self as u8;
        if self_bytes < 65 || (self_bytes > 90 && self_bytes < 97) {
            return Err("Only letters allowed");
        }
        if self_bytes <= 90 {
            return Ok(self_bytes - 38);
        }
        Ok(self_bytes - 96)
    }
}
fn part1(file: &str) {
    let mut sum: u32 = 0;

    for line in file.lines() {
        let (comp_1, comp_2) = line.split_at(line.len() / 2);
        for chr in comp_1.as_bytes().iter() {
            let real_chr = *chr as char;
            if comp_2.contains(real_chr) {
                sum += real_chr.to_priority().unwrap() as u32;
                break;
            }
        }
    }
    println!("part1: {sum}");
}
fn part2(file: &str) {
    let mut lines = file.lines();
    let mut sum: u32 = 0;

    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        for chr in line1.as_bytes().iter() {
            let real_chr = *chr as char;
            if line2.contains(real_chr) && line3.contains(real_chr) {
                sum += real_chr.to_priority().unwrap() as u32;
                break;
            }
        }
    }
    println!("part2: {sum}");
}

fn main() {
    let file = include_str!("./input");
    part1(file);
    part2(file);
}
