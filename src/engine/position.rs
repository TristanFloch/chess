#[derive(PartialEq, Copy, Clone)]
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

#[derive(PartialEq, Copy, Clone)]
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
    pub rank: Rank,
    pub file: File,
}
