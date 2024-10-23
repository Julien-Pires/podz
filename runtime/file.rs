pub trait Reader<T, E> {
    fn read(&self, content: String) -> Result<T, E>;
}

pub trait Builder<T> {
    fn new(&self) -> T;
}
