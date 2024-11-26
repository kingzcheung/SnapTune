use super::Compression;



pub struct Jpeg;

impl Compression for Jpeg {
    fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, anyhow::Error> {

        let image: image::RgbImage = turbojpeg::decompress_image(data)?;

        let jpeg_data = turbojpeg::compress_image(&image, quality.into(), turbojpeg::Subsamp::Sub2x2)?;

        Ok(jpeg_data.to_vec())
    }
}