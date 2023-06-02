use crate::engine::bits::BitOperations;
use crate::engine::board::Board;
use crate::engine::piece::{Color, PieceType};
use crate::engine::position::Position;
use crate::engine::r#move::Move;
use crate::engine::rules_bb::*;

const NOT_A_FILE: u64 = 0xfefefefefefefefe;
const NOT_H_FILE: u64 = 0x7f7f7f7f7f7f7f7f;
const ONE_RANK: u64 = 0xff;
const H_FILE: u64 = 0x101010101010101;
const DIAG: u64 = 0x8040201008040201;
const ANTI_DIAG: u64 = 0x102040810204080;

fn east_one(b: u64) -> u64 {
    (b << 1) & NOT_A_FILE
}

fn west_one(b: u64) -> u64 {
    (b >> 1) & NOT_H_FILE
}

fn diag_mask(sq: usize) -> u64 {
    let diag = (sq & 7) as isize - (sq >> 3) as isize;
    if diag >= 0 {
        DIAG >> diag * 8
    } else {
        DIAG << -diag * 8
    }
}

fn anti_diag_mask(sq: usize) -> u64 {
    let diag = 7 - (sq & 7) as isize - (sq >> 3) as isize;
    if diag >= 0 {
        ANTI_DIAG >> diag * 8
    } else {
        ANTI_DIAG << -diag * 8
    }
}

fn gen_attack_vec(pos: Position, mut attacks: u64, piece: PieceType, enemies: u64) -> Vec<Move> {
    let mut v = Vec::with_capacity(attacks.count_ones() as usize);

    while attacks != 0 {
        let sq = attacks.lsb_index();
        v.push(Move {
            start: pos.clone(),
            end: sq.into(),
            piece_type: piece.clone(),

            is_capture: enemies.test_bit(sq),
        });
        attacks.lsb_pop();
    }

    v
}

pub fn generate_pawn_moves(board: &Board) -> Vec<Move> {
    let mut pawns = board[PieceType::Pawn];
    let mut attacks = if board.side_to_move == Color::White {
        pawns << 8
    } else {
        pawns >> 8
    };

    let mut v = Vec::with_capacity(attacks.count_ones() as usize);
    while attacks != 0 {
        v.push(Move {
            start: pawns.into(),
            end: attacks.into(),
            piece_type: PieceType::Pawn,

            is_capture: false,
        });
        pawns.lsb_pop();
        attacks.lsb_pop();
    }

    v
}

pub fn generate_knight_moves(board: &Board) -> Vec<Move> {
    let mut knights = board[PieceType::Knight];
    let mut v = Vec::new();
    let enemies = board.enemies_bb();

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

        v.append(&mut gen_attack_vec(
            pos,
            attacks,
            PieceType::Knight,
            enemies,
        ));
        knights.lsb_pop();
    }

    v
}

pub fn generate_bishop_moves(board: &Board) -> Vec<Move> {
    let enemies = board.enemies_bb();
    let friends = board.friends_bb();
    let blockers = enemies | friends;

    let mut bishops = board[PieceType::Bishop];
    let mut v = Vec::new();

    while bishops != 0 {
        let sq = bishops.lsb_pop();
        let pos = Position::from(sq);
        let attacks = exclude_friends(bishop_attacks_bb(sq, blockers), friends);
        v.append(&mut gen_attack_vec(
            pos,
            attacks,
            PieceType::Bishop,
            enemies,
        ));
    }

    v
}

pub fn generate_rook_moves(board: &Board) -> Vec<Move> {
    let enemies = board.enemies_bb();
    let friends = board.friends_bb();
    let blockers = enemies | friends;

    let mut rooks = board[PieceType::Rook];
    let mut v = Vec::new();

    while rooks != 0 {
        let sq = rooks.lsb_pop();
        let pos = Position::from(sq);
        let attacks = exclude_friends(rook_attacks_bb(sq, blockers), friends);
        v.append(&mut gen_attack_vec(pos, attacks, PieceType::Rook, enemies));
    }

    v
}

pub fn generate_queen_moves(board: &Board) -> Vec<Move> {
    let mut queens = board[PieceType::Queen];
    let enemies = board.enemies_bb();
    let mut v = Vec::new();

    while queens != 0 {
        let index = queens.lsb_pop();
        let curr_bb = 1u64 << index;
        let pos = Position::from(curr_bb);
        let rook_attacks =
            ((ONE_RANK << (8 * pos.rank as usize)) | (H_FILE << pos.file as usize)) ^ curr_bb;
        let bishop_attacks =
            (diag_mask(index as usize) | anti_diag_mask(index as usize)) ^ 1 << index;

        v.append(&mut gen_attack_vec(
            pos,
            rook_attacks | bishop_attacks,
            PieceType::Queen,
            enemies,
        ));
    }

    v
}

