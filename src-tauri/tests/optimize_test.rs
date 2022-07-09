use std::io::Write;

use app::{self, optimize::{OptimizeImage, Jpeg}};
use image::EncodableLayout;

#[test]
fn jpeg_optimize() {
    let oi = OptimizeImage::new("/Users/kingzcheung/Downloads/zetong-li-mVqTumQH-c0-unsplash.jpeg", 7);
    assert!(oi.is_ok());
    let oi = oi.unwrap();

    let res = oi.optimize(Jpeg);
    assert!(res.is_ok());
    let data = res.unwrap();
    std::fs::File::create("testdata/test_new.jpg").unwrap().write_all(data.as_bytes()).unwrap();
}