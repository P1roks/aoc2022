use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let line = line?;
        if let Some((range1, range2)) = line.split_once(',') {
            let (r1n1, r1n2) = range1.split_once('-').unwrap();
            let (r2n1, r2n2) = range2.split_once('-').unwrap();
            let r1n1 = r1n1.parse::<u8>().unwrap();
            let r1n2 = r1n2.parse::<u8>().unwrap();
            let r2n1 = r2n1.parse::<u8>().unwrap();
            let r2n2 = r2n2.parse::<u8>().unwrap();

            if r1n1 <= r2n2 && r2n1 <= r1n2 {
                sum += 1;
            }

            /* if r1n1 <= r2n1 || r1n2 >= r2n2 {
                sum += 1;
            } else if r2n1 <= r1n1 || r2n2 >= r1n2 {
                sum += 1;
            } */
        }
    }

    print!("{sum}");
    Ok(())
}
