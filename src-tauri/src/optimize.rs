use oxipng::Options;

use crate::convert::Format;
use std::{ffi::OsStr, path::{Path, PathBuf}, any};

pub trait Optimizer {
    fn optimize(&self)->Result<(),anyhow::Error>;
}

pub struct Png(String);

impl Png {
    pub fn new(filename: String)->Self {
        Self(filename)
    }
}

impl Optimizer for Png {
    fn optimize(&self)->Result<(),anyhow::Error> {
        // let p = Path::new(&self.0);
        let pb = PathBuf::from(&self.0);
        
        let input = oxipng::InFile::Path(pb.clone());
        let output = oxipng::OutFile::Path(Some(pb));
        let opts = Options::max_compression();
        oxipng::optimize(&input, &output, &opts)?;

        Ok(())
    }
}

pub struct OptimizeImage<T> {
    format: Format,
    filename: String,
    optim: T
}

impl<T: Optimizer> OptimizeImage<T> {
    pub fn new(format: Format, filename: String) -> Self {
        Self { format, filename }
    }

    pub fn from_file(filename: String) -> Option<Self> {
        let p = std::path::Path::new(filename.as_str());

        let f = p.extension().and_then(OsStr::to_str)?;
        let format = f.to_string().into();
        let s = Self { format, filename };

        Some(s)
    }


    pub fn optimize(&self)->Result<(),anyhow::Error> {
        self.optim
    }
}


