use crate::engine::piece::PieceType;
use crate::engine::position::Position;

#[derive(Debug)]
pub struct Move {
    pub start: Position,
    pub end: Position,
    pub piece_type: PieceType,
}
