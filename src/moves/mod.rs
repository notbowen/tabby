#![allow(unused)]

use crate::state::square::Square;

pub mod movegen;
pub mod lookup;

#[derive(Debug)]
pub struct Move {
    from: Square,
    to: Square,
    flags: MoveType,
}

#[derive(Debug)]
pub enum MoveType {
    Quiet,
    Capture,
    EnPassant,
    QueenPromotion,
    RookPromotion,
    BishopPromotion,
    KnightPromotion,
}
