use std::path::PathBuf;

pub trait Reader<T, E> {
    fn path(&self) -> PathBuf;
    fn read(&self, content: String) -> Result<T, E>;
}
