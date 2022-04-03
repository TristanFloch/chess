#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Rank {
    One = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<usize> for Rank {
    fn from(i: usize) -> Self {
        match i {
            x if x == Rank::One as usize => Rank::One,
            x if x == Rank::Two as usize => Rank::Two,
            x if x == Rank::Three as usize => Rank::Three,
            x if x == Rank::Four as usize => Rank::Four,
            x if x == Rank::Five as usize => Rank::Five,
            x if x == Rank::Six as usize => Rank::Six,
            x if x == Rank::Seven as usize => Rank::Seven,
            x if x == Rank::Eight as usize => Rank::Eight,
            _ => panic!("cannot convert {} to a rank", i),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
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

impl From<usize> for File {
    fn from(i: usize) -> Self {
        match i {
            x if x == File::A as usize => File::A,
            x if x == File::B as usize => File::B,
            x if x == File::C as usize => File::C,
            x if x == File::D as usize => File::D,
            x if x == File::E as usize => File::E,
            x if x == File::F as usize => File::F,
            x if x == File::G as usize => File::G,
            x if x == File::H as usize => File::H,
            _ => panic!("cannot convert {} to a file", i),
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Position {
    pub rank: Rank,
    pub file: File,
}

impl From<u64> for Position {
    fn from(b: u64) -> Self {
        let index = (u64::BITS - (u64::MAX ^ b).leading_ones() - 1) as usize;
        // let index = (b & !(b - 1)).trailing_zeros() as usize; // msb index

        Position {
            rank: (index / 8).into(),
            file: (index % 8).into(),
        }
    }
}
