#![allow(unused)]

use strum_macros::EnumIter;

use crate::state::square::Square;

pub mod lookup;
pub mod movegen;

#[derive(Debug)]
pub struct Move {
    from: Square,
    to: Square,
    flags: MoveType,
}

#[derive(Clone, Debug, EnumIter)]
pub enum MoveType {
    Quiet,
    Capture,
    EnPassant,
    QueenPromotion,
    RookPromotion,
    BishopPromotion,
    KnightPromotion,
    QueenPromotionCapture,
    RookPromotionCapture,
    BishopPromotionCapture,
    KnightPromotionCapture,
}
