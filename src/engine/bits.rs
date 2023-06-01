pub trait BitOperations {
    fn lsb_index(&self) -> usize;
    fn lsb_pop(&mut self) -> usize;
    fn msb_index(&self) -> usize;
    fn toggle_bit(&mut self, index: usize);
}

impl BitOperations for u64 {
    fn lsb_index(&self) -> usize {
        self.trailing_zeros() as usize
    }

    fn lsb_pop(&mut self) -> usize {
        let index = self.trailing_zeros() as usize;
        *self &= !(1u64 << index);
        index
    }

    fn msb_index(&self) -> usize {
        (Self::BITS - self.leading_zeros() - 1) as usize
    }

    fn toggle_bit(&mut self, index: usize) {
        *self = *self ^ (1 << index);
    }
}
