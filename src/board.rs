use colored::Colorize;

use crate::piece::{Color, PieceType};

pub struct Board {
    bitboards: [u64; 12],

    side_to_move: Color,
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
        // using layout 2 (A1 is bit lsb)
        Self {
            bitboards: [
                0x000000000000ff00, // white pawns
                0x0000000000000042, // white knights
                0x0000000000000024, // white bishops
                0x0000000000000081, // white rooks
                0x0000000000000008, // white queen
                0x0000000000000010, // white king
                0x00ff000000000000, // black pawns
                0x4200000000000000, // black knights
                0x2400000000000000, // black bishops
                0x8100000000000000, // black rooks
                0x0800000000000000, // black queen
                0x1000000000000000, // black king
            ],

            side_to_move: Color::White,
            turn: 0,
            white_king_castling: true,
            white_queen_castling: true,
            black_king_castling: true,
            black_queen_castling: true,
        }
    }

    // TODO
    // fen constructor
    // generate_legal_moves
    // is_move_legal
    // do_move
    // is_check
    // is_checkmate
    // is_draw
    // operator[]: piece type and color from position
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut repr = [(); 8 * 8].map(|_| ".".normal());

        for (piece_index, board) in self.bitboards.iter().enumerate() {
            for j in (0..8).rev() {
                for k in 0..8 {
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
            for j in 0..8 {
                write!(f, "{} ", repr[i * 8 + j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
