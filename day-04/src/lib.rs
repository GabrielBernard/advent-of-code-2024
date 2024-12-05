const LETTERS: &[char] = &['X', 'M', 'A', 'S'];

pub struct Grid {
    data: Vec<char>,
    w: usize,
    h: usize,
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let chars: Vec<Vec<char>> = value.lines().map(|l| l.chars().collect()).collect();

        let h = chars.len();
        let w = chars[0].len();
        let data = chars.concat();

        Grid { data, w, h }
    }
}

impl Grid {
    fn char_at(&self, c: usize, r: usize) -> Option<&char> {
        let i = c + r * self.h;

        self.data.get(i)
    }

    fn column_row_from_index(&self, i: usize) -> (usize, usize) {
        let c = i % self.w;
        let r = i / self.w;

        (c, r)
    }

    pub fn find_all_xmas(&self) -> u32 {
        let mut xmas = 0;

        for i in 0..self.data.len() {
            let (c, r) = self.column_row_from_index(i);
            xmas += self.check(c, r, Direction::Left, 0);
            xmas += self.check(c, r, Direction::DiagDownLeft, 0);
            xmas += self.check(c, r, Direction::Down, 0);
            xmas += self.check(c, r, Direction::DiagDownRight, 0);
            xmas += self.check(c, r, Direction::Right, 0);
            xmas += self.check(c, r, Direction::DiagUpRight, 0);
            xmas += self.check(c, r, Direction::Up, 0);
            xmas += self.check(c, r, Direction::DiagUpLeft, 0)
        }

        xmas
    }

    pub fn find_all_x_mas(&self) -> u32 {
        let mut x_mas = 0;

        for i in 0..self.data.len() {
            let (c, r) = self.column_row_from_index(i);

            if self.check(c, r, Direction::DiagDownLeft, 1) == 1 {
                x_mas += self.check(c + 2, r, Direction::DiagDownRight, 1);
                x_mas += self.check(c, r + 2, Direction::DiagUpLeft, 1);
            }

            if self.check(c, r, Direction::DiagUpRight, 1) == 1 {
                if r > 1 {
                    x_mas += self.check(c, r - 2, Direction::DiagDownRight, 1);
                }

                if c > 1 {
                    x_mas += self.check(c - 2, r, Direction::DiagUpLeft, 1);
                }
            }
        }

        x_mas
    }

    fn check(&self, c: usize, r: usize, direction: Direction, next_letter: usize) -> u32 {
        match self.char_at(c, r) {
            Some(&'S') if next_letter == LETTERS.len() - 1 => {
                return 1;
            }
            Some(&ch) if ch != LETTERS[next_letter] => {
                return 0;
            }
            Some(_) => {}
            None => {
                return 0;
            }
        }

        let limw = self.w - 1;
        let limh = self.h - 1;
        match direction {
            Direction::Left => {
                if c >= limw {
                    0
                } else {
                    self.check(c + 1, r, direction, next_letter + 1)
                }
            }
            Direction::DiagDownLeft => {
                if c >= limw || r >= limh {
                    0
                } else {
                    self.check(c + 1, r + 1, direction, next_letter + 1)
                }
            }
            Direction::Down => {
                if r >= limh {
                    0
                } else {
                    self.check(c, r + 1, direction, next_letter + 1)
                }
            }
            Direction::DiagDownRight => {
                if c == 0 || r >= limh {
                    0
                } else {
                    self.check(c - 1, r + 1, direction, next_letter + 1)
                }
            }
            Direction::Right => {
                if c == 0 {
                    0
                } else {
                    self.check(c - 1, r, direction, next_letter + 1)
                }
            }
            Direction::DiagUpRight => {
                if c == 0 || r == 0 {
                    0
                } else {
                    self.check(c - 1, r - 1, direction, next_letter + 1)
                }
            }
            Direction::Up => {
                if r == 0 {
                    0
                } else {
                    self.check(c, r - 1, direction, next_letter + 1)
                }
            }
            Direction::DiagUpLeft => {
                if c >= limw || r == 0 {
                    0
                } else {
                    self.check(c + 1, r - 1, direction, next_letter + 1)
                }
            }
        }
    }
}

enum Direction {
    Left,
    DiagDownLeft,
    Down,
    DiagDownRight,
    Right,
    DiagUpRight,
    Up,
    DiagUpLeft,
}

#[cfg(test)]
mod test {
    use crate::Grid;

    const WORD_PUZZLE: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn find_xmas() {
        let grid = Grid::from(WORD_PUZZLE);

        let actual = grid.find_all_xmas();

        assert_eq!(18, actual)
    }

    #[test]
    fn find_x_mas() {
        let grid = Grid::from(WORD_PUZZLE);

        let actual = grid.find_all_x_mas();

        assert_eq!(9, actual)
    }
}
