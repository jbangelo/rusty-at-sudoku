use itertools::Itertools;
use std::fmt;
use std::io::{BufRead, BufReader, Read};

const MAX_ROWS: usize = 9;
const MAX_COLS: usize = 9;
const MAX_SQUARES: usize = 9;
const MAX_INDEX: usize = MAX_COLS * MAX_ROWS;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Field {
    Empty,
    Filled(u8),
}

use Field::{Empty, Filled};

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Empty => write!(f, "*"),
            Filled(value) => write!(f, "{}", value),
        }
    }
}

pub struct Sudoku {
    fields: [Field; MAX_INDEX],
}

/// A Sudoku puzzle, either partial or complete
impl Sudoku {
    pub fn read_from<R: Read>(source: R) -> Option<Sudoku> {
        let source = BufReader::new(source);
        let mut fields = [Empty; MAX_INDEX];
        let mut index = 0;

        for line in source.lines() {
            for square in line.ok()?.split_whitespace() {
                if square.len() != 1 {
                    return None;
                }
                let square = square.chars().next()?;

                if square.is_digit(10) {
                    let digit = square.to_digit(10)?;
                    if digit >= 1 && digit <= 9 {
                        fields[index] = Filled(digit as u8);
                    } else {
                        return None;
                    }
                } else if square == '*' {
                    fields[index] = Empty
                } else {
                    return None;
                }

                index += 1;
            }
        }

        Some(Sudoku { fields })
    }

    fn set_field(&mut self, index: usize, value: u8) {
        self.fields[index] = Filled(value);
    }

    pub fn into_iter(self) -> impl Iterator<Item = Field> {
        self.fields.into_iter()
    }

    fn get_first_empty_index(&self) -> Option<usize> {
        self.fields
            .iter()
            .enumerate()
            .find(|(_index, &field)| field == Empty)
            .map(|(index, _field)| index)
    }

    fn get_possible_values(&self, index: usize) -> Vec<u8> {
        let possible_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let row = self.row_of(index);
        let col = self.col_of(index);
        let square = self.square_of(index);

        possible_values
            .into_iter()
            .filter(|value| {
                let not_in_row = !row.into_iter().contains(&Filled(*value));
                let not_in_col = !col.into_iter().contains(&Filled(*value));
                let not_in_square = !square.into_iter().contains(&Filled(*value));

                not_in_row && not_in_col && not_in_square
            })
            .collect()
    }

    pub fn solve(self) -> Sudoku {
        Self::solve_impl(self).unwrap()
    }

    fn solve_impl(puzzle: Sudoku) -> Option<Sudoku> {
        let index = puzzle.get_first_empty_index();

        match index {
            None => {
                if puzzle.is_valid() {
                    Some(puzzle)
                } else {
                    None
                }
            }
            Some(index) => {
                let possible_values = puzzle.get_possible_values(index);
                possible_values
                    .into_iter()
                    .fold(None, |prev_result, value| {
                        if prev_result.is_some() {
                            return prev_result;
                        }

                        let mut puzzle = puzzle.clone();
                        puzzle.set_field(index, value);
                        if let Some(answer) = Self::solve_impl(puzzle) {
                            return Some(answer);
                        } else {
                            None
                        }
                    })
            }
        }
    }

    fn is_valid(&self) -> bool {
        for row in self.rows() {
            let is_filled = !row.iter().contains(&Empty);
            let is_unique = row.iter().unique().count() == 9;

            if !is_filled || !is_unique {
                return false;
            }
        }

        for col in self.cols() {
            let is_filled = !col.into_iter().contains(&Empty);
            let is_unique = col.into_iter().unique().count() == 9;

            if !is_filled || !is_unique {
                return false;
            }
        }

        for square in self.squares() {
            let is_filled = !square.into_iter().contains(&Empty);
            let is_unique = square.into_iter().unique().count() == 9;

            if !is_filled || !is_unique {
                return false;
            }
        }

        true
    }

    fn rows(&self) -> Rows {
        Rows {
            puzzle: self,
            curr_index: 0,
        }
    }

