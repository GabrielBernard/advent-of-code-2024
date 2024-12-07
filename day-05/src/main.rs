use std::{error::Error, fs::File, io::Read};

use day_05::{sum_of_middle_page_of_each_print, RulesAndPrints};

const INPUT: &str = "./day-05/INPUT.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(INPUT)?;
    let mut rules_and_prints = String::new();
    file.read_to_string(&mut rules_and_prints)?;

    let rules_and_prints = RulesAndPrints::from(rules_and_prints.as_str());

    let (valid_prints, corrected_prints) = rules_and_prints.validate_ordering();
    let sum_valid = sum_of_middle_page_of_each_print(&valid_prints);

    println!("sum of middle pages of valid prints: {}", sum_valid);

    let corrected_slice_of_slice: Vec<&[u32]> =
        corrected_prints.iter().map(|p| p.as_slice()).collect();

    let sum_corrected = sum_of_middle_page_of_each_print(&corrected_slice_of_slice);

    println!("sum of middle pages of corrected prints: {}", sum_corrected);

    Ok(())
}
