use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not};

#[derive(Debug, Clone, Copy)]
pub struct Bitboard(pub u64);

impl Bitboard {
    pub fn get_bit(&self, idx: u8) -> bool {
        (self.0 >> idx) & 1 == 1
    }

    pub fn set_bit(&mut self, idx: u8) {
        let setter = 1 << idx as u64;
        self.0 |= setter;
    }

    pub fn pop_bit(&mut self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }

        let n = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(n)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}
