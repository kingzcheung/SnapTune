use std::{fmt::Display, fs};

use serde::Serialize;

pub mod convert;
pub mod optimize;

const B: u64 = 1 << 0;
const KB: u64 = 1 << 10;
const MB: u64 = 1 << (10 * 2);
const GB: u64 = 1 << (10 * 3);
const TB: u64 = 1 << (10 * 4);
const EB: u64 = 1 << (10 * 5);
const ZB: u64 = 1 << (10 * 6);

#[derive(Debug, Serialize)]
pub struct FileSize(pub u64);

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0_u64..=B => write!(f, "{}B", self.0),
            B..=KB => write!(f, "{}KB", self.0),
            KB..=MB => write!(f, "{}MB", self.0),
            MB..=GB => write!(f, "{}GB", self.0),
            GB..=TB => write!(f, "{}TB", self.0),
            TB..=EB => write!(f, "{}EB", self.0),
            EB..=ZB => write!(f, "{}ZB", self.0),
            _ => write!(f, "{}", self.0),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Meta {
    file_size: FileSize,
    file: String,
}

// 获取文件meta信息
pub async fn file_metadata(files: Vec<String>) -> Result<Vec<Meta>, anyhow::Error> {
    let mut meta = vec![];
    for file in files {
        let m = fs::metadata(file.as_str())?;
        meta.push(Meta {
            file,
            file_size: FileSize(m.len()),
        })
    }
    Ok(meta)
}
