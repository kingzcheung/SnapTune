pub mod error;
use oxipng::Options;

use self::error::OptimizeError;

pub trait Optimizer {
    fn optimize(&self,data: &[u8],level:u8)->Result<Vec<u8>,OptimizeError>;
}

pub struct Png;

impl Optimizer for Png {
    fn optimize(&self,data: &[u8],level:u8)->Result<Vec<u8>,OptimizeError> {
        let opts = Options::from_preset(level);

        let output = oxipng::optimize_from_memory(data, &opts)?;

        Ok(output)
    }
}

pub struct Jpeg;

impl Optimizer for Jpeg {
    fn optimize(&self,data: &[u8],level:u8)->Result<Vec<u8>,OptimizeError> {

        // let result = std::panic::catch_unwind(|| {
        //     let mut comp = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
        
        //     comp.set_size(width, height);
        //     comp.set_mem_dest();
        //     comp.start_compress();
        
        //     // replace with your image data
        //     let pixels = vec![0; width * height * 3];
        //     assert!(comp.write_scanlines(&pixels[..]));
        
        //     comp.finish_compress();
        //     let jpeg_bytes = comp.data_to_vec()?;
        //     // write to file, etc.
        //     Ok(jpeg_bytes)
        // })?;

    Ok(vec![])
    }
}



pub struct OptimizeImage {
    data: Vec<u8>,
    compression_level: u8,
}

impl OptimizeImage {
    pub fn new(filename: &str, compression_level: u8) -> Result<Self,OptimizeError> {
        let p = std::path::Path::new(filename);
        let data = std::fs::read(p)?;
        Ok(Self { compression_level, data })
    }


    pub fn optimize<T:Optimizer>(&self,fmt:T)->Result<Vec<u8>,OptimizeError> {
        fmt.optimize(self.data.as_slice(),self.compression_level)
    }
}


