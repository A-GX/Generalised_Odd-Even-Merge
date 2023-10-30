pub trait CompareSwap {
    fn compare_swap(&mut self, a: usize, b: usize) -> std::io::Result<()>;
}