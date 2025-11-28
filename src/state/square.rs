#![allow(dead_code)]

use std::fmt;
use std::str::FromStr;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Square {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
}

#[rustfmt::skip]
impl Square {
    pub const fn to_index(self) -> u8 {
        match self {
            Square::A1 => 0,  Square::B1 => 1,  Square::C1 => 2,  Square::D1 => 3,
            Square::E1 => 4,  Square::F1 => 5,  Square::G1 => 6,  Square::H1 => 7,
            Square::A2 => 8,  Square::B2 => 9,  Square::C2 => 10, Square::D2 => 11,
            Square::E2 => 12, Square::F2 => 13, Square::G2 => 14, Square::H2 => 15,
            Square::A3 => 16, Square::B3 => 17, Square::C3 => 18, Square::D3 => 19,
            Square::E3 => 20, Square::F3 => 21, Square::G3 => 22, Square::H3 => 23,
            Square::A4 => 24, Square::B4 => 25, Square::C4 => 26, Square::D4 => 27,
            Square::E4 => 28, Square::F4 => 29, Square::G4 => 30, Square::H4 => 31,
            Square::A5 => 32, Square::B5 => 33, Square::C5 => 34, Square::D5 => 35,
            Square::E5 => 36, Square::F5 => 37, Square::G5 => 38, Square::H5 => 39,
            Square::A6 => 40, Square::B6 => 41, Square::C6 => 42, Square::D6 => 43,
            Square::E6 => 44, Square::F6 => 45, Square::G6 => 46, Square::H6 => 47,
            Square::A7 => 48, Square::B7 => 49, Square::C7 => 50, Square::D7 => 51,
            Square::E7 => 52, Square::F7 => 53, Square::G7 => 54, Square::H7 => 55,
            Square::A8 => 56, Square::B8 => 57, Square::C8 => 58, Square::D8 => 59,
            Square::E8 => 60, Square::F8 => 61, Square::G8 => 62, Square::H8 => 63,
        }
    }

    pub const fn from_index(i: u8) -> Option<Square> {
        match i {
            0 => Some(Square::A1), 1 => Some(Square::B1), 2 => Some(Square::C1), 3 => Some(Square::D1),
            4 => Some(Square::E1), 5 => Some(Square::F1), 6 => Some(Square::G1), 7 => Some(Square::H1),
            8 => Some(Square::A2), 9 => Some(Square::B2), 10 => Some(Square::C2), 11 => Some(Square::D2),
            12 => Some(Square::E2),13 => Some(Square::F2),14 => Some(Square::G2),15 => Some(Square::H2),
            16 => Some(Square::A3),17 => Some(Square::B3),18 => Some(Square::C3),19 => Some(Square::D3),
            20 => Some(Square::E3),21 => Some(Square::F3),22 => Some(Square::G3),23 => Some(Square::H3),
            24 => Some(Square::A4),25 => Some(Square::B4),26 => Some(Square::C4),27 => Some(Square::D4),
            28 => Some(Square::E4),29 => Some(Square::F4),30 => Some(Square::G4),31 => Some(Square::H4),
            32 => Some(Square::A5),33 => Some(Square::B5),34 => Some(Square::C5),35 => Some(Square::D5),
            36 => Some(Square::E5),37 => Some(Square::F5),38 => Some(Square::G5),39 => Some(Square::H5),
            40 => Some(Square::A6),41 => Some(Square::B6),42 => Some(Square::C6),43 => Some(Square::D6),
            44 => Some(Square::E6),45 => Some(Square::F6),46 => Some(Square::G6),47 => Some(Square::H6),
            48 => Some(Square::A7),49 => Some(Square::B7),50 => Some(Square::C7),51 => Some(Square::D7),
            52 => Some(Square::E7),53 => Some(Square::F7),54 => Some(Square::G7),55 => Some(Square::H7),
            56 => Some(Square::A8),57 => Some(Square::B8),58 => Some(Square::C8),59 => Some(Square::D8),
            60 => Some(Square::E8),61 => Some(Square::F8),62 => Some(Square::G8),63 => Some(Square::H8),
            _ => None,
        }
    }
}

