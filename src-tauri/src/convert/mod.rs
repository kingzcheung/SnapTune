use image::{ImageFormat, ImageReader};
use std::io::Cursor;

pub enum ConvertFormat {
    Jpeg,
    Png,
    Webp,
    Avif,
    Bmp,
    Gif,
    Tiff,
    Hdr,
    OpenExr,
}

impl ConvertFormat {
    fn get_ext(&self) -> &'static str {
        match self {
            ConvertFormat::Jpeg => "jpg",
            ConvertFormat::Png => "png",
            ConvertFormat::Webp => "webp",
            ConvertFormat::Avif => "avif",
            ConvertFormat::Bmp => "bmp",
            ConvertFormat::Gif => "gif",
            ConvertFormat::Tiff => "tiff",
            ConvertFormat::Hdr => "hdr",
            ConvertFormat::OpenExr => "exr",
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
            "avif" => Ok(ConvertFormat::Avif),
            "bmp" => Ok(ConvertFormat::Bmp),
            "gif" => Ok(ConvertFormat::Gif),
            "tiff" => Ok(ConvertFormat::Tiff),
            "exr" => Ok(ConvertFormat::OpenExr),
            "hdr" => Ok(ConvertFormat::Hdr),
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
            "avif" => Ok(ConvertFormat::Avif),
            "bmp" => Ok(ConvertFormat::Bmp),
            "gif" => Ok(ConvertFormat::Gif),
            "tiff" => Ok(ConvertFormat::Tiff),
            "exr" => Ok(ConvertFormat::OpenExr),
            "hdr" => Ok(ConvertFormat::Hdr),
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
            ConvertFormat::Avif => ImageFormat::Avif,
            ConvertFormat::Bmp => ImageFormat::Bmp,
            ConvertFormat::Gif => ImageFormat::Gif,
            ConvertFormat::Tiff => ImageFormat::Tiff,
            ConvertFormat::OpenExr => ImageFormat::OpenExr,
            ConvertFormat::Hdr => ImageFormat::Hdr,
        }
    }
}
impl From<ConvertFormat> for ImageFormat {
    fn from(value: ConvertFormat) -> Self {
        match value {
            ConvertFormat::Jpeg => ImageFormat::Jpeg,
            ConvertFormat::Png => ImageFormat::Png,
            ConvertFormat::Webp => ImageFormat::WebP,
            ConvertFormat::Avif => ImageFormat::Avif,
            ConvertFormat::Bmp => ImageFormat::Bmp,
            ConvertFormat::Gif => ImageFormat::Gif,
            ConvertFormat::Tiff => ImageFormat::Tiff,
            ConvertFormat::OpenExr => ImageFormat::OpenExr,
            ConvertFormat::Hdr => ImageFormat::Hdr,
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
        let mut buf = std::vec![];
        match to_format {
            ConvertFormat::Hdr =>{
                let img =  img.to_rgb32f();
                img.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
                Ok(buf)
            }
            ConvertFormat::OpenExr =>{
                let img = img.to_rgb32f();
                img.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
                Ok(buf)
            }
            _=>{
                let img = img.to_rgb8();
                img.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
                Ok(buf)
            }
        }
    }
}
