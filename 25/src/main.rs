use std::iter::repeat;

fn snafu_to_dec(snafu: &str) -> u64 {
    let mut decimal: i64 = 0;
    let mut modifier = 1;

    for digit in snafu.bytes().rev() {
        let digit = digit as char;
        match digit {
            '0' => {}
            '1' => decimal += 1 * modifier,
            '2' => decimal += 2 * modifier,
            '-' => decimal -= 1 * modifier,
            '=' => decimal -= 2 * modifier,
            _ => unreachable!("Only 0,1,2,-,= allowed in snafu numbers!"),
        }
        modifier *= 5;
    }
    decimal as u64
}

fn calc_remainder(remainder: i64) -> String {
    String::from("TODO")
}

fn dec_to_snafu(dec: u64) -> String {
    let mut snafu: String;
    let (digits_count, remainder) = {
        let mut max_number = 2;
        let mut counter = 1;
        loop {
            max_number += 5u64.pow(counter) * 2;
            if max_number >= dec {
                snafu = String::with_capacity(counter as usize);
                let remainder = {
                    if dec > (max_number - 5u64.pow(counter)) {
                        snafu.push('2');
                        dec as i64 - (5i64.pow(counter) * 2) as i64
                    } else {
                        snafu.push('1');
                        dec as i64 - 5i64.pow(counter)
                    }
                };
                break (counter, remainder);
            }
            counter += 1;
        }
    };
    snafu
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn snafu_to_dec_valid() {
        assert_eq!(35, snafu_to_dec("120"));
        assert_eq!(12345, snafu_to_dec("1-0---0"));
        assert_eq!(314159265, snafu_to_dec("1121-1110-1=0"));
        assert_eq!(1234, snafu_to_dec("20-2-"));
        assert_eq!(3742, snafu_to_dec("1100=2"));
        assert_eq!(5291, snafu_to_dec("2=22=1"));
        assert_eq!(610351562, snafu_to_dec("2222222222222"));
        assert_eq!(937, snafu_to_dec("12222"));
    }
}

fn main() {
    let input = include_str!("input");
    let sum = input.lines().map(|line| snafu_to_dec(line)).sum::<u64>();
    println!("{}", dec_to_snafu(3742));
}
