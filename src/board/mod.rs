#![allow(dead_code)]

use strum::IntoEnumIterator;

pub mod square;

use crate::{
    board::square::Square,
    pieces::{Bitboard, Color, Piece, piece_to_str},
};

pub struct Board {
    color_bb: [Bitboard; 2],
    piece_bb: [Bitboard; 6],
}

impl Board {
    pub fn new() -> Board {
        Board {
            color_bb: [0; 2],
            piece_bb: [0; 6],
        }
    }

    pub fn get_piece(&mut self, square: Square) -> Option<(Color, Piece)> {
        let index = square.to_index();
        self.get_piece_index(index)
    }

    pub fn get_piece_index(&mut self, index: u8) -> Option<(Color, Piece)> {
        let color = if (self.color_bb[Color::White as usize] >> index) & 1 == 1 {
            Color::White
        } else {
            Color::Black
        };

        for piece in Piece::iter() {
            if (self.piece_bb[piece as usize] >> index) & 1 == 1 {
                return Some((color, piece));
            }
        }

        None
    }

    pub fn set_piece(&mut self, color: Color, piece: Piece, square: Square) {
        let index = square.to_index();
        self.set_piece_index(color, piece, index)
    }

    pub fn set_piece_index(&mut self, color: Color, piece: Piece, index: u8) {
        let setter = (1 as u64) << index;
        self.color_bb[color as usize] |= setter;
        self.piece_bb[piece as usize] |= setter;
    }

    pub fn unset_piece(&mut self, color: Color, piece: Piece, square: Square) {
        let index = square.to_index();
        self.unset_piece_index(color, piece, index)
    }

    pub fn unset_piece_index(&mut self, color: Color, piece: Piece, index: u8) {
        let setter = (1 as u64) << index;
        self.color_bb[color as usize] ^= setter;
        self.piece_bb[piece as usize] ^= setter;
    }

    pub fn show_bitboards(&self) {
        for color in Color::iter() {
            println!("{:?}\n", color);
            println!("{}\n\n", format_bitboard(self.color_bb[color as usize]));
        }

        for piece in Piece::iter() {
            println!("{:?}\n", piece);
            println!("{}\n\n", format_bitboard(self.piece_bb[piece as usize]));
        }
    }

    pub fn print_board(&mut self) {
        let files = ('A'..='H').collect::<String>().replace("", " ");
        println!(" {}", files);
        for row in (0..8).rev() {
            print!("{} ", (('1' as u8) + row) as char);
            for col in 0..8 {
                if let Some((color, piece)) = self.get_piece_index(row * 8 + col) {
                    print!("{} ", piece_to_str(color, piece));
                } else {
                    print!(". ");
                }
            }
            println!();
        }
    }
}

fn format_bitboard(bb: Bitboard) -> String {
    (0..8)
        .map(|row| {
            (0..8)
                .map(|col| {
                    let i = 63 - (row * 8 + col);
                    ((bb >> i) & 1).to_string()
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
