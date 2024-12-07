use std::collections::HashMap;

#[derive(Debug)]
pub struct RulesAndPrints {
    page_order_rules: HashMap<u32, Vec<u32>>,
    updated_prints: Vec<Vec<u32>>,
}

impl RulesAndPrints {
    pub fn validate_ordering(&self) -> (Vec<&[u32]>, Vec<Vec<u32>>) {
        let (valid_prints, invalid_prints) = self.updated_prints.iter().fold(
            (Vec::new(), Vec::new()),
            |(mut valid_prints, mut invalid_prints), print| {
                if self.is_valid(print) {
                    valid_prints.push(print.as_slice());
                } else {
                    let mut corrected_print = print.to_owned();
                    while !self.is_valid(&corrected_print) {
                        corrected_print = self.fix_ordering(&corrected_print);
                    }
                    invalid_prints.push(corrected_print);
                }

                (valid_prints, invalid_prints)
            },
        );

        (valid_prints, invalid_prints)
    }

    fn is_valid<'a>(&'a self, print: &'a [u32]) -> bool {
        let mut valid = true;
        for (i, p) in print.iter().enumerate() {
            if let Some(rules) = self.page_order_rules.get(p) {
                valid = validate_rule(i, rules, print);
            }

            if !valid {
                return valid;
            }
        }

        valid
    }

    pub fn fix_ordering(&self, invalid_print: &[u32]) -> Vec<u32> {
        let mut corrected_print: Vec<u32> = Vec::new();

        for (current_invalid_print_index, invalid_page) in invalid_print.iter().enumerate() {
            corrected_print.push(*invalid_page);

            if let Some(rules) = self.page_order_rules.get(invalid_page) {
                for rule_page in rules {
                    if let Some(rule_validation_page_index) = invalid_print
                        .iter()
                        .position(|print_page| print_page == rule_page)
                    {
                        if current_invalid_print_index > rule_validation_page_index {
                            let new_position = corrected_print
                                .iter()
                                .position(|page| page == &invalid_print[rule_validation_page_index])
                                .expect("this should exist");
                            corrected_print.swap(current_invalid_print_index, new_position);
                        }
                    }
                }
            }
        }

        assert_eq!(invalid_print.len(), corrected_print.len());
        corrected_print
    }
}

fn validate_rule(current_index: usize, rules: &[u32], print: &[u32]) -> bool {
    let mut valid = true;
    for page in rules {
        if let Some(i) = print
            .iter()
            .position(|print_page_position| print_page_position == page)
        {
            valid = current_index < i;
        }

        if !valid {
            return valid;
        }
    }

    valid
}

pub fn sum_of_middle_page_of_each_print(prints: &[&[u32]]) -> u32 {
    prints
        .iter()
        .fold(0, |acc, print| acc + print[print.len() / 2])
}

impl From<&str> for RulesAndPrints {
    fn from(input: &str) -> Self {
        let start_of_prints = input
            .find("\n\n")
            .expect("a new line before list of updated prints");

        let page_order_rules = input
            .get(0..start_of_prints)
            .expect("page orders")
            .lines()
            .fold(HashMap::new(), |mut acc, l| {
                let (p_before, p_after): (u32, u32) = l
                    .split_once('|')
                    .map(|(p_before, p_after)| {
                        (
                            p_before.parse().expect("page should be a number"),
                            p_after.parse().expect("page should be a number"),
                        )
                    })
                    .expect("needs an order");

                acc.entry(p_before)
                    .and_modify(|pl: &mut Vec<u32>| pl.push(p_after))
                    .or_insert(vec![p_after]);

                acc
            });

        let updated_prints: Vec<Vec<u32>> = input
            .get(start_of_prints + 2..)
            .expect("print updates")
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse().expect("should be a page number"))
                    .collect()
            })
            .collect();

        RulesAndPrints {
            page_order_rules,
            updated_prints,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::RulesAndPrints;

    #[test]
    fn parse_input() {
        let s = RulesAndPrints::from(PAGES_AND_ORDERING);
        let (actual_valid, actual_corrected) = s.validate_ordering();

        assert_eq!(3, actual_valid.len());
        assert_eq!(3, actual_corrected.len())
    }

    #[test]
    fn sum_of_valid_middle_pages() {
        let s = RulesAndPrints::from(PAGES_AND_ORDERING);
        let (valid_prints, _) = s.validate_ordering();

        let actual = super::sum_of_middle_page_of_each_print(&valid_prints);

        assert_eq!(143, actual);
    }

    #[test]
    fn sum_of_corrected_middle_pages() {
        let s = RulesAndPrints::from(PAGES_AND_ORDERING);
        let (_, corrected_prints) = s.validate_ordering();

        let corrected_slice_of_slice: Vec<&[u32]> =
            corrected_prints.iter().map(|p| p.as_slice()).collect();

        let actual = super::sum_of_middle_page_of_each_print(&corrected_slice_of_slice);

        assert_eq!(123, actual);
    }

    const PAGES_AND_ORDERING: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;
}
