use std::str::FromStr;

use crate::{
    errors::FenParseError,
    pieces::{Color, str_to_colored_piece},
    state::{GameState, castling::CastlingRights, square::Square},
};

impl GameState {
    pub fn from_fen(fen: &str) -> Result<GameState, FenParseError> {
        let parts = fen.split(' ').collect::<Vec<&str>>();

        if parts.len() != 6 {
            return Err(FenParseError("FEN parts != 6".into()));
        }

        let mut board = GameState::new();
        parse_board_fen(&mut board, parts[0])?;
        parse_color(&mut board, parts[1])?;
        parse_castling_rights(&mut board, parts[2])?;
        parse_en_passant(&mut board, parts[3])?;
        parse_halfmove(&mut board, parts[4])?;
        parse_fullmove(&mut board, parts[5])?;

        Ok(board)
    }
}

fn parse_board_fen(board: &mut GameState, fen: &str) -> Result<(), FenParseError> {
    // [TODO]: Add integer overflow protection here
    let mut row = 7;
    let mut col = 0;

    for c in fen.chars() {
        match c {
            '1'..='8' => col += c as u8 - '1' as u8,
            'A'..='z' => {
                if let Some((color, piece)) = str_to_colored_piece(c) {
                    board.set_piece_index(color, piece, row * 8 + col);
                } else {
                    return Err(FenParseError(format!("Invalid character {}", c)));
                };
            }
            '/' => {
                row -= 1;
                col = 0;
                continue;
            }
            _ => return Err(FenParseError(format!("Invalid character {}", c))),
        }
        col += 1
    }

    Ok(())
}

fn parse_color(board: &mut GameState, color: &str) -> Result<(), FenParseError> {
    let c = match color {
        "w" => Color::White,
        "b" => Color::Black,
        _ => return Err(FenParseError(format!("Invalid color {}", color))),
    };

    board.current_side = c;
    Ok(())
}

fn parse_castling_rights(board: &mut GameState, castling: &str) -> Result<(), FenParseError> {
    if castling.len() > 4 {
        return Err(FenParseError("Castling rights is too long!".into()));
    }

    if castling == "-" {
        return Ok(());
    }

    for c in castling.chars() {
        match c {
            'k' => board.castling |= CastlingRights::BlackKing as u8,
            'q' => board.castling |= CastlingRights::BlackQueen as u8,
            'K' => board.castling |= CastlingRights::WhiteKing as u8,
            'Q' => board.castling |= CastlingRights::WhiteQueen as u8,
            _ => {
                return Err(FenParseError(format!(
                    "Invalid castling rights: {}",
                    castling
                )));
            }
        }
    }

    Ok(())
}

fn parse_en_passant(board: &mut GameState, square_str: &str) -> Result<(), FenParseError> {
    let square = if square_str != "-" {
        Some(
            Square::from_str(square_str)
                .map_err(|_| FenParseError(format!("Invalid en passant square: {}", square_str)))?,
        )
    } else {
        None
    };

    board.en_passant = square;
    Ok(())
}

fn parse_halfmove(board: &mut GameState, halfmove_str: &str) -> Result<(), FenParseError> {
    let halfmove = halfmove_str
        .parse::<u8>()
        .map_err(|_| FenParseError(format!("Invalid halfmove: {}", halfmove_str)))?;

    board.halfmove = halfmove;
    Ok(())
}

fn parse_fullmove(board: &mut GameState, fullmove_str: &str) -> Result<(), FenParseError> {
    let fullmove = fullmove_str
        .parse::<u8>()
        .map_err(|_| FenParseError(format!("Invalid fullmove: {}", fullmove_str)))?;

    board.fullmove = fullmove;
    Ok(())
}
