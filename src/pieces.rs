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

pub fn piece_to_str(color: Color, piece: Piece) -> String {
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
