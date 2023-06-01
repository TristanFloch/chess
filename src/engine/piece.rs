#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Color {
    White = 0,
    Black,
}

impl From<usize> for Color {
    fn from(i: usize) -> Self {
        match i {
            x if x == Color::White as usize => Color::White,
            x if x == Color::Black as usize => Color::Black,
            _ => panic!("cannot convert {} to Color", i),
        }
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl From<PieceType> for char {
    fn from(p: PieceType) -> Self {
        match p {
            PieceType::Pawn => 'P',
            PieceType::Knight => 'N',
            PieceType::Bishop => 'B',
            PieceType::Rook => 'R',
            PieceType::Queen => 'Q',
            PieceType::King => 'K',
        }
    }
}

impl From<usize> for PieceType {
    fn from(i: usize) -> Self {
        match i {
            x if x == PieceType::Pawn as usize => PieceType::Pawn,
            x if x == PieceType::Knight as usize => PieceType::Knight,
            x if x == PieceType::Bishop as usize => PieceType::Bishop,
            x if x == PieceType::Rook as usize => PieceType::Rook,
            x if x == PieceType::Queen as usize => PieceType::Queen,
            x if x == PieceType::King as usize => PieceType::King,
            _ => panic!("cannot convert {} to PieceType", i),
        }
    }
}
