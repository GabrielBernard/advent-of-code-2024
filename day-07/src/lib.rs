#[derive(Debug, Clone)]
pub struct Equation {
    test_value: u64,
    equation: Vec<u64>,
}

pub fn equations_from_str(value: &str) -> Vec<Equation> {
    value.lines().map(Equation::from).collect()
}

impl Equation {
    pub fn test_value(&self) -> u64 {
        self.test_value
    }

    pub fn is_true_equation(&self) -> bool {
        let mut ops: Vec<u64> = Vec::new();

        for (mut h, n) in self.equation.iter().enumerate() {
            if h >= 2 {
                let mut ops_r = ops.clone();
                ops_r.reverse();
                ops.append(&mut ops_r);
                h = ops.len()
            } else {
                h = 2
            }

            for i in 0..h {
                if let Some(o) = ops.get_mut(i) {
                    if i % 2 == 0 {
                        *o *= n
                    } else {
                        *o += n
                    }
                } else {
                    ops.push(*n);
                };
            }
        }

        ops.iter().any(|r| *r == self.test_value)
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (test_value, equation) = value
            .split_once(':')
            .expect("there should be a test value and an equation");

        let test_value = test_value.parse().expect("test value should be a number");
        let equation = equation
            .split_whitespace()
            .map(|n| n.parse().expect("equation should be made of numbers"))
            .collect();

        Self {
            test_value,
            equation,
        }
    }
}

pub fn true_equations(equations: &[Equation]) -> Vec<Equation> {
    equations
        .iter()
        .filter(|&eq| eq.is_true_equation())
        .map(Clone::clone)
        .collect()
}

pub fn sum_results_of_true_equations(equations: &[Equation]) -> u64 {
    let true_equations = true_equations(equations);

    true_equations.iter().map(|e| e.test_value()).sum()
}

#[cfg(test)]
mod test {

    #[test]
    fn find_equations_that_are_true() {
        let equations = super::equations_from_str(TEST_VALUES_AND_EQUATIONS);
        let true_eq = super::true_equations(&equations);

        let actual = true_eq.len();

        assert_eq!(3, actual)
    }

    #[test]
    fn sum_of_result_of_true_equations() {
        let equations = super::equations_from_str(TEST_VALUES_AND_EQUATIONS);

        let actual = super::sum_results_of_true_equations(&equations);

        assert_eq!(3749, actual)
    }

    const TEST_VALUES_AND_EQUATIONS: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;
}
