use std::{error::Error, fs::File, io::Read};

use day_06::{find_guard_looping_solutions, Grid};

const INPUT: &str = "./day-06/INPUT.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open(INPUT)?;
    let mut map_string = String::new();
    file.read_to_string(&mut map_string)?;

    let grid = Grid::from(map_string.as_str());

    let (guard_path, _) = grid.clone().find_guard_path();

    let sum_guard_path = Grid::sum_guard_path(guard_path.clone());

    println!("sum of guard path: {}", sum_guard_path);

    let loop_found = find_guard_looping_solutions(grid);

    println!("potential loop found: {}", loop_found);

    Ok(())
}
