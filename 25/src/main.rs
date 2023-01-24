//sum can be written in 20 digits
const MAX_VALS: [i64; 20] = snafu_digs!(20);
mod macros {
    #[macro_export]
    macro_rules! snafu_digs {
        ($no: expr) => {{
            let mut max_val = 2;
            let mut digits: [i64; $no] = [2i64; $no];

            //no for in macros
            let mut i = 1;
            while i < $no {
                max_val += 2 * 5i64.pow(i as u32);
                digits[i] = max_val;
                i += 1;
            }

            digits
        }};
    }
}
fn snafu_to_dec(snafu: &str) -> i64 {
    let mut decimal: i64 = 0;
    let mut r#mod = 1;

    for digit in snafu.bytes().rev() {
        let digit = digit as char;
        match digit {
            '0' => {}
            '1' => decimal += 1 * r#mod,
            '2' => decimal += 2 * r#mod,
            '-' => decimal -= 1 * r#mod,
            '=' => decimal -= 2 * r#mod,
            _ => unreachable!("Only 0,1,2,-,= allowed in snafu numbers!"),
        }
        r#mod *= 5;
    }
    decimal
}

//return type = (digit,remainder)
fn get_digit(curr_no: i64, idx: usize) -> (char, i64) {
    let max_val = 5i64.pow(idx as u32);
    let prev_idx = (idx as isize - 1).clamp(0, 19) as usize;
    let prev_max = MAX_VALS[prev_idx];

    if idx == 0 {
        return match curr_no {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            -1 => ('-', 0),
            -2 => ('=', 0),
            _ => ('0', 0),
            // _ => unreachable!("not possible!"),
        };
    }

    println!("curr = {curr_no} max = {max_val} prev_max = {prev_max}");

    //Edge cases, edge cases, edge cases, love 'em or hate 'em they're present
    if prev_max > curr_no.abs() {
        return ('0', curr_no);
    } else if curr_no.is_positive() {
        if (curr_no - max_val).abs() < prev_max
            || (curr_no - MAX_VALS[idx]).abs() == max_val
            || curr_no == prev_max + 1
        {
            return ('1', curr_no - max_val);
        } else {
            return ('2', curr_no - max_val * 2);
        }
    } else if curr_no.is_negative() {
        if (max_val + curr_no).abs() < prev_max {
            return ('-', curr_no + max_val);
        } else {
            return ('=', curr_no + max_val * 2);
        }
    } else {
        return ('0', curr_no);
    }
}

fn dec_to_snafu(mut dec: i64) -> String {
    let max_idx = {
        let mut i = 0;
        loop {
            if MAX_VALS[i] > dec {
                break i;
            }
            i += 1;
        }
    };
    (0..=max_idx)
        .rev()
        .map(|i| {
            let (digit, remainder) = get_digit(dec, i);
            dec = remainder;
            digit
        })
        .collect::<String>()
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
        assert_eq!(937, snafu_to_dec("12222"));
        assert_eq!(313, snafu_to_dec("1===="));
    }

    #[test]
    fn dec_to_snafu_valid() {
        assert_eq!(dec_to_snafu(35), "120");
        assert_eq!(dec_to_snafu(12345), "1-0---0");
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
        assert_eq!(dec_to_snafu(1234), "20-2-");
        assert_eq!(dec_to_snafu(3742), "1100=2");
        assert_eq!(dec_to_snafu(5291), "2=22=1");
        assert_eq!(dec_to_snafu(937), "12222");
        assert_eq!(dec_to_snafu(313), "1====");
    }
}

fn main() {
    let input = include_str!("input");
    let sum = input.lines().map(|line| snafu_to_dec(line)).sum::<i64>();
    println!("{}", dec_to_snafu(sum));
}
