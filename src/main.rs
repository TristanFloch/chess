mod engine;

use crate::engine::board::Board;

fn main() {
    let b = Board::new();
    println!("{}", b);
}
