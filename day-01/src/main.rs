use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const INPUT: &str = "./day-01/INPUT.txt";

type D1Result<T> = Result<T, Box<dyn Error>>;

fn p1_distance(locations_1: &Vec<u32>, locations_2: &Vec<u32>) -> u32 {
    locations_1
        .iter()
        .zip(locations_2.iter())
        .fold(0, |acc, (l1, l2)| {
            if l1 > l2 {
                acc + (l1 - l2)
            } else {
                acc + (l2 - l1)
            }
        })
}

fn p2_similarity_score(locations_1: &Vec<u32>, locations_2: &Vec<u32>) -> u32 {
    let frequency = locations_2
        .iter()
        .copied()
        .fold(HashMap::new(), |mut acc, l2| {
            acc.entry(l2).and_modify(|d| *d += 1).or_insert(1u32);

            acc
        });

    locations_1.iter().copied().fold(0, |acc, v| {
        let f = frequency.get(&v).copied().unwrap_or_default();
        acc + v * f
    })
}

fn main() -> D1Result<()> {
    let file = File::open(INPUT)?;
    let reader = BufReader::new(file);

    let mut locations_1: Vec<u32> = Vec::new();
    let mut locations_2: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let line = line?;

        let mut split = line.split_whitespace();
        assert_eq!(split.clone().count(), 2);

        let location_id_1 = split.next().expect("location id 1 should always exit");
        let loaction_id_2 = split.next().expect("location id 2 should always exit");

        locations_1.push(location_id_1.parse()?);
        locations_2.push(loaction_id_2.parse()?);
    }

    locations_1.sort();
    locations_2.sort();

    let sum_of_distances: u32 = p1_distance(&locations_1, &locations_2);
    println!("Sum of distances: {}", sum_of_distances);

    let similarity_score = p2_similarity_score(&locations_1, &locations_2);
    println!("Similarity score: {}", similarity_score);

    Ok(())
}
