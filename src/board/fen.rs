use crate::{board::Board, errors::FenParseError, pieces::str_to_colored_piece};

impl Board {
    pub fn from_fen(fen: &str) -> Result<Board, FenParseError> {
        let parts = fen.split(' ').collect::<Vec<&str>>();

        if parts.len() != 6 {
            return Err(FenParseError("FEN parts != 6".into()));
        }

        let mut board = Board::new();
        parse_board_fen(&mut board, parts[0])?;

        Ok(board)
    }
}

fn parse_board_fen(board: &mut Board, fen: &str) -> Result<(), FenParseError> {
    let mut idx = 64;
    for c in fen.chars() {
        idx -= 1;
        match c {
            '1'..='8' => idx -= c as u8 - '1' as u8, // [TODO]: Ensure this is accurate
            'A'..='z' => {
                if let Some((color, piece)) = str_to_colored_piece(c) {
                    board.set_piece_index(color, piece, idx);
                } else {
                    return Err(FenParseError(format!("Invalid character {}", c)));
                };
            }
            '/' => idx += 1,
            _ => return Err(FenParseError(format!("Invalid character {}", c))),
        }
    }

    Ok(())
}
