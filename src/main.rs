use crate::{board::Board, pieces::{Color, Piece}};

mod board;
mod pieces;

fn main() {
    let mut b = Board::new();
    b.set_piece(Color::White, Piece::Pawn, 0);
    b.set_piece(Color::Black, Piece::King, 62);
    b.print_board();
}
