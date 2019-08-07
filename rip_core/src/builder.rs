

pub trait Builder<T, E> {
    fn build(&self) -> Result<T, E>;
}
