use std::{error::Error, fs::File, io::Read};

use day_03::{do_dont_mul, mul};

const INPUT: &str = "./day-03/INPUT.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(INPUT)?;
    let mut instructions = String::new();
    file.read_to_string(&mut instructions)?;

    let res = mul(&instructions);
    println!("mul\t\t{:10}", res);

    let res = do_dont_mul(&instructions);
    println!("do don't mul\t{:10}", res);

    Ok(())
}
