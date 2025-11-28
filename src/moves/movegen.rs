use crate::{
    moves::{
        Move, MoveType,
        lookup::{KING_ATTACKS, KNIGHT_ATTACKS},
    },
    pieces::{Color, Piece},
    state::{GameState, bitboard::Bitboard, square::Square},
};

const FILE_A: Bitboard = Bitboard(0x0101010101010101);
const FILE_H: Bitboard = Bitboard(0x8080808080808080);

const RANK_1: Bitboard = Bitboard(0x00000000000000FF);
const RANK_3: Bitboard = Bitboard(0x0000000000FF0000);
const RANK_6: Bitboard = Bitboard(0x0000FF0000000000);
const RANK_8: Bitboard = Bitboard(0xFF00000000000000);

pub struct MoveGen;

impl MoveGen {
    pub fn new() -> MoveGen {
        MoveGen
    }

    /// Generates a vector of legal moves given a game state
    pub fn generate_moves(&self, state: &mut GameState) -> Vec<Move> {
        let mut moves = Vec::with_capacity(256);

        self.generate_knights(state, &mut moves);
        self.generate_kings(state, &mut moves);
        self.generate_pawn_moves(state, &mut moves);

        println!("{:#?}", &moves);

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
    }

    pub fn generate_kings(&self, state: &mut GameState, moves: &mut Vec<Move>) {
        let mut our_king = state.color_bb[state.current_side] & state.piece_bb[Piece::King];
        let legal_spaces = !state.color_bb[state.current_side];

        let pos = our_king.pop_bit().unwrap(); // should nvr fail coz every board has a king
        let attacks = KING_ATTACKS[pos as usize];

        let mut legal_moves = attacks & legal_spaces;
        while let Some(m) = legal_moves.pop_bit() {
            moves.push(Move {
                from: Square::from_index(pos).unwrap(),
                to: Square::from_index(m).unwrap(),
                flags: if state.color_bb[!state.current_side].get_bit(m) {
                    crate::moves::MoveType::Capture
                } else {
                    crate::moves::MoveType::Quiet
                },
            });
        }
    }

    pub fn generate_pawn_moves(&self, state: &mut GameState, moves: &mut Vec<Move>) {
        match state.current_side {
            Color::White => self.generate_white_pawn_moves(state, moves),
            Color::Black => self.generate_black_pawn_moves(state, moves),
        }
    }

    pub fn generate_white_pawn_moves(&self, state: &mut GameState, moves: &mut Vec<Move>) {
        let empty_squares = state.get_empty_squares();

        let our_pawns = state.color_bb[state.current_side] & state.piece_bb[Piece::Pawn];
        let pushed_pawns = (our_pawns << 8) & empty_squares;
        let double_pushes = ((pushed_pawns & RANK_3) << 8) & empty_squares;

        let enemy_pieces = state.color_bb[!state.current_side];

        let mut quiet_moves = (pushed_pawns & !RANK_8) | double_pushes;
        let mut captures_left = ((our_pawns & !FILE_A) << 7) & enemy_pieces;
        let mut captures_right = ((our_pawns & !FILE_H) << 9) & enemy_pieces;

        let mut promotion_moves = (pushed_pawns & RANK_8) & empty_squares;
        let mut promotion_left_captures = (captures_left & RANK_8) & state.piece_bb[Color::White];
        let mut promotion_right_captures = (captures_right & RANK_8) & state.piece_bb[Color::White];

        self.process_pawn_bitboards(
            moves,
            &mut promotion_moves,
            &mut [&mut promotion_left_captures, &mut promotion_right_captures],
            &mut quiet_moves,
            &mut [&mut captures_left, &mut captures_right],
            state.current_side,
        );

        if state.en_passant.is_none() {
            return;
        }

        // en passant calculation
        let en_passant_idx = state.en_passant.unwrap().to_index();
        let en_passant_board = Bitboard(1 << en_passant_idx) & enemy_pieces;

        let mut en_passant_left = captures_left & en_passant_board;
        let mut en_passant_right = captures_right & en_passant_board;

        if en_passant_left != 0 {
            let to = en_passant_left.pop_bit().unwrap();
            moves.push(Move {
                from: Square::from_index(to - 7).unwrap(),
                to: Square::from_index(to).unwrap(),
                flags: MoveType::EnPassant,
            });
        }

        if en_passant_right != 0 {
            let to = en_passant_right.pop_bit().unwrap();
            moves.push(Move {
                from: Square::from_index(to - 9).unwrap(),
                to: Square::from_index(to).unwrap(),
                flags: MoveType::EnPassant,
            });
        }
    }

