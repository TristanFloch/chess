#[derive(PartialEq)]
pub enum Rank {
    ONE = 0,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
}

#[derive(PartialEq)]
pub enum File {
    A = 0,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(PartialEq)]
pub struct Position {
    rank: Rank,
    file: File,
}
