use std::io::{self, Write};

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let puzzle = rusty_at_sudoku::Sudoku::read_from(stdin).unwrap().solve();
    write!(stdout.lock(), "{:?}\n", &puzzle).unwrap();
}
