pub trait BitsOperations {
    fn lsb_index(&self) -> usize;
    fn msb_index(&self) -> usize;
    fn toggle_bit(&self, index: usize) -> Self;
}

impl BitsOperations for u64 {
    fn lsb_index(&self) -> usize {
        self.trailing_zeros() as usize
    }

    fn msb_index(&self) -> usize {
        (Self::BITS - self.leading_zeros() - 1) as usize
    }

    fn toggle_bit(&self, index: usize) -> Self {
        self ^ (1 << index)
    }
}
