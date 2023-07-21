use crate::engine::bits::BitOperations;
use crate::engine::board::Board;
use crate::engine::piece::{Color, PieceType};
use crate::engine::position::{self, Position};
use crate::engine::r#move::Move;
use crate::engine::rules_bb::*;

const A_FILE: u64 = 0x101010101010101;

fn gen_attack_vec(pos: Position, mut attacks: u64, piece: PieceType, enemies: u64) -> Vec<Move> {
    let mut v = Vec::with_capacity(attacks.count_ones() as usize);

    while attacks != 0 {
        let sq = attacks.lsb_pop();
        v.push(Move {
            start: pos.clone(),
            end: sq.into(),
            piece_type: piece.clone(),

            is_capture: enemies.test_bit(sq),
        });
    }

    v
}

pub fn generate_pawn_moves(board: &Board) -> Vec<Move> {
    fn find_pusher(sq: usize, pawns: u64) -> Position {
        let file = sq % 8;
        let low_mask = (1u64 << sq) - 1;
        let top_pawn_sq = ((pawns | (A_FILE >> file)) & low_mask).msb_index();
        top_pawn_sq.into()
    }

    let enemies = board.enemies_bb();
    let blockers = enemies | board.friends_bb();
    let empty = !blockers;

    let pawns = board[PieceType::Pawn];
    let single_pushes;
    let double_pushes;
    let west_attacks;
    let west_attack_finder;
    let east_attacks;
    let east_attack_finder;

    if board.side_to_move == Color::White {
        single_pushes = white_pawns_pushes(pawns, empty);
        double_pushes = white_pawns_double_pushes(single_pushes, empty);
        (west_attacks, east_attacks) = white_pawns_attacks(pawns, enemies);
        west_attack_finder = south_east_one;
        east_attack_finder = south_west_one;
    } else {
        single_pushes = black_pawns_pushes(pawns, empty);
        double_pushes = black_pawns_double_pushes(single_pushes, empty);
        attacks = black_pawns_attacks(pawns, enemies);
        attacker_finder = (
            north_east_one as fn(u64) -> u64,
            north_west_one as fn(u64) -> u64,
        );
    }

    let mut v = Vec::with_capacity(
        (single_pushes.count_ones()
            + double_pushes.count_ones()
            + west_attacks.count_ones()
            + east_attacks.count_ones()) as usize,
    );

    for mut targets in [single_pushes, double_pushes] {
        while targets != 0 {
            let sq = targets.lsb_pop();
            v.push(Move::new(
                find_pusher(sq, pawns),
                sq.into(),
                PieceType::Pawn,
            ));
        }
    }

    while west_attacks != 0 {
        let sq = west_attacks.lsb_pop();
        let bb = 1u64 << sq;
        v.push(Move {
            // white only atm
            start: (bb << 7).into(),
            end: sq.into(),
            piece_type: PieceType::Pawn,
            is_capture: true,
        });
    }

    while east_attacks != 0 {
        let sq = east_attacks.lsb_pop();
        let bb = 1u64 << sq;
        v.push(Move {
            // TODO
            start: (bb << 7).into(),
            end: sq.into(),
            piece_type: PieceType::Pawn,
            is_capture: true,
        });
    }

    // TODO captures

    v
}

pub fn generate_knight_moves(board: &Board) -> Vec<Move> {
    let enemies = board.enemies_bb();
    let friends = board.friends_bb();

    let mut knights = board[PieceType::Knight];
    let mut v = Vec::new();

    while knights != 0 {
        let sq = knights.lsb_pop();
        let pos = Position::from(sq);
        let attacks = exclude_friends(knight_attacks_bb(sq), friends);
        v.append(&mut gen_attack_vec(
            pos,
            attacks,
            PieceType::Knight,
            enemies,
        ));
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
    let enemies = board.enemies_bb();
    let friends = board.friends_bb();
    let blockers = enemies | friends;

    let mut queens = board[PieceType::Queen];
    let mut v = Vec::new();

    while queens != 0 {
        let sq = queens.lsb_pop();
        let pos = Position::from(sq);
        let attacks = exclude_friends(queen_attacks_bb(sq, blockers), friends);
        v.append(&mut gen_attack_vec(pos, attacks, PieceType::Queen, enemies));
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

        board[PieceType::Pawn] = 0u64;
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

        board[PieceType::Knight] = 0x400000000; // one knight on c5
        let res = generate_knight_moves(&board);
        assert_eq!(0xa1100110a0000, moves_to_u64(&res));

        board[PieceType::Knight] = 0; // no knight
        let res = generate_knight_moves(&board);
        assert!(res.is_empty());
    }

    #[test]
    fn knight_moves_blockers() {
        let mut board = Board::empty();
        board[PieceType::Knight] = 0x400000000; // one knight on c5

        board[PieceType::Pawn] = 0x8020001080000; // a4, b6, d3, d7
        let res = generate_knight_moves(&board);
        assert_eq!(0x2110010020000, moves_to_u64(&res));

        board[PieceType::Pawn] = 0u64;
        board.set_bb(PieceType::Pawn, Color::Black, 0x8020001080000); // a4, b6, d3, d7
        let res = generate_knight_moves(&board);
        assert_eq!(0xa1100110a0000, moves_to_u64(&res));
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
        board.set_bb(PieceType::Pawn, Color::Black, 0x2100444000002100);
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
        assert_eq!(0x20df9d0000, moves_to_u64(&res));

        board.side_to_move = Color::Black;
        board[PieceType::Pawn] = 0;
        board[PieceType::Pawn] = 0x20429d00000000;
        let res = generate_pawn_moves(&board);
        assert_eq!(0x20629d000000, moves_to_u64(&res));
    }

    #[test]
    fn pawn_moves() {
        let mut board = Board::empty();
        board[PieceType::Pawn] = 0x20000020800d00; // a2, c2, d2, f4, f7, h3

        let res = generate_pawn_moves(&board);
        assert_eq!(0x200000208d0d0000, moves_to_u64(&res));
        assert_eq!(9, res.len());

        let blockers = 0x208001040000u64; // a4, c3, f6, h5
        board[PieceType::Queen] = blockers;
        let res = generate_pawn_moves(&board);
        assert_eq!(0x2000002088090000, moves_to_u64(&res));
        assert_eq!(6, res.len());
    }

    #[test]
    fn queen_moves_empty() {
        let mut board = Board::empty();

        board[PieceType::Queen] = 0x4000000; // c4
        let res = generate_queen_moves(&board);
        assert_eq!(0x4424150efb0e1524, moves_to_u64(&res));

        board[PieceType::Queen] = 0x4008000; // c4 and h2
        let res = generate_queen_moves(&board);
        assert_eq!(0xc6a49d9efbce7fe4, moves_to_u64(&res));
    }

    #[test]
    fn queen_moves_blockers() {
        let mut board = Board::empty();
        board[PieceType::Queen] = 0x4000000; // c4

        board[PieceType::Pawn] = 0x100041040000; // a4, c3, e6, g4
        let res = generate_queen_moves(&board);
        assert_eq!(0x404050e3a0a1120, moves_to_u64(&res));

        board[PieceType::Pawn] = 0u64; // remove white blockers
        board.set_bb(PieceType::Pawn, Color::Black, 0x100041040000); // a4, c3, e6, g4
        let res = generate_queen_moves(&board);
        assert_eq!(0x404150e7b0e1120, moves_to_u64(&res));
    }
}
