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
