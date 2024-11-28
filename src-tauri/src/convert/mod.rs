use image::{ImageFormat, ImageReader};
use std::io::Cursor;

pub enum ConvertFormat {
    Jpeg,
    Png,
    Webp,
}

impl ConvertFormat {
    fn get_ext(&self) -> &'static str {
        match self {
            ConvertFormat::Jpeg => "jpg",
            ConvertFormat::Png => "png",
            ConvertFormat::Webp => "webp",
        }
    }
}

impl TryFrom<String> for ConvertFormat {
    type Error = crate::error::AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "jpeg" | "jpg" => Ok(ConvertFormat::Jpeg),
            "png" => Ok(ConvertFormat::Png),
            "webp" => Ok(ConvertFormat::Webp),
            _ => Err(crate::error::AppError::InvalidFormat),
        }
    }
}
impl TryFrom<&str> for ConvertFormat {
    type Error = crate::error::AppError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "jpeg" | "jpg" => Ok(ConvertFormat::Jpeg),
            "png" => Ok(ConvertFormat::Png),
            "webp" => Ok(ConvertFormat::Webp),
            _ => Err(crate::error::AppError::InvalidFormat),
        }
    }
}

impl From<&ConvertFormat> for ImageFormat {
    fn from(value: &ConvertFormat) -> Self {
        match value {
            ConvertFormat::Jpeg => ImageFormat::Jpeg,
            ConvertFormat::Png => ImageFormat::Png,
            ConvertFormat::Webp => ImageFormat::WebP,
        }
    }
}
impl From<ConvertFormat> for ImageFormat {
    fn from(value: ConvertFormat) -> Self {
        match value {
            ConvertFormat::Jpeg => ImageFormat::Jpeg,
            ConvertFormat::Png => ImageFormat::Png,
            ConvertFormat::Webp => ImageFormat::WebP,
        }
    }
}

pub trait Converter {
    fn convert(&self, data: &[u8], to_format: ConvertFormat) -> Result<Vec<u8>, anyhow::Error>;
}

impl Converter for ConvertFormat {
    fn convert(&self, data: &[u8], to_format: ConvertFormat) -> Result<Vec<u8>, anyhow::Error> {
        let from_format: ImageFormat = self.into();
        let img = ImageReader::with_format(Cursor::new(data), from_format).decode()?;
        // let img = ImageReader::new(Cursor::new(data)).decode()?;
        let img = img.to_rgb8();
        let mut buf = std::vec![];
        img.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
        Ok(buf)
    }
}
