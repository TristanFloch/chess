pub trait BitsOperations {
    fn msb_index(&self) -> usize;
    fn toggle_bit(&self, index: usize) -> Self;
}

impl BitsOperations for u64 {
    fn msb_index(&self) -> usize {
        (Self::BITS - self.leading_zeros() - 1) as usize
    }

    fn toggle_bit(&self, index: usize) -> Self {
        self ^ (1 << index)
    }
}
