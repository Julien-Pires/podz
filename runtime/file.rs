use std::path::PathBuf;

pub trait Reader<T, E> {
    fn path(&self) -> PathBuf;

    fn empty(&self) -> T;
    fn read(&self, content: String) -> Result<T, E>;
}