pub fn generate_king_moves(board: &Board) -> Vec<Move> {
    let king = board[PieceType::King];
    let enemies = board.enemies_bb();
    let friends = board.friends_bb();

    let sq = king.lsb_index();
    let attacks = exclude_friends(king_attacks_bb(sq), friends);

    let start = Position::from(sq);

    gen_attack_vec(start, attacks, PieceType::King, enemies)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::engine::board::tests::print_u64;

    pub fn moves_to_u64(moves: &Vec<Move>) -> u64 {
        moves.iter().fold(0u64, |b, m| {
            b | (1 << (m.end.rank as u8 * 8 + m.end.file as u8))
        })
    }

    #[allow(dead_code)]
    fn print_moves(moves: &Vec<Move>) {
        let board = moves_to_u64(&moves);
        print_u64(board);
    }

    #[test]
    fn king_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::King] = 0x800000000; // d5
        let res = generate_king_moves(&board);
        assert_eq!(0x1c141c000000, moves_to_u64(&res));

        board[PieceType::King] = 0x1; // a1
        let res = generate_king_moves(&board);
        assert_eq!(0x302, moves_to_u64(&res));

        board[PieceType::King] = 0x80; // h8
        let res = generate_king_moves(&board);
        assert_eq!(0xc040, moves_to_u64(&res));

        board[PieceType::King] = 0x1000000000000000; // e8
        let res = generate_king_moves(&board);
        assert_eq!(0x2838000000000000, moves_to_u64(&res));
    }

    #[test]
    fn king_moves_blockers() {
        let mut board = Board::empty();
        board[PieceType::King] = 0x1000000000000000; // e8

        board[PieceType::Pawn] = 0x820000000000000; // d8, f7
        let res = generate_king_moves(&board);
        assert_eq!(0x2018000000000000, moves_to_u64(&res));

        board[PieceType::Pawn] = 0;
        board.set_bb(PieceType::Pawn, Color::Black, 0x820000000000000); // d8, f7
        let res = generate_king_moves(&board);
        assert_eq!(0x2838000000000000, moves_to_u64(&res));
    }

    #[test]
    fn knight_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::Knight] = 0x42; // initial white pos
        let res = generate_knight_moves(&board);
        assert_eq!(0xa51800, moves_to_u64(&res));

        board[PieceType::Knight] = 0x400000000; // one knight on f5
        let res = generate_knight_moves(&board);
        assert_eq!(0xa1100110a0000, moves_to_u64(&res));

        board[PieceType::Knight] = 0; // no knight
        let res = generate_knight_moves(&board);
        assert!(res.is_empty());
    }

    #[test]
    fn rook_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::Rook] = 0x200100000000; // c6 & h5
        let res = generate_rook_moves(&board);
        assert_eq!(0x2121dffe21212121, moves_to_u64(&res));
        assert_eq!(28, res.len());
    }

    #[test]
    fn rook_moves_blockers() {
        let mut board = Board::empty();
        board[PieceType::Rook] = 0x200100000000; // c6 & h5

        board[PieceType::Pawn] = 0x2100444000002100; // lots of blockers
        let res = generate_rook_moves(&board);
        assert_eq!(0x21193e21210000, moves_to_u64(&res));

        board[PieceType::Pawn] = 0u64;
        board.set_bb(PieceType::Pawn, Color::Black,  0x2100444000002100);
        let res = generate_rook_moves(&board);
        assert_eq!(0x21215d7e21212100, moves_to_u64(&res));
    }

    #[test]
    fn bishop_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::Bishop] = 0x4000000; // c4
        let res = generate_bishop_moves(&board);
        assert_eq!(0x4020110a000a1120, moves_to_u64(&res));

        board[PieceType::Bishop] = 0x200000; // f3
        let res = generate_bishop_moves(&board);
        assert_eq!(0x102048850005088, moves_to_u64(&res));

        board[PieceType::Bishop] = 0x80000000000000; // H7
        let res = generate_bishop_moves(&board);
        assert_eq!(0x4000402010080402, moves_to_u64(&res));

        board[PieceType::Bishop] = 0x8000000000000; // D7
        let res = generate_bishop_moves(&board);
        assert_eq!(0x1400142241800000, moves_to_u64(&res));
    }

    #[test]
    fn bishop_moves_blockers() {
        let mut board = Board::empty();
        board[PieceType::Bishop] = 0x4000000; // c4

        board[PieceType::Pawn] = 0x20010020020000; // a6, b3, f7, f4
        let res = generate_bishop_moves(&board);
        assert_eq!(0x100a00081020, moves_to_u64(&res));

        board[PieceType::Pawn] = 0u64; // remove white blockers
        board.set_bb(PieceType::Pawn, Color::Black, 0x20010020020000); // a6, b3, f7, f4
        let res = generate_bishop_moves(&board);
        assert_eq!(0x20110a000a1020, moves_to_u64(&res));
    }

    #[test]
    fn pawn_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::Pawn] = 0x20429d00;
        let res = generate_pawn_moves(&board);
        assert_eq!(0x20429d0000, moves_to_u64(&res));

        board.side_to_move = Color::Black;
        board[PieceType::Pawn] = 0x20429d00000000;
        let res = generate_pawn_moves(&board);
        assert_eq!(0x20429d000000, moves_to_u64(&res));
    }

    #[test]
    fn queen_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::Queen] = 0x4000000; // C4
        let res = generate_queen_moves(&board);
        assert_eq!(0x4424150efb0e1524, moves_to_u64(&res));

        board[PieceType::Queen] = 0x4008000; // C4 and H2
        let res = generate_queen_moves(&board);
        assert_eq!(0xc6a49d9efbce7fe4, moves_to_u64(&res));
    }
}
