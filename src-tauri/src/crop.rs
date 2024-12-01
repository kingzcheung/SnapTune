use image::ImageReader;

use crate::error::AppError;



pub async fn crop_image(image_path: String,crop_width: u32, crop_height: u32,crop_x: u32,crop_y: u32,save_path: String)-> Result<(), AppError> {
    let mut img = ImageReader::open(image_path)?.decode().map_err(|_|AppError::ImageDecodeError)?;
    let img = img.crop(crop_x as u32, crop_y as u32, crop_width as u32, crop_height as u32);

    let _ = img.save(save_path).map_err(|e|AppError::ImageSaveError);

    Ok(())
}