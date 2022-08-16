// Copyright 2022 kingzcheung
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::Ok;
use image::{ImageFormat, RgbImage, ImageBuffer};
use libheif_rs::{HeifContext, ItemId, ColorSpace, RgbChroma};
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
    HEIF,
    HEIC,
}

impl From<String> for Format {
    fn from(item: String) -> Self {
        match item {
            item if item == "PNG" => Format::PNG,
            item if item == "JPEG" || item == "JPG" => Format::JPEG,
            item if item == "GIF" => Format::GIF,
            item if item == "BMP" => Format::BMP,
            item if item == "ICO" => Format::ICO,
            item if item == "TIFF" => Format::TIFF,
            item if item == "WebP" => Format::WebP,
            item if item == "AVIF" => Format::AVIF,
            item if item == "PNM" => Format::PNM,
            item if item == "HEIF" => Format::HEIF,
            item if item == "HEIC" => Format::HEIC,
            _ => Format::JPEG,
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
            Format::HEIC => write!(f, "heic"),
            Format::HEIF => write!(f, "heic"),
        }
    }
}

pub async fn image2x(output_format: Format, source: &str,source_format: Format) -> Result<(), anyhow::Error> {
    let path = Path::new(source);
    let path = path.with_extension(output_format.to_string().as_str());
    let err = anyhow::format_err!("格式错误");
    let image_format = ImageFormat::from_extension(output_format.to_string().as_str()).ok_or(err);
    
    match source_format {
        Format::HEIC => {
            dbg!(source);
            heif2x(path, image_format?, source).await?;
            
            Ok(())
        }
        _ => {
            let img = image::open(source)?;
            img.save_with_format(path, image_format?)?;
            println!("转换成功");
            Ok(())
        }
    }
}

pub async fn heif2x(
    path: PathBuf,
    image_format: ImageFormat,
    source: &str,
) -> Result<(), anyhow::Error> {
    let ctx = HeifContext::read_from_file(source)?;
    let handle = ctx.primary_image_handle()?;
    let mut meta_ids: Vec<ItemId> = vec![0; 1];
    handle.metadata_block_ids("Exif", &mut meta_ids);

    let width = handle.width();
    let height = handle.height();

    let img = handle.decode(ColorSpace::Rgb(RgbChroma::C444), false).unwrap();

    let planes = img.planes();
    let plane_r = planes.r.unwrap();
    let data_r = plane_r.data;
    let data_g = planes.g.unwrap().data;
    let data_b = planes.b.unwrap().data;
    let stride = plane_r.stride;

    let mut newimg: RgbImage = ImageBuffer::new(width, height);
        for y in 0..height {
            let mut row_start = stride * y as usize;
            for x in 0..width {
                
                newimg.put_pixel(x, y, image::Rgb([data_r[row_start],data_g[row_start],data_b[row_start]]));
                row_start += 1;
            }
        }
    
    newimg.save_with_format(path, image_format)?;
    Ok(())
}

#[cfg(test)]
mod test {
    // use image::{ImageBuffer, RgbImage, GenericImage};
    // use libheif_rs::{ColorSpace, RgbChroma, Channel, Chroma};

    use super::*;

    #[tokio::test]
    async fn test_image2x() {
        let source = "testdata/test.jpg";
        let result_path = std::path::Path::new(source).with_extension("png");
        let res = image2x(Format::PNG, source, Format::JPEG).await;

        assert!(res.is_ok());
        assert!(std::fs::metadata(result_path).is_ok())
    }

    #[tokio::test]
    async fn test_heif2x(){
        let  source = "./testdata/IMG_3627.HEIC";
        let path = PathBuf::from("./testdata/IMG_3627.jpeg");
        let image_format = ImageFormat::Jpeg;
        
        let res = heif2x(path, image_format, source).await;
        assert!(res.is_ok());
        // let ctx = HeifContext::read_from_file(source).unwrap();
        // let handle = ctx.primary_image_handle().unwrap();

        // // let mut meta_ids: Vec<ItemId> = vec![0; 1];
        // // let _count = handle.metadata_block_ids("Exif", &mut meta_ids);
        // // let exif: Vec<u8> = handle.metadata(meta_ids[0]).unwrap();
        // let img = handle.decode(ColorSpace::Rgb(RgbChroma::C444), false).unwrap();

        // assert_eq!(img.color_space(), Some(ColorSpace::Rgb(RgbChroma::C444)));
        // let width = handle.width();
        // let height = handle.height();
        
        // let planes = img.planes();
        // let plane_r = planes.r.unwrap();
        // let data_r = plane_r.data;
        // let data_g = planes.g.unwrap().data;
        // let data_b = planes.b.unwrap().data;
        // let stride = plane_r.stride;

        // let mut newimg: RgbImage = ImageBuffer::new(width, height);
        // for y in 0..height {
        //     let mut row_start = stride * y as usize;
        //     for x in 0..width {
                
        //         newimg.put_pixel(x, y, image::Rgb([data_r[row_start],data_g[row_start],data_b[row_start]]));
        //         row_start += 1;
        //     }
        // }
        // // newimg.put_pixel(0, 0, pixel)
        // newimg.save(path);
    }

}
