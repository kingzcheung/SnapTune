pub mod jpeg;
pub mod png;

// 量化压缩

pub trait Compression {
    fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, anyhow::Error>;
}
