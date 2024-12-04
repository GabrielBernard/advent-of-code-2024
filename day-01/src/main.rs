use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use day_01::{p1_sum_of_distances, p2_similarity_score, D1Result};

const INPUT: &str = "./day-01/INPUT.txt";

fn main() -> D1Result<()> {
    let file = File::open(INPUT)?;
    let reader = BufReader::new(file);

    let mut locations_1: Vec<u32> = Vec::new();
    let mut locations_2: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        let mut split = line.split_whitespace();
        assert_eq!(split.clone().count(), 2);

        let location_id_1 = split.next().expect("location id 1 should always exist");
        let loaction_id_2 = split.next().expect("location id 2 should always exist");

        locations_1.push(location_id_1.parse()?);
        locations_2.push(loaction_id_2.parse()?);
    }

    let sum_of_distances: u32 = p1_sum_of_distances(&locations_1, &locations_2);
    println!("Sum of distances: {}", sum_of_distances);

    let similarity_score = p2_similarity_score(&locations_1, &locations_2);
    println!("Similarity score: {}", similarity_score);

    Ok(())
}
