use super::Compression;


pub struct Png;

impl Compression for Png {
    fn compress(data: &[u8], quality: u8) -> Result<Vec<u8>, anyhow::Error> {
        quant_png(data, quality, 4)
    }
}



pub fn quant_png(data: &[u8], quality: u8, speed: i32) -> Result<Vec<u8>, anyhow::Error> {
    let bitmap = lodepng::decode32(data).map_err(anyhow::Error::from)?;
    let width = bitmap.width;
    let height = bitmap.height;

    let mut liq = imagequant::new();
    liq.set_speed(speed).map_err(anyhow::Error::from)?;
    liq.set_quality(20, quality).map_err(anyhow::Error::from)?;

    let mut img = liq
        .new_image(bitmap.buffer, width, height, 0.0)
        .map_err(anyhow::Error::from)?;

    let mut res = liq.quantize(&mut img).map_err(anyhow::Error::from)?;

    // Enable dithering for subsequent remappings
    res.set_dithering_level(1.0).map_err(anyhow::Error::from)?;

    let (palette, pixels) = res.remapped(&mut img).map_err(anyhow::Error::from)?;

    let mut enc = lodepng::Encoder::new();
    enc.set_palette(&palette).map_err(anyhow::Error::from)?;

    let r = enc.encode(&pixels, width, height).map_err(anyhow::Error::from)?;
    Ok(r)
}