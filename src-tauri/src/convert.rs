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
use image::{EncodableLayout, ImageFormat};
use libheif_rs::{HeifContext, ItemId};
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
            item if item == "HEIC" => Format::HEIF,
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
            Format::HEIF => write!(f, "heic"),
        }
    }
}

pub async fn image2x(x: Format, source: String) -> Result<String, anyhow::Error> {
    let path = Path::new(source.as_str());
    let path = path.with_extension(x.to_string().as_str());
    let err = anyhow::format_err!("格式错误");
    let image_format = ImageFormat::from_extension(x.to_string().as_str()).ok_or(err);

    match x {
        Format::HEIF => {
            heif2x(path, image_format?, source.as_str()).await?;
            Ok(source)
        }
        _ => {
            let img = image::open(source.as_str())?;
            img.save_with_format(path, image_format?)?;
            println!("转换成功");
            Ok(source)
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
    let exif: Vec<u8> = handle.metadata(meta_ids[0])?;

    let img = image::load_from_memory(exif.as_bytes())?;
    img.save_with_format(path, image_format)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_image2x() {
        let source = "testdata/test.jpg";
        let result_path = std::path::Path::new(source).with_extension("png");
        let res = image2x(Format::PNG, source.into()).await;

        assert!(res.is_ok());
        assert!(std::fs::metadata(result_path).is_ok())
    }
}
