use crate::engine::piece::PieceType;
use crate::engine::position::Position;

pub struct Move {
    pub start: Position,
    pub end: Position,
    pub piece_type: PieceType,
}
