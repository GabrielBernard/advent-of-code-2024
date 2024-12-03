use std::{collections::HashMap, error::Error};

pub type D1Result<T> = Result<T, Box<dyn Error>>;

pub fn p1_sum_of_distances(locations_1: &[u32], locations_2: &[u32]) -> u32 {
    let mut locations_1 = locations_1.to_owned();
    locations_1.sort();

    let mut locations_2 = locations_2.to_owned();
    locations_2.sort();

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

pub fn p2_similarity_score(locations_1: &[u32], locations_2: &[u32]) -> u32 {
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

#[cfg(test)]
mod test {
    const LOCATIONS_1: &[u32] = &[3, 4, 2, 1, 3, 3];
    const LOCATIONS_2: &[u32] = &[4, 3, 5, 3, 9, 3];

    #[test]
    fn sum_of_distances() {
        let actual = super::p1_sum_of_distances(LOCATIONS_1, LOCATIONS_2);

        assert_eq!(11, actual)
    }

    #[test]
    fn similarity_score() {
        let actual = super::p2_similarity_score(LOCATIONS_1, LOCATIONS_2);

        assert_eq!(31, actual)
    }
}
