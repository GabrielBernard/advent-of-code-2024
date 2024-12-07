use std::{collections::HashMap, error::Error};

const OBSTACLE: char = '#';
const GUARD: char = '^';
const GUARD_PATH: char = 'X';

const ADDED_OBSTACLE: char = 'O';

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    data: Vec<char>,
    w: usize,
    h: usize,
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for (i, &c) in self.data.iter().enumerate() {
            if i % self.w == 0 {
                s.push('\n');
            }
            s.push(c);
        }

        write!(f, "{}", s)
    }
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
        if c >= self.w {
            return None;
        }

        let i = c + r * self.h;

        self.data.get(i)
    }

    fn index_of(&self, c: usize, r: usize) -> usize {
        c + r * self.h
    }

    fn column_row_from_index(&self, i: usize) -> (usize, usize) {
        let c = i % self.w;
        let r = i / self.w;

        (c, r)
    }

    pub fn guard_position(&self) -> Option<usize> {
        self.data.iter().position(|&c| c == GUARD)
    }

    pub fn find_guard_path(&self) -> (Grid, bool) {
        let guard_position = self.guard_position().expect("there should be a guard");
        let (mut gc, mut gr) = self.column_row_from_index(guard_position);
        let mut is_guard_looping = false;

        let mut data = self.data.clone();
        let mut direction = Direction::Top;
        let mut previous_obstacle_map = HashMap::new();
        let mut previous_obstacle = (gc as i64, gr as i64);

        loop {
            data[self.index_of(gc, gr)] = GUARD_PATH;
            let (dc, dr) = direction.velocity();

            let nc = gc as i64 + dc;
            let nr = gr as i64 + dr;
            if nc < 0 || nr < 0 {
                break;
            }

            let ch = match self.char_at(nc as usize, nr as usize) {
                Some(&ch) => ch,
                None => {
                    break;
                }
            };

            if ch == OBSTACLE || ch == ADDED_OBSTACLE {
                let previous_obstacle_that_brought_the_guard_here: Option<&(i64, i64)> =
                    previous_obstacle_map.get(&(nc, nr));

                if previous_obstacle_that_brought_the_guard_here
                    .is_some_and(|&o| o == previous_obstacle)
                {
                    is_guard_looping = true;
                }

                previous_obstacle_map
                    .entry((nc, nr))
                    .and_modify(|v| *v = previous_obstacle)
                    .or_insert(previous_obstacle);
                previous_obstacle = (nc, nr);

                direction = direction.turn_right();
            } else {
                gc = nc as usize;
                gr = nr as usize;
            }

            if is_guard_looping {
                break;
            }
        }

        (
            Grid {
                data,
                w: self.w,
                h: self.h,
            },
            is_guard_looping,
        )
    }

    pub fn sum_guard_path(grid: Grid) -> u32 {
        grid.data
            .iter()
            .filter_map(|&c| if c == GUARD_PATH { Some(1u32) } else { None })
            .sum()
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn replace_index_with_added_obstacle(&mut self, i: usize) -> Result<(), Box<dyn Error>> {
        let ch = self.data[i];
        if ch != GUARD && ch != OBSTACLE {
            self.data[i] = ADDED_OBSTACLE;
            Ok(())
        } else {
            Err("found guard or obstacle".into())
        }
    }
}

enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            Direction::Top => Direction::Right,
            Direction::Right => Direction::Bottom,
            Direction::Bottom => Direction::Left,
            Direction::Left => Direction::Top,
        }
    }

    fn velocity(&self) -> (i64, i64) {
        match self {
            Direction::Top => (0, -1),
            Direction::Right => (1, 0),
            Direction::Bottom => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

pub fn find_guard_looping_solutions(initial_grid: Grid) -> u32 {
    let mut loop_found = 0;
    for i in 0..initial_grid.size() {
        let mut new_grid = initial_grid.clone();
        if new_grid.replace_index_with_added_obstacle(i).is_err() {
            continue;
        }

        let (_, is_guard_looping) = new_grid.find_guard_path();
        if is_guard_looping {
            loop_found += 1;
        }
    }

    loop_found
}

#[cfg(test)]
mod test {
    use crate::Grid;

    #[test]
    fn guard_path() {
        let expected = Grid::from(GUARD_PATH);
        let grid = Grid::from(MAP_START);

        let (actual, _) = grid.find_guard_path();

        assert_eq!(expected, actual);
    }

    #[test]
    fn number_of_position_visited() {
        let guard_path = Grid::from(GUARD_PATH);

        let actual = Grid::sum_guard_path(guard_path);

        assert_eq!(41, actual)
    }

    #[test]
    fn is_loop() {
        let guard_path = Grid::from(MAP_GUARD_LOOP);

        let (_, is_guard_looping) = guard_path.find_guard_path();

        assert!(is_guard_looping)
    }

    #[test]
    fn not_as_loop() {
        let guard_path = Grid::from(NOT_A_LOOP);

        let (_, is_guard_looping) = guard_path.find_guard_path();

        assert!(!is_guard_looping)
    }

    #[test]
    fn possible_loops() {
        let guard_path = Grid::from(MAP_START);

        let loop_found = super::find_guard_looping_solutions(guard_path);

        assert_eq!(6, loop_found);
    }

    const MAP_START: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    const GUARD_PATH: &str = r#"....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X.."#;

    const MAP_GUARD_LOOP: &str = r#"....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O.."#;

    const NOT_A_LOOP: &str = r#"....#.....
.........#
..........
..#.......
..O....#..
..........
.#..^.....
........#.
#.........
......#..."#;
}
