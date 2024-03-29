use crate::engine::bits::BitOperations;
use crate::engine::piece::{Color, PieceType};
use crate::engine::position::Position;
use crate::engine::r#move::Move;
use crate::engine::rules::*;

use colored::Colorize;

#[derive(Clone)]
pub struct Board {
    pub bitboards: [u64; 12],

    pub side_to_move: Color,
    turn: u32,
    white_king_castling: bool,
    white_queen_castling: bool,
    black_king_castling: bool,
    black_queen_castling: bool,
    // TODO en_passant:
    // TODO last_fifty_turn:
}

const COLOR_SWITCH: usize = 6;

impl Board {
    // TODO position method

    pub fn new() -> Self {
        // bitboards computed here:
        // https://gekomad.github.io/Cinnamon/BitboardCalculator/
        // using "little endian file and rank mapping", layout 2 (A1 bit is lsb)
        Self {
            bitboards: [
                0x000000000000ff00, // white pawns
                0x0000000000000042, // white knights
                0x0000000000000024, // white bishops
                0x0000000000000081, // white rooks
                0x0000000000000010, // white queen
                0x0000000000000008, // white king
                0x00ff000000000000, // black pawns
                0x4200000000000000, // black knights
                0x2400000000000000, // black bishops
                0x8100000000000000, // black rooks
                0x1000000000000000, // black queen
                0x0800000000000000, // black king
            ],

            side_to_move: Color::White,
            turn: 0,
            white_king_castling: true,
            white_queen_castling: true,
            black_king_castling: true,
            black_queen_castling: true,
        }
    }

    fn is_move_legal(&self, _m: &Move) -> bool {
        true
    }

    pub fn generate_legal_moves(&self) -> Vec<Move> {
        let mut v = Vec::new();

        let generators: Vec<fn(&Board) -> Vec<Move>> = vec![
            generate_pawn_moves,
            generate_rook_moves,
            generate_knight_moves,
            generate_bishop_moves,
            generate_queen_moves,
            generate_king_moves,
        ];

        for generator in generators {
            v.append(
                &mut (generator)(&self)
                    .into_iter()
                    .filter(|m| self.is_move_legal(m))
                    .collect(),
            );
        }

        v
    }

    pub fn do_move(&mut self, m: &Move) {
        let mut bb = self[m.piece_type];
        bb.toggle_bit(m.start.rank as usize * 8 + m.start.file as usize);
        bb.toggle_bit(m.end.rank as usize * 8 + m.end.file as usize);
        self[m.piece_type] = bb;

        self.side_to_move = if self.side_to_move == Color::White {
            self.turn += 1;
            Color::Black
        } else {
            Color::White
        };
    }

    pub fn enemies_bb(&self) -> u64 {
        let offset = !self.side_to_move as usize * COLOR_SWITCH;

        self.bitboards[offset..(COLOR_SWITCH + offset)]
            .iter()
            .fold(0u64, |res, bb| res | bb)
    }

    pub fn friends_bb(&self) -> u64 {
        let offset = self.side_to_move as usize * COLOR_SWITCH;

        self.bitboards[offset..(COLOR_SWITCH + offset)]
            .iter()
            .fold(0u64, |res, bb| res | bb)
    }

    // TODO
    // fen constructor
    // is_check
    // is_checkmate
    // is_draw
}

impl std::ops::Index<Position> for Board {
    type Output = (PieceType, Color);

    // operator[]: piece type and color from position
    fn index(&self, _pos: Position) -> &Self::Output {
        todo!();
    }
}

impl std::ops::Index<PieceType> for Board {
    type Output = u64;

    fn index(&self, piece: PieceType) -> &Self::Output {
        &self.bitboards[self.side_to_move as usize * COLOR_SWITCH + piece as usize]
    }
}

impl std::ops::IndexMut<PieceType> for Board {
    fn index_mut(&mut self, piece: PieceType) -> &mut u64 {
        &mut self.bitboards[self.side_to_move as usize * COLOR_SWITCH + piece as usize]
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = [(); 8 * 8].map(|_| ".".normal());

        for (piece_index, board) in self.bitboards.iter().enumerate() {
            for j in (0..8).rev() {
                for k in (0..8).rev() {
                    let bit_index = j * 8 + k;
                    if board & (1 << bit_index) != 0 {
                        let c: char = PieceType::from(piece_index % COLOR_SWITCH).into();
                        let s: String = c.into();
                        repr[bit_index] = if piece_index < COLOR_SWITCH {
                            s.cyan()
                        } else {
                            s.purple()
                        };
                    }
                }
            }
        }

        for i in (0..8).rev() {
            write!(f, "{}  ", i + 1)?;
            for j in (0..8).rev() {
                write!(f, "{} ", repr[i * 8 + j])?;
            }
            writeln!(f)?;
        }
        writeln!(f, "\n   A B C D E F G H")
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::engine::position::{File, Rank};
    use crate::engine::rules::tests::moves_to_u64;

    impl Board {
        pub fn empty() -> Self {
            Self {
                bitboards: [0; 12],
                side_to_move: Color::White,
                turn: 0,
                white_king_castling: true,
                white_queen_castling: true,
                black_king_castling: true,
                black_queen_castling: true,
            }
        }

        fn at(&self, piece: PieceType, color: Color) -> u64 {
            self.bitboards[color as usize * COLOR_SWITCH + piece as usize]
        }

        pub fn set_bb(&mut self, piece: PieceType, color: Color, bb: u64) {
            self.bitboards[color as usize * COLOR_SWITCH + piece as usize] = bb;
        }
    }

    pub fn print_u64(b: u64) {
        for i in (0..8).rev() {
            for j in (0..8).rev() {
                let index = i * 8 + j;
                print!("{} ", if b & (1 << index) != 0 { '1' } else { '.' });
            }
            println!();
        }
    }

    #[test]
    fn do_move_e4() {
        let mut board = Board::new();
        let m = Move::new(
            Position {
                rank: Rank::Two,
                file: File::E,
            },
            Position {
                rank: Rank::Four,
                file: File::E,
            },
            PieceType::Pawn,
        );

        assert!(board.is_move_legal(&m));

        board.do_move(&m);

        assert_eq!(board.turn, 1);
        assert_eq!(board.side_to_move, Color::Black);
        assert_eq!(0x1000ef00, board.at(PieceType::Pawn, Color::White));
    }

    #[test]
    fn test_blockers_bb() {
        let board = Board::new();
        assert_eq!(0xffff000000000000, board.enemies_bb());
        assert_eq!(0xffff, board.friends_bb());
    }

    #[test]
    fn legal_moves_pieces_start() {
        let board = Board::new();

        let moves = generate_rook_moves(&board);
        assert_eq!(0, moves.len(), "expected 0, got {}", moves_to_u64(&moves));

        let moves = generate_knight_moves(&board);
        assert_eq!(4, moves.len(), "expected 4, got {}", moves_to_u64(&moves));

        let moves = generate_bishop_moves(&board);
        assert_eq!(0, moves.len(), "expected 0, got {}", moves_to_u64(&moves));

        let moves = generate_queen_moves(&board);
        assert_eq!(0, moves.len(), "expected 0, got {}", moves_to_u64(&moves));

        let moves = generate_king_moves(&board);
        assert_eq!(0, moves.len(), "expected 0, got {}", moves_to_u64(&moves));
    }
}