    pub fn generate_black_pawn_moves(&self, state: &mut GameState, moves: &mut Vec<Move>) {
        let empty_squares = state.get_empty_squares();

        let our_pawns = state.color_bb[state.current_side] & state.piece_bb[Piece::Pawn];
        let pushed_pawns = (our_pawns >> 8) & empty_squares;
        let double_pushes = ((pushed_pawns & RANK_8) >> 8) & empty_squares;

        let enemy_pieces = state.color_bb[!state.current_side];

        let mut quiet_moves = (pushed_pawns & !RANK_1) | double_pushes;
        let mut captures_left = ((our_pawns & !FILE_A) >> 7) & enemy_pieces;
        let mut captures_right = ((our_pawns & !FILE_H) >> 9) & enemy_pieces;

        let mut promotion_moves = (pushed_pawns & RANK_1) & empty_squares;
        let mut promotion_left_captures = (captures_left & RANK_1) & state.piece_bb[Color::White];
        let mut promotion_right_captures = (captures_right & RANK_1) & state.piece_bb[Color::White];

        self.process_pawn_bitboards(
            moves,
            &mut promotion_moves,
            &mut [&mut promotion_left_captures, &mut promotion_right_captures],
            &mut quiet_moves,
            &mut [&mut captures_left, &mut captures_right],
            state.current_side,
        );

        if state.en_passant.is_none() {
            return;
        }

        // en passant calculation
        let en_passant_idx = state.en_passant.unwrap().to_index();
        let en_passant_board = Bitboard(1 << en_passant_idx) & enemy_pieces;

        let mut en_passant_left = captures_left & en_passant_board;
        let mut en_passant_right = captures_right & en_passant_board;

        if en_passant_left != 0 {
            let to = en_passant_left.pop_bit().unwrap();
            moves.push(Move {
                from: Square::from_index(to + 7).unwrap(),
                to: Square::from_index(to).unwrap(),
                flags: MoveType::EnPassant,
            });
        }

        if en_passant_right != 0 {
            let to = en_passant_right.pop_bit().unwrap();
            moves.push(Move {
                from: Square::from_index(to + 9).unwrap(),
                to: Square::from_index(to).unwrap(),
                flags: MoveType::EnPassant,
            });
        }
    }

    fn process_pawn_bitboards(
        &self,
        moves: &mut Vec<Move>,
        promotion_moves: &mut Bitboard,
        promotion_capture_moves: &mut [&mut Bitboard; 2],
        quiet_moves: &mut Bitboard,
        capture_moves: &mut [&mut Bitboard; 2],
        side: Color,
    ) {
        while let Some(to) = quiet_moves.pop_bit() {
            moves.push(Move {
                from: Square::from_index(match side {
                    Color::Black => to + 8,
                    Color::White => to - 8,
                })
                .unwrap(),
                to: Square::from_index(to).unwrap(),
                flags: MoveType::Quiet,
            });
        }

        capture_moves.iter_mut().enumerate().for_each(|(i, board)| {
            while let Some(to) = board.pop_bit() {
                moves.push(Move {
                    from: Square::from_index(match side {
                        Color::Black => to + (7 + (i * 2) as u8),
                        Color::White => to - (7 + (i * 2) as u8),
                    })
                    .unwrap(),
                    to: Square::from_index(to).unwrap(),
                    flags: MoveType::Capture,
                });
            }
        });

        while let Some(to) = promotion_moves.pop_bit() {
            [
                MoveType::RookPromotion,
                MoveType::QueenPromotion,
                MoveType::BishopPromotion,
                MoveType::KnightPromotion,
            ]
            .iter()
            .for_each(|f| {
                moves.push(Move {
                    from: Square::from_index(match side {
                        Color::Black => to + 8,
                        Color::White => to - 8,
                    })
                    .unwrap(),
                    to: Square::from_index(to).unwrap(),
                    flags: f.clone(),
                })
            });
        }

        promotion_capture_moves
            .iter_mut()
            .enumerate()
            .for_each(|(i, board)| {
                while let Some(to) = board.pop_bit() {
                    [
                        MoveType::RookPromotionCapture,
                        MoveType::QueenPromotionCapture,
                        MoveType::BishopPromotionCapture,
                        MoveType::KnightPromotionCapture,
                    ]
                    .iter()
                    .for_each(|f| {
                        moves.push(Move {
                            from: Square::from_index(match side {
                                Color::Black => to + (7 + (i * 2) as u8),
                                Color::White => to - (7 + (i * 2) as u8),
                            })
                            .unwrap(),
                            to: Square::from_index(to).unwrap(),
                            flags: f.clone(),
                        })
                    });
                }
            });
    }
}
