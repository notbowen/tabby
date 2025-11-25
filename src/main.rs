use clap::Parser;
use std::error::Error;

use crate::{moves::movegen::MoveGen, state::GameState};

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

    /// FEN of starting position (defaults to normal)
    #[arg(short, long)]
    fen: Option<String>,
}

fn run_engine(fen: &str) -> Result<(), Box<dyn Error>> {
    // [TODO]: Run a UCI compatible engine from here
    let mut b = GameState::from_fen(fen)?;
    b.print_board();
    Ok(())
}

fn perft(fen: &str, _n: u8) -> Result<(), Box<dyn Error>> {
    let mut state = GameState::from_fen(fen)?;
    let movegen = MoveGen::new();

    movegen.generate_moves(&mut state);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let fen = if let Some(f) = args.fen {
        f
    } else {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".into()
    };

    if let Some(n) = args.perft {
        perft(&fen, n)?;
    } else {
        run_engine(&fen)?;
    }

    Ok(())
}
