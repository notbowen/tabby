use strum_macros::EnumIter;

pub type Bitboard = u64;

#[derive(Debug, EnumIter)]
pub enum Color {
    White = 0,
    Black = 1,
}

#[derive(Clone, Copy, Debug, EnumIter)]
pub enum Piece {
    Pawn = 0,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub fn str_to_piece(s: char) -> Option<Piece> {
    match s.to_ascii_lowercase() {
        'p' => Some(Piece::Pawn),
        'n' => Some(Piece::Knight),
        'b' => Some(Piece::Bishop),
        'r' => Some(Piece::Rook),
        'q' => Some(Piece::Queen),
        'k' => Some(Piece::King),
        _ => None,
    }
}

pub fn str_to_colored_piece(s: char) -> Option<(Color, Piece)> {
    let piece = match str_to_piece(s) {
        Some(p) => p,
        None => return None,
    };

    let color = if s.is_uppercase() {
        Color::White
    } else {
        Color::Black
    };

    Some((color, piece))
}

pub fn piece_to_art(color: Color, piece: Piece) -> String {
    match (color, piece) {
        (Color::White, Piece::Pawn) => "󰡙",
        (Color::White, Piece::Knight) => "󰡘",
        (Color::White, Piece::Bishop) => "󰡜",
        (Color::White, Piece::Rook) => "󰡛",
        (Color::White, Piece::Queen) => "󰡚",
        (Color::White, Piece::King) => "󰡗",

        (Color::Black, Piece::Pawn) => "",
        (Color::Black, Piece::Knight) => "",
        (Color::Black, Piece::Bishop) => "",
        (Color::Black, Piece::Rook) => "",
        (Color::Black, Piece::Queen) => "",
        (Color::Black, Piece::King) => "",
    }
    .into()
}
