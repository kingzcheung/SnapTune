use std::{fmt::Display};

use oxipng::PngError;


#[derive(Debug)]
pub enum OptimizeError {
    PngError(PngError),
    IOError(std::io::Error),
    TurboJpegError(turbojpeg::Error),
    ScanLinesError,
}

impl Display for OptimizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OptimizeError::PngError(pe) => write!(f,"png error: {}",pe),
            OptimizeError::IOError(e) => write!(f,"io error: {}",e),
            OptimizeError::TurboJpegError(e) => write!(f,"turbojpeg error: {}", e),
            OptimizeError::ScanLinesError => write!(f,"mozjpeg scan lines error"),
        }
    }
}
impl From<turbojpeg::Error> for OptimizeError{
    fn from(e: turbojpeg::Error) -> Self {
        Self::TurboJpegError(e)
    }
}

impl From<PngError> for OptimizeError {
    fn from(pe: PngError) -> Self {
        Self::PngError(pe)
    }
}

impl From<std::io::Error> for OptimizeError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}