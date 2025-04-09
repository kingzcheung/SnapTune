use std::io::Cursor;

use anyhow::Context;
use image::Rgb;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, Plane, Planes, RgbChroma};

use super::ConvertFormat;

pub fn heif_to_x(data: &[u8], to_format: ConvertFormat) -> Result<Vec<u8>, anyhow::Error> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_bytes(data)?;
    let handle = ctx.primary_image_handle()?;
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    // Get "pixels"
    let planes = image.planes();

    let img_buf = image::RgbImage::from_raw(
        image.width(),
        image.height(),
        planes.interleaved.unwrap().data.to_vec(),
    )
    .context("Invalid image data")?;


    let mut buf = std::vec![];
    match to_format {
        ConvertFormat::Hdr => {
            let img_buf = image::DynamicImage::ImageRgb8(img_buf);
            let img = img_buf.to_rgb32f();
            img.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
            Ok(buf)
        }
        ConvertFormat::OpenExr => {
            let img_buf = image::DynamicImage::ImageRgb8(img_buf);
            let img = img_buf.to_rgb32f();
            img.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
            Ok(buf)
        }
        _ => {
            img_buf.write_to(&mut Cursor::new(&mut buf), to_format.into())?;
            Ok(buf)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_heif_to_x() -> anyhow::Result<()> {
        let data = include_bytes!("../../testdata/IMG_0203.HEIC");
        let buf = heif_to_x(data, ConvertFormat::Jpeg)?;
        std::fs::write("./test.jpg", buf)?;
        // remove  test file

        Ok(())
    }
}
