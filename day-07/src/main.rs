use std::{error::Error, fs::File, io::Read};

use day_07::{equations_from_str, sum_results_of_true_equations, sum_results_of_true_equations_p2};

const INPUT: &str = "./day-07/INPUT.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(INPUT)?;
    let mut equations = String::new();
    file.read_to_string(&mut equations)?;

    let equations = equations_from_str(&equations);

    let sum_result_of_true_equations = sum_results_of_true_equations(&equations);
    println!(
        "sum of results of true equations: {}",
        sum_result_of_true_equations
    );

    let sum_result_of_true_equations_p2 = sum_results_of_true_equations_p2(&equations);
    println!(
        "sum of results of true equations p2: {}",
        sum_result_of_true_equations_p2
    );

    Ok(())
}
