use std::error::Error;

use crate::state::GameState;

mod errors;
mod moves;
mod pieces;
mod state;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    // b.show_bitboards();
    b.debug_board();

    Ok(())
}
