pub mod bitboard;
pub mod castling;
pub mod fen;
pub mod square;

use strum::IntoEnumIterator;

use crate::{
    pieces::{Color, Piece, piece_to_art},
    state::{bitboard::Bitboard, square::Square},
};

#[derive(Debug)]
pub struct GameState {
    pub color_bb: [Bitboard; 2],
    pub piece_bb: [Bitboard; 6],

    pub current_side: Color,
    pub castling: u8,
    pub en_passant: Option<Square>,

    pub halfmove: u8,
    pub fullmove: u8,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            color_bb: [Bitboard(0); 2],
            piece_bb: [Bitboard(0); 6],

            current_side: Color::White,
            castling: 0,
            en_passant: None,

            halfmove: 0,
            fullmove: 1,
        }
    }

    pub fn get_piece_index(&mut self, index: u8) -> Option<(Color, Piece)> {
        let color = if self.color_bb[Color::White].get_bit(index) {
            Color::White
        } else {
            Color::Black
        };

        for piece in Piece::iter() {
            if self.piece_bb[piece].get_bit(index) {
                return Some((color, piece));
            }
        }

        None
    }

    pub fn set_piece_index(&mut self, color: Color, piece: Piece, index: u8) {
        self.color_bb[color].set_bit(index);
        self.piece_bb[piece].set_bit(index);
    }

    pub fn print_board(&mut self) {
        let files = ('A'..='H').collect::<String>().replace("", " ");
        println!(" {}", files);
        for row in (0..8).rev() {
            print!("{} ", (('1' as u8) + row) as char);
            for col in 0..8 {
                if let Some((color, piece)) = self.get_piece_index(row * 8 + col) {
                    print!("{} ", piece_to_art(color, piece));
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }

    pub fn debug_board(&mut self) {
        println!("{:#?}\n", &self);
        self.print_board();
    }
}
