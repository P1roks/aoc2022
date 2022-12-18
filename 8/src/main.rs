use std::{collections::HashSet, fs, iter::repeat};

struct Tree {
    tree_grid: Vec<Vec<u8>>,
    dimen: usize,
}

impl Tree {
    fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = fs::read_to_string(filename)?;
        let dimen = file.lines().count();
        let mut tree_grid = Vec::<Vec<u8>>::with_capacity(dimen);

        for line in file.lines() {
            let row = line
                .as_bytes()
                .iter()
                .map(|numb| (*numb as char).to_digit(10).unwrap() as u8)
                .collect();
            tree_grid.push(row);
        }

        Ok(Self { tree_grid, dimen })
    }

    fn display(&self) {
        for line in self.tree_grid.iter() {
            for elem in line.iter() {
                print!("{elem}");
            }
            println!();
        }
    }

    fn check_vis(&self, row: &[u8], idx: usize) -> usize {
        let mut uniq: HashSet<u8> = HashSet::with_capacity(row.len());
        //First and last index are always visible
        uniq.insert(0);
        uniq.insert((row.len() - 1) as u8);

        let mut tallest = row[0];
        let loop_row = &row[1..row.len() - 1];
        //left
        for (idx, elem) in loop_row.iter().enumerate() {
            if row[idx] > tallest {
                tallest = row[idx];
            }

            if elem > &tallest {
                uniq.insert((idx + 1) as u8);
            }
        }
        //right
        tallest = row[row.len() - 1];
        for (idx, elem) in loop_row.iter().rev().enumerate() {
            let idx = row.len() - idx - 1;
            if row[idx] > tallest {
                tallest = row[idx];
            }
            if uniq.contains(&((idx - 1) as u8)) {
                continue;
            }
            if elem > &tallest {
                uniq.insert(idx as u8 - 1);
            }
        }
        //up
        for (el_idx, elem) in loop_row.iter().enumerate() {
            let el_idx = el_idx + 1;
            if uniq.contains(&(el_idx as u8)) {
                continue;
            }
            tallest = self.tree_grid[0][el_idx];
            for curr in 0..idx {
                if self.tree_grid[curr][el_idx] > tallest {
                    tallest = self.tree_grid[curr][el_idx];
                }
            }
            if &tallest < elem {
                uniq.insert((el_idx) as u8);
            }
        }
        //down
        for (el_idx, elem) in loop_row.iter().enumerate() {
            let el_idx = el_idx + 1;
            if uniq.contains(&(el_idx as u8)) {
                continue;
            }
            tallest = self.tree_grid.last().unwrap()[el_idx];
            for curr in (idx..self.dimen).skip(1).rev() {
                if self.tree_grid[curr][el_idx] > tallest {
                    tallest = self.tree_grid[curr][el_idx];
                }
            }
            if &tallest < elem {
                uniq.insert((el_idx) as u8);
            }
        }
        uniq.len()
    }
    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let mut res = 1;
        let base = self.tree_grid[y][x];

        'left: for (idx, x) in (0..=x).rev().enumerate().skip(1) {
            if self.tree_grid[y][x] >= base {
                res *= idx;
                break 'left;
            }
            if x == 0 {
                res *= idx;
                break 'left;
            }
        }

        'right: for (idx, x) in (x..self.dimen).enumerate().skip(1) {
            if self.tree_grid[y][x] >= base {
                res *= idx;
                break 'right;
            }
            if x == (self.dimen - 1) {
                res *= idx;
                break 'right;
            }
        }

        'up: for (idx, y) in (0..=y).rev().enumerate().skip(1) {
            if self.tree_grid[y][x] >= base {
                res *= idx;
                break 'up;
            }
            if y == 0 {
                res *= idx;
                break 'up;
            }
        }

        'down: for (idx, y) in (y..self.dimen).enumerate().skip(1) {
            if self.tree_grid[y][x] >= base {
                res *= idx;
                break 'down;
            }
            if y == (self.dimen - 1) {
                res *= idx;
                break 'down;
            }
        }

        res
    }

    fn part1(&self) -> usize {
        //top and bottom row always visible
        let mut visible = self.dimen * 2;
        let grid = &self.tree_grid;

        for (idx, row) in grid.iter().skip(1).take(self.dimen - 2).enumerate() {
            visible += self.check_vis(row, idx + 1);
        }
        visible
    }

    fn part2(&self) -> usize {
        let mut max = 0;
        for y in (1..self.dimen).take(self.dimen - 2) {
            for x in (1..self.dimen).take(self.dimen - 2) {
                let val = self.scenic_score(x, y);
                if val > max {
                    max = val;
                }
            }
        }
        max
    }
}

fn main() {
    let tree = Tree::new("./input").unwrap();
    println!("{}", tree.part2());
}
