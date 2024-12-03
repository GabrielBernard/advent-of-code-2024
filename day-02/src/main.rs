use std::{fs::File, io::Read};

use day_02::{how_many_safe_p1, how_many_safe_p2_brute_force, parse_reports, D2Result};

const INPUT: &str = "./day-02/INPUT.txt";

fn main() -> D2Result<()> {
    let mut file = File::open(INPUT)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let reports = parse_reports(&buf);

    let p1_how_many_safe = how_many_safe_p1(&reports);
    println!("{} safe reports", p1_how_many_safe);

    let p2_how_many_safe = how_many_safe_p2_brute_force(&reports);
    println!("{} safe reports", p2_how_many_safe);

    Ok(())
}
