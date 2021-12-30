use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let puzzle = rusty_at_sudoku::Sudoku::read_from(stdin).unwrap().solve();
    write!(stdout.lock(), "{:?}\n", &puzzle).unwrap();
}
