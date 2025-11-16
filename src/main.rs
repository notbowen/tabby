use std::error::Error;

use crate::board::Board;

mod board;
mod errors;
mod pieces;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    // b.show_bitboards();
    b.debug_board();

    Ok(())
}
