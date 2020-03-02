//! # icub3d Sudoku Solver
//!
//! `icub3d_sudoku_solver` is simple utility for solving a sudoku
//! board using backtracking.

#![crate_name = "icub3d_sudoku_solver"]

use failure::{bail, Error};
use regex::Regex;

/// A representation of a sudoku board.
///
/// # Example
/// # use icub3d_sudoku_solver::Board;
/// let mut board = Board::new(
///     "120400586060201403040096000090000014081000360430000070000720030608903040372008051"
///         .to_string(),
/// )
/// .unwrap();
///
/// assert_eq!(board.solve(), true);
/// assert_eq!(board.to_string(), "1 2 9 | 4 3 7 | 5 8 6\n8 6 7 | 2 5 1 | 4 9 3\n5 4 3 | 8 9 6 | 1 2 7\n------+-------+------\n7 9 5 | 3 6 2 | 8 1 4\n2 8 1 | 5 7 4 | 3 6 9\n4 3 6 | 1 8 9 | 2 7 5\n------+-------+------\n9 1 4 | 7 2 5 | 6 3 8\n6 5 8 | 9 1 3 | 7 4 2\n3 7 2 | 6 4 8 | 9 5 1\n");
#[derive(Debug, PartialEq)]
pub struct Board {
    grid: Vec<u8>,
}

impl Board {
    /// Create a new board. Will fail if the string isn't exactly 81
    /// characters long. '.', '_', ' ', and '0' can be used for empty
    /// spaces.
    pub fn new(s: String) -> Result<Board, Error> {
        let re = Regex::new(r"^[ 0-9._]*$")?;
        if s.len() != 81 {
            bail!("string must be exactly 81 characters");
        } else if !re.is_match(&s) {
            bail!("string must contain only digits (and _, ' ', or . for zero, empty)");
        }

        let mut b = Board { grid: Vec::new() };
        for c in s.chars() {
            if c == '.' || c == '_' || c == ' ' {
                b.grid.push(0);
            } else {
                b.grid.push(c.to_string().parse()?);
            }
        }
        Ok(b)
    }

    fn valid(&self, p: usize, n: u8) -> bool {
        let x = p % 9;
        let y = p / 9;
        // Check the column and row but exclude the position being checked.
        for i in 0..9 {
            if i != x && self.grid[y * 9 + i] == n {
                return false;
            }
            if i != y && self.grid[i * 9 + x] == n {
                return false;
            }
        }

        // Check the containing box.
        let x0 = (x / 3) * 3;
        let y0 = (y / 3) * 3;
        for dx in 0..3 {
            for dy in 0..3 {
                // Ignore the position being checked.
                if (y0 + dy) == y && (x0 + dx) == x {
                    continue;
                }
                if self.grid[(y0 + dy) * 9 + x0 + dx] == n {
                    return false;
                }
            }
        }
        return true;
    }

    fn solved(&self) -> bool {
        for (p, n) in self.grid.iter().enumerate() {
            if *n == 0 || !self.valid(p, *n) {
                return false;
            }
        }
        true
    }

    /// Solve the board. Returns true on success and false if no
    /// solution was found.
    pub fn solve(&mut self) -> bool {
        self.solve_helper(self.next_unsolved(0))
    }

    fn next_unsolved(&self, p: usize) -> usize {
        for i in p..81 {
            if self.grid[i] == 0 {
                return i;
            }
        }
        return 81;
    }

    fn solve_helper(&mut self, p: usize) -> bool {
        // Check to see if we have reached the end.
        if p == 81 {
            return self.solved();
        }

        // We are at an unsolved square. Let's try different values.
        for n in 1..10 {
            // Try all valid positions.
            if self.valid(p, n) {
                // Check to see if this was a solution.
                self.grid[p] = n;

                if self.solve_helper(self.next_unsolved(p + 1)) {
                    return true;
                }
            }
        }

        // If we've tried them all, this one isn't the solution.
        self.grid[p] = 0;
        return false;
    }
}

