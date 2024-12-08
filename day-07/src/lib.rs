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

pub fn is_true_equation_p2_recursive(result: u64, equation: &[u64]) -> bool {
    let Some((last, eq)) = equation.split_last() else {
        return result == 0;
    };

    let is_addition_true = if *last <= result {
        let addition = result - last;
        is_true_equation_p2_recursive(addition, eq)
    } else {
        false
    };

    let multiplication = result / last;
    let is_multiplication_true =
        result % last == 0 && is_true_equation_p2_recursive(multiplication, eq);

    let is_concat_true = if *last <= result {
        let concat = result - last;
        let how_many_tens = 10u64.pow(last.ilog10() + 1);

        concat % how_many_tens == 0 && is_true_equation_p2_recursive(concat / how_many_tens, eq)
    } else {
        false
    };

    is_addition_true || is_multiplication_true || is_concat_true
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

pub fn true_equations_p2(equations: &[Equation]) -> Vec<Equation> {
    equations
        .iter()
        .filter(|&eq| is_true_equation_p2_recursive(eq.test_value(), &eq.equation))
        .map(Clone::clone)
        .collect()
}

pub fn sum_results_of_true_equations_p2(equations: &[Equation]) -> u64 {
    let true_equations = true_equations_p2(equations);

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

    #[test]
    fn find_equations_that_are_true_p2() {
        let equations = super::equations_from_str(TEST_VALUES_AND_EQUATIONS);
        let true_eq = super::true_equations_p2(&equations);

        println!("{:?}", true_eq);

        let actual = true_eq.len();

        assert_eq!(6, actual)
    }

    #[test]
    fn sum_of_result_of_true_equations_p2() {
        let equations = super::equations_from_str(TEST_VALUES_AND_EQUATIONS);

        let actual = super::sum_results_of_true_equations_p2(&equations);

        assert_eq!(11387, actual)
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
