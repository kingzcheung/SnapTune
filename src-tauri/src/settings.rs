use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default = "default_quality")]
    pub quality: u8,
    #[serde(default)]
    #[serde(rename = "imageCompressSavePath")]
    pub image_compress_save_path: String,
    #[serde(default)]
    #[serde(rename = "imageConvertSavePath")]
    pub image_convert_save_path: String,
}

fn default_quality() -> u8 {
    80
}
