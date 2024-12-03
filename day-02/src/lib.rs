use std::error::Error;

const MIN_DIFF: i32 = 1;
const MAX_DIFF: i32 = 3;

pub type D2Result<T> = Result<T, Box<dyn Error>>;

pub fn parse_reports(reports: &str) -> Vec<Vec<i32>> {
    reports
        .lines()
        .filter_map(|line| {
            let report: D2Result<Vec<i32>> = line
                .split_whitespace()
                .map(|n| n.parse().map_err(Into::into))
                .collect();
            report.ok()
        })
        .collect()
}

pub fn how_many_safe_p1(reports: &[Vec<i32>]) -> u32 {
    reports.iter().fold(
        0,
        |acc, report| {
            if safe_report(report) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

pub fn how_many_safe_p2_brute_force(reports: &[Vec<i32>]) -> u32 {
    reports.iter().fold(0, |acc, report| {
        if !safe_report(report) {
            let mut safe = false;
            for i in 0..report.len() {
                let mut r = Vec::new();
                r.append(&mut report[0..i].to_vec());
                r.append(&mut report[i + 1..].to_vec());
                safe |= safe_report(&r);
            }

            if safe {
                acc + 1
            } else {
                acc
            }
        } else {
            acc + 1
        }
    })
}

pub fn safe_report(report: &[i32]) -> bool {
    let mut acceptable_diff = true;

    let mut iter = report.iter().copied();
    let mut previous = iter.next().unwrap_or_default();

    let mut all_increasing = true;
    let mut all_decreasing = true;
    for n in iter {
        let (increasing, decreasing, second_condition) = conditions(previous, n);

        all_increasing &= increasing;
        all_decreasing &= decreasing;
        acceptable_diff &= second_condition;
        previous = n;
    }

    (all_increasing || all_decreasing) && acceptable_diff
}

fn conditions(previous: i32, n: i32) -> (bool, bool, bool) {
    let increasing = n > previous;
    let decreasing = n < previous;

    let diff = (n - previous).abs();
    let diff_contained = (MIN_DIFF..=MAX_DIFF).contains(&diff);

    (increasing, decreasing, diff_contained)
}

#[cfg(test)]
mod test {
    use crate::{how_many_safe_p1, parse_reports};

    use super::D2Result;

    const REPORTS: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

    #[test]
    fn safe_reports() -> D2Result<()> {
        let reports = parse_reports(REPORTS);

        let actual = how_many_safe_p1(&reports);

        assert_eq!(2, actual);

        Ok(())
    }

    #[test]
    fn safe_reports_with_problem_dampener() -> D2Result<()> {
        let reports = parse_reports(REPORTS);

        let actual = super::how_many_safe_p2_brute_force(&reports);

        assert_eq!(4, actual);

        Ok(())
    }
}