    fn row_of(&self, index: usize) -> <Rows as Iterator>::Item {
        let row_index = index / MAX_COLS;
        self.rows().nth(row_index).unwrap()
    }

    fn cols(&self) -> Cols {
        Cols {
            puzzle: self,
            curr_col: 0,
        }
    }

    fn col_of(&self, index: usize) -> <Cols as Iterator>::Item {
        let col_index = index % MAX_ROWS;
        self.cols().nth(col_index).unwrap()
    }

    fn squares(&self) -> Squares {
        Squares {
            puzzle: self,
            curr_square: 0,
        }
    }

    fn square_of(&self, index: usize) -> <Squares as Iterator>::Item {
        let col_index = (index % 9) / 3;
        let row_index = (index / 9) / 3;
        let square_index = col_index + 3 * row_index;
        self.squares().nth(square_index).unwrap()
    }
}

impl Clone for Sudoku {
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
        }
    }
}

// impl Copy for Sudoku {}

impl PartialEq for Sudoku {
    fn eq(&self, other: &Self) -> bool {
        self.fields == other.fields
    }
}

impl Eq for Sudoku {}

impl fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        for row in self.rows() {
            for element in row {
                write!(f, "{:?} ", element)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

struct Rows<'a> {
    puzzle: &'a Sudoku,
    curr_index: usize,
}

impl<'a> Iterator for Rows<'a> {
    type Item = &'a [Field];

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index >= MAX_INDEX {
            None
        } else {
            let result = &self.puzzle.fields[self.curr_index..self.curr_index + MAX_COLS];
            self.curr_index += MAX_COLS;
            Some(result)
        }
    }
}

struct Col<'a> {
    puzzle: &'a Sudoku,
    start_index: usize,
}

impl<'a> IntoIterator for &Col<'a> {
    type Item = Field;
    type IntoIter = ColIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ColIter {
            puzzle: self.puzzle,
            curr_index: self.start_index,
        }
    }
}

struct ColIter<'a> {
    puzzle: &'a Sudoku,
    curr_index: usize,
}

impl<'a> Iterator for ColIter<'a> {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index >= MAX_INDEX {
            None
        } else {
            let result = self.puzzle.fields[self.curr_index];
            self.curr_index += MAX_ROWS;
            Some(result)
        }
    }
}

struct Cols<'a> {
    puzzle: &'a Sudoku,
    curr_col: usize,
}

impl<'a> Iterator for Cols<'a> {
    type Item = Col<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_col >= MAX_COLS {
            None
        } else {
            let result = Col {
                puzzle: self.puzzle,
                start_index: self.curr_col,
            };
            self.curr_col += 1;
            Some(result)
        }
    }
}

struct Square<'a> {
    puzzle: &'a Sudoku,
    square: usize,
}

impl<'a> IntoIterator for &Square<'a> {
    type Item = Field;
    type IntoIter = SquareIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SquareIter {
            puzzle: self.puzzle,
            square: self.square,
            curr_index: 0,
        }
    }
}

struct SquareIter<'a> {
    puzzle: &'a Sudoku,
    square: usize,
    curr_index: usize,
}

impl<'a> Iterator for SquareIter<'a> {
    type Item = Field;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_index >= MAX_SQUARES {
            None
        } else {
            let square_col_index = self.curr_index % 3;
            let square_row_index = self.curr_index / 3;

            let col_index = square_col_index + 3 * (self.square % 3);
            let row_index = square_row_index + 3 * (self.square / 3);

            let index = col_index + MAX_COLS * row_index;
            let result = self.puzzle.fields[index];
            self.curr_index += 1;
            Some(result)
        }
    }
}

struct Squares<'a> {
    puzzle: &'a Sudoku,
    curr_square: usize,
}

impl<'a> Iterator for Squares<'a> {
    type Item = Square<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr_square >= MAX_SQUARES {
            None
        } else {
            let result = Square {
                puzzle: self.puzzle,
                square: self.curr_square,
            };
            self.curr_square += 1;
            Some(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Empty, Field, Sudoku};

    #[test]
    fn empty_puzzle() {
        let str_puzzle = "* * * * * * * * *\n".to_owned()
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n"
            + "* * * * * * * * *\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for field in puzzle.into_iter() {
            assert_eq!(field, Empty);
        }
    }

