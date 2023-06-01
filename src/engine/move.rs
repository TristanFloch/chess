use crate::engine::piece::PieceType;
use crate::engine::position::Position;

#[derive(Debug)]
pub struct Move {
    pub start: Position,
    pub end: Position,
    pub piece_type: PieceType,

    pub is_capture: bool,
}

impl Move {
    pub fn new(start: Position, end: Position, piece_type: PieceType) -> Self {
        Self {
            start,
            end,
            piece_type,

            is_capture: false,
        }
    }
}
