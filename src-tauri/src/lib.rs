use std::fs;

use serde::Serialize;

pub mod convert;
pub mod optimize;

#[derive(Debug, Serialize)]
pub struct Meta {
    file_size: u64,
    file: String,
}

// 获取文件meta信息
pub async fn file_metadata(files: Vec<String>) -> Result<Vec<Meta>, anyhow::Error> {
    let mut meta = vec![];
    for file in files {
        let m = fs::metadata(file.as_str())?;
        meta.push(Meta {
            file,
            file_size: m.len(),
        })
    }
    Ok(meta)
}
