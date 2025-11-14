use crate::{
    board::{Board, square::Square},
    pieces::{Color, Piece},
};

mod board;
mod pieces;

fn main() {
    let mut b = Board::new();
    b.set_piece(Color::White, Piece::Pawn, Square::A1);
    b.set_piece(Color::Black, Piece::King, Square::H7);
    b.print_board();
}