impl std::string::ToString for Board {
    fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..9 {
            if y % 3 == 0 && y != 0 {
                s.push_str(&format!("------+-------+------\n"));
            }
            s.push_str(&format!(
                "{} {} {} | {} {} {} | {} {} {}\n",
                self.grid[y * 9 + 0],
                self.grid[y * 9 + 1],
                self.grid[y * 9 + 2],
                self.grid[y * 9 + 3],
                self.grid[y * 9 + 4],
                self.grid[y * 9 + 5],
                self.grid[y * 9 + 6],
                self.grid[y * 9 + 7],
                self.grid[y * 9 + 8]
            ));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid() {
        let tests = vec![
            (
                true,
                0,
                1,
                "023456789000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                2,
                "023456789000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                1,
                "000000001000000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                1,
                "000000000100000000000000000000000000000000000000000000000000000000000000000000000",
            ),
            (
                false,
                0,
                1,
                "000000000000000000000000000000000000000000000000000000000000000000000000100000000",
            ),
        ];
        for (n, test) in tests.iter().enumerate() {
            let b = Board::new(test.3.to_string()).unwrap();
            assert_eq!(b.valid(test.1, test.2), test.0, "test {}", n);
        }
    }

    #[test]
    fn solvable() {
        let mut board = Board::new(
            "120400586060201403040096000090000014081000360430000070000720030608903040372008051"
                .to_string(),
        )
        .unwrap();

        assert_eq!(board.solve(), true);
    }

    #[test]
    fn to_string() {
        let mut board = Board::new(
            "120400586060201403040096000090000014081000360430000070000720030608903040372008051"
                .to_string(),
        )
        .unwrap();
        board.solve();
        assert_eq!(board.to_string(), "1 2 9 | 4 3 7 | 5 8 6\n8 6 7 | 2 5 1 | 4 9 3\n5 4 3 | 8 9 6 | 1 2 7\n------+-------+------\n7 9 5 | 3 6 2 | 8 1 4\n2 8 1 | 5 7 4 | 3 6 9\n4 3 6 | 1 8 9 | 2 7 5\n------+-------+------\n9 1 4 | 7 2 5 | 6 3 8\n6 5 8 | 9 1 3 | 7 4 2\n3 7 2 | 6 4 8 | 9 5 1\n");
    }

    #[test]
    fn board_new_with_various_blanks() {
        let board = Board::new(
            "240000789308000016001800023034502698 ._030070000060030087000902053000801600084357"
                .to_string(),
        )
        .unwrap();
        assert_eq!(
            board.grid,
            vec![
                2, 4, 0, 0, 0, 0, 7, 8, 9, 3, 0, 8, 0, 0, 0, 0, 1, 6, 0, 0, 1, 8, 0, 0, 0, 2, 3, 0,
                3, 4, 5, 0, 2, 6, 9, 8, 0, 0, 0, 0, 3, 0, 0, 7, 0, 0, 0, 0, 0, 6, 0, 0, 3, 0, 0, 8,
                7, 0, 0, 0, 9, 0, 2, 0, 5, 3, 0, 0, 0, 8, 0, 1, 6, 0, 0, 0, 8, 4, 3, 5, 7
            ]
        );
    }

    #[test]
    #[should_panic(expected = r#"string must contain only digits"#)]
    fn board_new_non_numeric() {
        Board::new(
            "240000789308000016001800023034502698 ._03007000006003008700090205300080160008435a"
                .to_string(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic(expected = r#"string must be exactly 81 characters"#)]
    fn board_new_too_short() {
        Board::new("24000078930800001600180002".to_string()).unwrap();
    }

    #[test]
    #[should_panic(expected = r#"string must be exactly 81 characters"#)]
    fn board_new_too_long() {
        Board::new(
            "240000789308000016000000001800000000000000000000000000000000000000000000000000000000000002"
                .to_string(),
        ).unwrap();
    }
}
