use image::ImageReader;

use crate::error::AppError;

pub async fn crop_image(
    image_path: String,
    crop_width: u32,
    crop_height: u32,
    crop_x: u32,
    crop_y: u32,
    save_path: String,
) -> Result<(), AppError> {
    let mut img = ImageReader::open(image_path)?
        .decode()
        .map_err(|_| AppError::ImageDecodeError)?;
    let img = img.crop(
        crop_x,
        crop_y,
        crop_width,
        crop_height,
    );

    let _ = img.save(save_path).map_err(|e| AppError::ImageSaveError);

    Ok(())
}

pub async fn resize_image(
    image_path: String,
    width: u32,
    height: u32,
    save_path: String,
) -> Result<(), AppError> {
    let img = ImageReader::open(image_path)?
        .decode()
        .map_err(|_| AppError::ImageDecodeError)?;
    let img = img.resize(
        width as u32,
        height as u32,
        image::imageops::FilterType::Nearest,
    );

    let _ = img.save(save_path).map_err(|_e| AppError::ImageSaveError);
    Ok(())
}
