fn parse_input(file: &str) -> Vec<[i8; 3]> {
    let mut cubes: Vec<[i8; 3]> = Vec::with_capacity(file.lines().count());

    for line in file.lines() {
        let mut numbers = line.split(',').map(|x| x.parse::<i8>().unwrap());
        if let Some((no1, (no2, no3))) = numbers.next().zip(numbers.next().zip(numbers.next())) {
            cubes.push([no1, no2, no3]);
        }
    }

    cubes
}

fn solve_part_1(lava: Vec<[i8; 3]>) -> u32 {
    let mut exposed = vec![[true; 6]; lava.len()];

    //TODO: rework this atrocious code, maybe even optimize it so it no longer is O(n^2)
    for (idx, single) in lava.iter().enumerate() {
        let ex = &mut exposed[idx];
        for other in lava.iter() {
            if single == other {
                continue;
            }
            if single[0] == other[0] && single[1] == other[1] {
                if single[2] - other[2] == 1 {
                    ex[0] = false;
                } else if single[2] - other[2] == -1 {
                    ex[1] = false;
                }
            }
            if single[1] == other[1] && single[2] == other[2] {
                if single[0] - other[0] == 1 {
                    ex[2] = false;
                } else if single[0] - other[0] == -1 {
                    ex[3] = false;
                }
            }
            if single[0] == other[0] && single[2] == other[2] {
                if single[1] - other[1] == 1 {
                    ex[4] = false;
                } else if single[1] - other[1] == -1 {
                    ex[5] = false;
                }
            }
        }
    }

    exposed
        .iter()
        .map(move |val| {
            val[0] as u32
                + val[1] as u32
                + val[2] as u32
                + val[3] as u32
                + val[4] as u32
                + val[5] as u32
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn good_conversion() {
        assert_eq!(
            vec![[1, 2, 2], [2, 2, 2], [3, 3, 1]],
            parse_input("1,2,2\n2,2,2\n3,3,1\n")
        );
    }

    #[test]
    fn part1() {
        assert_eq!(10, solve_part_1(vec![[1, 1, 1], [2, 1, 1]]));
    }

    #[test]
    fn abs_diff() {
        assert_eq!(1, 3u8.abs_diff(4));
        assert_eq!(1, 3u8.abs_diff(2));
        assert_eq!(0, 3u8.abs_diff(3));
    }
}

fn main() {
    let lava = parse_input(include_str!("./input"));
    print!("{}", solve_part_1(lava));
}
