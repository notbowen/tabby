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
}
