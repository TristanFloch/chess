use crate::engine::bits::BitsOperations;
use crate::engine::board::Board;
use crate::engine::piece::PieceType;
use crate::engine::position::Position;
use crate::engine::r#move::Move;

fn south_one(b: u64) -> u64 {
    b >> 8
}

fn north_one(b: u64) -> u64 {
    b << 8
}

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;

// post shift masks

fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_A_FILE
}

fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_H_FILE
}

pub fn generate_pawn_moves(board: &Board) -> Vec<Move> {
    todo!();
}

pub fn generate_knight_moves(board: &Board) -> Vec<Move> {
    let mut knights = board[PieceType::Knight];
    let mut v = Vec::new();

    while knights != 0 {
        let pos = Position::from(knights);

        let mut east = east_one(knights);
        let mut west = west_one(knights);
        let mut attacks = (east | west) << 16;
        attacks |= (east | west) >> 16;
        east = east_one(east);
        west = west_one(west);
        attacks |= (east | west) << 8;
        attacks |= (east | west) >> 8;

        while attacks != 0 {
            v.push(Move {
                start: pos.clone(),
                end: attacks.into(),
                piece_type: PieceType::Knight,
            });
            attacks = attacks.toggle_bit(attacks.lsb_index());
        }
        knights = knights.toggle_bit(knights.lsb_index());
    }

    v
}

pub fn generate_bishop_moves(board: &Board) -> Vec<Move> {
    todo!();
}

pub fn generate_rook_moves(board: &Board) -> Vec<Move> {
    todo!();
}

pub fn generate_queen_moves(board: &Board) -> Vec<Move> {
    todo!();
}

pub fn generate_king_moves(board: &Board) -> Vec<Move> {
    let mut king = board[PieceType::King];
    let start = Position::from(king);

    let mut attacks = east_one(king) | west_one(king);
    king |= attacks;
    attacks |= north_one(king) | south_one(king);

    let mut v = Vec::with_capacity(attacks.count_ones() as usize);
    while attacks != 0 {
        v.push(Move {
            start: start.clone(),
            end: attacks.into(),
            piece_type: PieceType::King,
        });
        attacks = attacks.toggle_bit(attacks.lsb_index());
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    fn moves_to_u64(moves: &Vec<Move>) -> u64 {
        moves.iter().fold(0u64, |b, m| {
            b | (1 << (m.end.rank as u8 * 8 + m.end.file as u8))
        })
    }

    fn print_moves(moves: &Vec<Move>) {
        let board = moves_to_u64(&moves);

        for i in (0..8).rev() {
            for j in 0..8 {
                let index = i * 8 + j;
                print!("{} ", if board & (1 << index) != 0 { '1' } else { '.' });
            }
            println!();
        }
    }

    #[test]
    fn king_moves() {
        let mut board = Board::empty();

        board.bitboards[PieceType::King as usize] = 0x800000000; // d5
        let mut res = generate_king_moves(&board);
        assert_eq!(0x1c141c000000, moves_to_u64(&res));

        board.bitboards[PieceType::King as usize] = 0x1; // a1
        res = generate_king_moves(&board);
        assert_eq!(0x302, moves_to_u64(&res));

        board.bitboards[PieceType::King as usize] = 0x80; // h8
        res = generate_king_moves(&board);
        assert_eq!(0xc040, moves_to_u64(&res));

        board.bitboards[PieceType::King as usize] = 0x1000000000000000; // e8
        res = generate_king_moves(&board);
        assert_eq!(0x2838000000000000, moves_to_u64(&res));
    }

    #[test]
    fn knight_moves() {
        let mut board = Board::empty();

        board.bitboards[PieceType::Knight as usize] = 0x42; // initial white pos
        let mut res = generate_knight_moves(&board);
        assert_eq!(0xa51800, moves_to_u64(&res));

        board.bitboards[PieceType::Knight as usize] = 0x400000000; // one knight on f5
        res = generate_knight_moves(&board);
        assert_eq!(0xa1100110a0000, moves_to_u64(&res));

        board.bitboards[PieceType::Knight as usize] = 0; // no knight
        res = generate_knight_moves(&board);
        assert!(res.is_empty());
    }
}
