use crate::engine::board::Board;
use crate::engine::r#move::Move;

pub fn generate_pawn_moves(board: Board) -> Vec<Move> {
    todo!();
}

pub fn generate_knight_moves(board: Board) -> Vec<Move> {
    todo!();
}

pub fn generate_bishop_moves(board: Board) -> Vec<Move> {
    todo!();
}

pub fn generate_rook_moves(board: Board) -> Vec<Move> {
    todo!();
}

pub fn generate_queen_moves(board: Board) -> Vec<Move> {
    todo!();
}

const NOTAFILE: u64 = 0xfefefefefefefefe; // ~0x0101010101010101
const NOTHFILE: u64 = 0x7f7f7f7f7f7f7f7f; // ~0x8080808080808080

pub fn generate_king_moves(board: Board) -> Vec<Move> {
    return Vec::new();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::piece;

    fn moves_to_u64(moves: &Vec<Move>) -> u64 {
        moves
            .iter()
            .fold(0, |b, m| b | (1 << m.end.file as u8 * 8 + m.end.rank as u8))
    }

    fn print_moves(moves: &Vec<Move>) {
        let board = moves_to_u64(&moves);

        for i in (0..8).rev() {
            for j in 0..8 {
                let index = i * 8 + j;
                print!("{}", if board & (1 << index) != 0 { '1' } else { '.' });
            }
        }
    }

    #[test]
    fn king_moves() {
        let mut board = Board::empty();

        board.bitboards[piece::PieceType::King as usize] = 0x80; // A1
        let mut res = generate_king_moves(board);
        print_moves(&res);
        assert_eq!(0xc040, moves_to_u64(&res));
    }
}