#[rustfmt::skip]
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Square::A1 => "a1", Square::B1 => "b1", Square::C1 => "c1", Square::D1 => "d1",
            Square::E1 => "e1", Square::F1 => "f1", Square::G1 => "g1", Square::H1 => "h1",
            Square::A2 => "a2", Square::B2 => "b2", Square::C2 => "c2", Square::D2 => "d2",
            Square::E2 => "e2", Square::F2 => "f2", Square::G2 => "g2", Square::H2 => "h2",
            Square::A3 => "a3", Square::B3 => "b3", Square::C3 => "c3", Square::D3 => "d3",
            Square::E3 => "e3", Square::F3 => "f3", Square::G3 => "g3", Square::H3 => "h3",
            Square::A4 => "a4", Square::B4 => "b4", Square::C4 => "c4", Square::D4 => "d4",
            Square::E4 => "e4", Square::F4 => "f4", Square::G4 => "g4", Square::H4 => "h4",
            Square::A5 => "a5", Square::B5 => "b5", Square::C5 => "c5", Square::D5 => "d5",
            Square::E5 => "e5", Square::F5 => "f5", Square::G5 => "g5", Square::H5 => "h5",
            Square::A6 => "a6", Square::B6 => "b6", Square::C6 => "c6", Square::D6 => "d6",
            Square::E6 => "e6", Square::F6 => "f6", Square::G6 => "g6", Square::H6 => "h6",
            Square::A7 => "a7", Square::B7 => "b7", Square::C7 => "c7", Square::D7 => "d7",
            Square::E7 => "e7", Square::F7 => "f7", Square::G7 => "g7", Square::H7 => "h7",
            Square::A8 => "a8", Square::B8 => "b8", Square::C8 => "c8", Square::D8 => "d8",
            Square::E8 => "e8", Square::F8 => "f8", Square::G8 => "g8", Square::H8 => "h8",
        };
        write!(f, "{}", s)
    }
}

#[rustfmt::skip]
impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "a1" => Ok(Square::A1), "b1" => Ok(Square::B1), "c1" => Ok(Square::C1), "d1" => Ok(Square::D1),
            "e1" => Ok(Square::E1), "f1" => Ok(Square::F1), "g1" => Ok(Square::G1), "h1" => Ok(Square::H1),
            "a2" => Ok(Square::A2), "b2" => Ok(Square::B2), "c2" => Ok(Square::C2), "d2" => Ok(Square::D2),
            "e2" => Ok(Square::E2), "f2" => Ok(Square::F2), "g2" => Ok(Square::G2), "h2" => Ok(Square::H2),
            "a3" => Ok(Square::A3), "b3" => Ok(Square::B3), "c3" => Ok(Square::C3), "d3" => Ok(Square::D3),
            "e3" => Ok(Square::E3), "f3" => Ok(Square::F3), "g3" => Ok(Square::G3), "h3" => Ok(Square::H3),
            "a4" => Ok(Square::A4), "b4" => Ok(Square::B4), "c4" => Ok(Square::C4), "d4" => Ok(Square::D4),
            "e4" => Ok(Square::E4), "f4" => Ok(Square::F4), "g4" => Ok(Square::G4), "h4" => Ok(Square::H4),
            "a5" => Ok(Square::A5), "b5" => Ok(Square::B5), "c5" => Ok(Square::C5), "d5" => Ok(Square::D5),
            "e5" => Ok(Square::E5), "f5" => Ok(Square::F5), "g5" => Ok(Square::G5), "h5" => Ok(Square::H5),
            "a6" => Ok(Square::A6), "b6" => Ok(Square::B6), "c6" => Ok(Square::C6), "d6" => Ok(Square::D6),
            "e6" => Ok(Square::E6), "f6" => Ok(Square::F6), "g6" => Ok(Square::G6), "h6" => Ok(Square::H6),
            "a7" => Ok(Square::A7), "b7" => Ok(Square::B7), "c7" => Ok(Square::C7), "d7" => Ok(Square::D7),
            "e7" => Ok(Square::E7), "f7" => Ok(Square::F7), "g7" => Ok(Square::G7), "h7" => Ok(Square::H7),
            "a8" => Ok(Square::A8), "b8" => Ok(Square::B8), "c8" => Ok(Square::C8), "d8" => Ok(Square::D8),
            "e8" => Ok(Square::E8), "f8" => Ok(Square::F8), "g8" => Ok(Square::G8), "h8" => Ok(Square::H8),
            _ => Err(())
        }
    }
}