    #[test]
    fn full_puzzle() {
        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();
        let mut expectation = 1;

        for field in puzzle.into_iter() {
            assert_eq!(field, Field::Filled(expectation));

            expectation += 1;
            if expectation > 9 {
                expectation = 1;
            }
        }
    }

    #[test]
    fn invalid_parse() {
        let str_puzzle = "1 22 3 4 5 6 7 8 9\n".to_owned()
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n";

        assert!(Sudoku::read_from(str_puzzle.as_bytes()).is_none());

        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "1 2 3 4 5 6 L 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n";

        assert!(Sudoku::read_from(str_puzzle.as_bytes()).is_none());

        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 $ 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n"
            + "1 2 3 4 5 6 7 8 9\n";

        assert!(Sudoku::read_from(str_puzzle.as_bytes()).is_none());
    }

    #[test]
    fn row_iter() {
        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "2 3 4 5 6 7 8 9 1\n"
            + "3 4 5 6 7 8 9 1 2\n"
            + "4 5 6 7 8 9 1 2 3\n"
            + "5 6 7 8 9 1 2 3 4\n"
            + "6 7 8 9 1 2 3 4 5\n"
            + "7 8 9 1 2 3 4 5 6\n"
            + "8 9 1 2 3 4 5 6 7\n"
            + "9 1 2 3 4 5 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for (starting_index, row) in puzzle.rows().enumerate() {
            let mut expectations = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(Field::Filled);
            expectations.rotate_left(starting_index);

            itertools::assert_equal(row, &expectations);
        }
    }

    #[test]
    fn row_of() {
        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "2 3 4 5 6 7 8 9 1\n"
            + "3 4 5 6 7 8 9 1 2\n"
            + "4 5 6 7 8 9 1 2 3\n"
            + "5 6 7 8 9 1 2 3 4\n"
            + "6 7 8 9 1 2 3 4 5\n"
            + "7 8 9 1 2 3 4 5 6\n"
            + "8 9 1 2 3 4 5 6 7\n"
            + "9 1 2 3 4 5 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for row_index in 0..9 {
            for col_index in 0..9 {
                let mut expectations = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(Field::Filled);
                expectations.rotate_left(row_index);

                let index = col_index + 9 * row_index;
                let row = puzzle.row_of(index);

                itertools::assert_equal(row, &expectations);
            }
        }
    }

    #[test]
    fn col_iter() {
        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "2 3 4 5 6 7 8 9 1\n"
            + "3 4 5 6 7 8 9 1 2\n"
            + "4 5 6 7 8 9 1 2 3\n"
            + "5 6 7 8 9 1 2 3 4\n"
            + "6 7 8 9 1 2 3 4 5\n"
            + "7 8 9 1 2 3 4 5 6\n"
            + "8 9 1 2 3 4 5 6 7\n"
            + "9 1 2 3 4 5 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for (starting_index, col) in puzzle.cols().enumerate() {
            let mut expectations = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(Field::Filled);
            expectations.rotate_left(starting_index);

            itertools::assert_equal(col.into_iter(), expectations);
        }
    }

    #[test]
    fn col_of() {
        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "2 3 4 5 6 7 8 9 1\n"
            + "3 4 5 6 7 8 9 1 2\n"
            + "4 5 6 7 8 9 1 2 3\n"
            + "5 6 7 8 9 1 2 3 4\n"
            + "6 7 8 9 1 2 3 4 5\n"
            + "7 8 9 1 2 3 4 5 6\n"
            + "8 9 1 2 3 4 5 6 7\n"
            + "9 1 2 3 4 5 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for row_index in 0..9 {
            for col_index in 0..9 {
                let mut expectations = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(Field::Filled);
                expectations.rotate_left(row_index);

                let index = row_index + 9 * col_index;
                let col = puzzle.col_of(index);

                itertools::assert_equal(col.into_iter(), expectations);
            }
        }
    }

    #[test]
    fn square_iter() {
        let str_puzzle = "1 2 3 2 3 4 3 4 5\n".to_owned()
            + "4 5 6 5 6 7 6 7 8\n"
            + "7 8 9 8 9 1 9 1 2\n"
            + "4 5 6 5 6 7 6 7 8\n"
            + "7 8 9 8 9 1 9 1 2\n"
            + "1 2 3 2 3 4 3 4 5\n"
            + "7 8 9 8 9 1 9 1 2\n"
            + "1 2 3 2 3 4 3 4 5\n"
            + "4 5 6 5 6 7 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for (starting_index, square) in puzzle.squares().enumerate() {
            let mut expectations = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(Field::Filled);
            expectations.rotate_left(starting_index);

            itertools::assert_equal(square.into_iter(), expectations);
        }
    }

    #[test]
    fn square_of() {
        let str_puzzle = "1 2 3 2 3 4 3 4 5\n".to_owned()
            + "4 5 6 5 6 7 6 7 8\n"
            + "7 8 9 8 9 1 9 1 2\n"
            + "4 5 6 5 6 7 6 7 8\n"
            + "7 8 9 8 9 1 9 1 2\n"
            + "1 2 3 2 3 4 3 4 5\n"
            + "7 8 9 8 9 1 9 1 2\n"
            + "1 2 3 2 3 4 3 4 5\n"
            + "4 5 6 5 6 7 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        for row_index in 0..9 {
            for col_index in 0..9 {
                let mut expectations = [1, 2, 3, 4, 5, 6, 7, 8, 9].map(Field::Filled);
                let rotate_amount = (col_index % 3) + 3 * (row_index / 3);
                expectations.rotate_left(rotate_amount);

                let index = (((3 * col_index) % 9) + col_index / 3) + (row_index * 9);
                let square = puzzle.square_of(index);

                itertools::assert_equal(square.into_iter(), expectations);
            }
        }
    }

    #[test]
    fn is_valid() {
        let str_puzzle = "1 2 3 4 5 6 7 8 9\n".to_owned()
            + "4 5 6 7 8 9 1 2 3\n"
            + "7 8 9 1 2 3 4 5 6\n"
            + "2 3 4 5 6 7 8 9 1\n"
            + "5 6 7 8 9 1 2 3 4\n"
            + "8 9 1 2 3 4 5 6 7\n"
            + "3 4 5 6 7 8 9 1 2\n"
            + "6 7 8 9 1 2 3 4 5\n"
            + "9 1 2 3 4 5 6 7 8\n";

        let puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        assert!(puzzle.is_valid());
    }

    #[test]
    fn easy_solve() {
        let str_puzzle = "* 8 6 * 4 1 * 3 9\n".to_owned()
            + "* 4 * * * 7 8 * *\n"
            + "* * 9 * * 6 2 4 *\n"
            + "7 3 * * * 4 6 * *\n"
            + "1 * * 2 * * * 9 5\n"
            + "* * * 6 5 * * 7 4\n"
            + "* * 2 * 6 9 5 * 3\n"
            + "8 * * 3 1 * * * 2\n"
            + "6 5 3 * * * 9 * *\n";

        let mut puzzle = Sudoku::read_from(str_puzzle.as_bytes()).unwrap();

        puzzle.solve();

        let str_answer = "2 8 6 5 4 1 7 3 9\n".to_owned()
            + "3 4 1 9 2 7 8 5 6\n"
            + "5 7 9 8 3 6 2 4 1\n"
            + "7 3 5 1 9 4 6 2 8\n"
            + "1 6 4 2 7 8 3 9 5\n"
            + "9 2 8 6 5 3 1 7 4\n"
            + "4 1 2 7 6 9 5 8 3\n"
            + "8 9 7 3 1 5 4 6 2\n"
            + "6 5 3 4 8 2 9 1 7\n";
        let answer = Sudoku::read_from(str_answer.as_bytes()).unwrap();

        assert_eq!(puzzle, answer);
    }
}
