pub trait BitsOperations {
    fn msb_index(&self) -> usize;
}

impl BitsOperations for u64 {
    fn msb_index(&self) -> usize {
        (Self::BITS - self.leading_zeros() - 1) as usize
    }
}
