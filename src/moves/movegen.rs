use crate::{
    moves::{Move, lookup::KNIGHT_ATTACKS},
    pieces::Piece,
    state::{GameState, bitboard::Bitboard, square::Square},
};

pub struct MoveGen;

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen
    }

    /// Generates a vector of legal moves given a game state
    pub fn generate_moves(&self, state: &mut GameState) -> Vec<Move> {
        let mut moves = Vec::with_capacity(256);

        self.generate_knights(state, &mut moves);

        moves
    }

    pub fn generate_knights(&self, state: &mut GameState, moves: &mut Vec<Move>) {
        let mut our_knights = state.color_bb[state.current_side] & state.piece_bb[Piece::Knight];
        let not_us = !state.color_bb[state.current_side];

        while let Some(p) = our_knights.pop_bit() {
            let knight_moves = KNIGHT_ATTACKS[p as usize];
            let mut final_moves = not_us & knight_moves;

            while let Some(t) = final_moves.pop_bit() {
                moves.push(Move {
                    from: Square::from_index(p).unwrap(),
                    to: Square::from_index(t).unwrap(),
                    flags: if state.color_bb[!state.current_side].get_bit(t) {
                        crate::moves::MoveType::Capture
                    } else {
                        crate::moves::MoveType::Quiet
                    },
                })
            }
        }

        println!("{:#?}", moves);
    }
}
