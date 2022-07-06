use std::{fs};

use serde::Serialize;

pub mod convert;
pub mod optimize;

#[derive(Debug,Serialize)]
pub struct Meta {
    length: u64
}
// 获取文件meta信息
pub fn file_metadatas(files: Vec<String>) ->Result<Vec<Meta>, anyhow::Error> {
    let mut meta = vec![];
    for file in files{
        let m = fs::metadata(file)?;
        meta.push(Meta{
            length: m.len(),
        })
    }
    Ok(meta)
}