
use std::{fmt::Display, path::Path};

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
}


impl From<String> for Format {
    fn from(item: String) -> Self {
        match item {
            item if item =="PNG"=> Format::PNG,
            item if item =="JPEG" || item == "JPG"=> Format::JPEG,
            item if item =="GIF"=> Format::GIF,
            item if item =="BMP"=> Format::BMP,
            item if item =="ICO"=> Format::ICO,
            item if item =="TIFF"=> Format::TIFF,
            item if item =="WebP"=> Format::WebP,
            item if item =="AVIF"=> Format::AVIF,
            item if item =="PNM"=> Format::PNM,
            _=>Format::JPEG
        }
    }
}



impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::PNG => write!(f, "png"),
            Format::JPEG => write!(f, "jpg"),
            Format::GIF => write!(f, "gif"),
            Format::BMP => write!(f, "bmp"),
            Format::ICO => write!(f, "ico"),
            Format::TIFF => write!(f, "tiff"),
            Format::WebP => write!(f, "webp"),
            Format::AVIF => write!(f, "avif"),
            Format::PNM => write!(f, "pnm"),
        }
    }
}

pub async fn image2x(x: Format, source: String) -> Result<String, anyhow::Error> {
    let path = Path::new(source.as_str());
    let path = path.with_extension(x.to_string().as_str());

    let img = image::open(source.as_str())?;

    let err = anyhow::format_err!("格式错误");
    let image_format = ImageFormat::from_extension(x.to_string().as_str()).ok_or(err);

    img.save_with_format(path, image_format?)?;
    println!("转换成功");
    Ok(source)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[tokio::test]
    async fn test_image2x(){
        
        let source = "testdata/test.jpg"; 
        let result_path = std::path::Path::new(source).with_extension("png");
        let res = image2x(Format::PNG, source.into()).await;
        
        assert!(res.is_ok());
        assert!(std::fs::metadata(result_path).is_ok())
    }
}