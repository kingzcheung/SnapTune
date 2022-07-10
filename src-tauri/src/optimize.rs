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

pub mod error;
use oxipng::Options;
use turbojpeg::{compress_image, decompress_image};

use self::error::OptimizeError;

pub trait Optimizer {
    fn optimize(&self, data: &[u8], level: u8) -> Result<Vec<u8>, OptimizeError>;
}

pub struct Png;

impl Optimizer for Png {
    fn optimize(&self, data: &[u8], level: u8) -> Result<Vec<u8>, OptimizeError> {
        let opts = Options::from_preset(level);

        let output = oxipng::optimize_from_memory(data, &opts)?;

        Ok(output)
    }
}

pub struct Jpeg;

impl Optimizer for Jpeg {
    /// On mac, need install Yasm and jpeg-turbo.
    fn optimize(&self, data: &[u8], level: u8) -> Result<Vec<u8>, OptimizeError> {
        let quality = level as i32 * 10;
        let image: image::RgbImage = decompress_image(data)?;
        let jpeg_bytes = compress_image(&image, quality, turbojpeg::Subsamp::Sub2x2)?;
        let jpeg_bytes = jpeg_bytes.to_vec();
        Ok(jpeg_bytes)
    }
}

pub struct OptimizeImage {
    data: Vec<u8>,
    compression_level: u8,
}

impl OptimizeImage {
    pub fn new(filename: &str, compression_level: u8) -> Result<Self, OptimizeError> {
        let p = std::path::Path::new(filename);
        let data = std::fs::read(p)?;
        Ok(Self {
            compression_level,
            data,
        })
    }

    pub fn optimize<T: Optimizer>(&self, fmt: T) -> Result<Vec<u8>, OptimizeError> {
        fmt.optimize(self.data.as_slice(), self.compression_level)
    }
}
