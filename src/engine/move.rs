use crate::engine::position::Position;
use crate::engine::piece::PieceType;

pub struct Move {
    start: Position,
    end: Position,
    piece_type: PieceType,
}
