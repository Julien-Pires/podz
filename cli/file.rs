pub trait FileReader<T, E> {
    fn read(&self, content: Vec<u8>) -> Result<T, E>;
}

pub trait Builder<T> {
    fn new(&self) -> T;
}
