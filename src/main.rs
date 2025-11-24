use clap::Parser;
use std::error::Error;

use crate::state::GameState;

mod errors;
mod moves;
mod pieces;
mod state;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run Perft with depth n
    #[arg(short, long)]
    perft: Option<u8>,
}

fn run_engine() -> Result<(), Box<dyn Error>> {
    // [TODO]: Run a UCI compatible engine from here
    let mut b = GameState::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")?;
    b.debug_board();
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(_n) = args.perft {
        todo!("Implement perft with depth n");
    } else {
        run_engine()?;
    }

    Ok(())
}
