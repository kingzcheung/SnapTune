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

use std::io::Write;

use app::{self, optimize::{OptimizeImage, Jpeg}};
use image::EncodableLayout;

#[test]
fn jpeg_optimize() {
    let oi = OptimizeImage::new("./testdata/test.jpg", 4);
    assert!(oi.is_ok());
    let oi = oi.unwrap();

    let res = oi.optimize(Jpeg);
    assert!(res.is_ok());
    let data = res.unwrap();
    std::fs::File::create("testdata/test_new.jpg").unwrap().write_all(data.as_bytes()).unwrap();
}