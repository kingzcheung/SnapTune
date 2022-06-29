use std::{fmt::{Display}, path::Path, };

use image::ImageFormat;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Format {
    PNG,
    JPEG,
    GIF,
    BMP,
    ICO,
    TIFF,
    WebP,
    AVIF,
    PNM,
    DDS,
    TGA,
    OpenEXR,
    Farbfeld
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::PNG => write!(f,"png"),
            Format::JPEG => write!(f,"jpg"),
            Format::GIF => write!(f,"gif"),
            Format::BMP => write!(f,"bmp"),
            Format::ICO => write!(f,"ico"),
            Format::TIFF => write!(f,"tiff"),
            Format::WebP => write!(f,"webp"),
            Format::AVIF => write!(f,"avif"),
            Format::PNM => write!(f,"pnm"),
            Format::TGA => write!(f,"gta"),
            Format::OpenEXR => write!(f,"exr"),
            Format::Farbfeld => write!(f,"ff"),
            Format::DDS => write!(f,"dds"),
        }
    }
}


pub async fn image2x(x: Format,source: String)-> Result<String,anyhow::Error>{
    let path = Path::new(source.as_str());
    let path = path.with_extension(x.to_string().as_str());

    let img = image::open(source.as_str())?;

    let err = anyhow::format_err!("格式错误");
    let image_format = ImageFormat::from_extension(x.to_string().as_str()).ok_or(err);
    
    img.save_with_format(path, image_format?)?;
    Ok(source)
}