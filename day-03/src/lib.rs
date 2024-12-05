use regex::Regex;

const MUL_REGEX: &str = r"mul\((\d+),(\d+)\)";
const DO_DONT_MUL_REGEX: &str = r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))";

pub fn mul(instructions: &str) -> u32 {
    let re = Regex::new(MUL_REGEX).expect("regex should compile");

    re.captures_iter(instructions).fold(0u32, |res, c| {
        let n1: u32 = c
            .get(1)
            .expect("n1 should be present")
            .as_str()
            .parse()
            .expect("should be a number");

        let n2: u32 = c
            .get(2)
            .expect("n2 should be present")
            .as_str()
            .parse()
            .expect("should be a number");

        res + n1 * n2
    })
}

pub fn do_dont_mul(instructions: &str) -> u32 {
    let re = Regex::new(DO_DONT_MUL_REGEX).expect("regex should compile");

    let mut dont = false;
    re.captures_iter(instructions).fold(0u32, |res, c| {
        let statement = c.get(1).expect("should always have a statement").as_str();
        if statement == "don't()" {
            dont = true;
        } else if statement == "do()" {
            dont = false;
            return res;
        }

        if dont {
            return res;
        }

        if !statement.contains("mul(") {
            return res;
        }

        let n1: u32 = c
            .get(2)
            .expect("n1 should be present")
            .as_str()
            .parse()
            .expect("should be a number");

        let n2: u32 = c
            .get(3)
            .expect("n2 should be present")
            .as_str()
            .parse()
            .expect("should be a number");

        res + n1 * n2
    })
}

#[cfg(test)]
mod test {
    use crate::{do_dont_mul, mul};

    const INSTRUCTIONS_P1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const INSTRUCTIONS_P2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn mul_instructions() {
        let actual = mul(INSTRUCTIONS_P1);

        assert_eq!(161, actual);
    }

    #[test]
    fn do_dont() {
        let actual = do_dont_mul(INSTRUCTIONS_P2);

        assert_eq!(48, actual);
    }
}
