use std::{error::Error, fs::File, io::Read};

use day_04::Grid;

const INPUT: &str = "./day-04/INPUT.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(INPUT)?;
    let mut puzzle = String::new();
    file.read_to_string(&mut puzzle)?;

    let grid = Grid::from(puzzle.as_str());
    let all_xmas = grid.find_all_xmas();

    println!("{} xmas", all_xmas);

    let all_x_mas = grid.find_all_x_mas();

    println!("{} x_mas", all_x_mas);

    Ok(())
}
