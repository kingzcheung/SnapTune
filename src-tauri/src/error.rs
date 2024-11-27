#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid image")]
    InvalidImage,
    #[error("not implemented")]
    NotImplemented,
    #[error("{0}")]
    Any(anyhow::Error),
    #[error("file not found")]
    FileNotFound(#[from] std::io::Error),
    #[error("invalid format")]
    InvalidFormat,
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}