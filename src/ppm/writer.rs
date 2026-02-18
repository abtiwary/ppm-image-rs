//! Write a valid P6 PPM file to a given path. 

use std::fs::File;

use crate::ppm::ppm_image::PpmImage;
use crate::ppm::errors::PpmError;
use std::io::Write;

#[derive(Clone, Debug, Default)]
pub struct PpmWriter {
    pub ppm_image: PpmImage,
    pub file_path: String,
}

impl PpmWriter {
    pub fn new(width: usize, height: usize, bit_depth: usize, file_path: &str) -> Self {
        let mut ppm_writer = PpmWriter { 
            ppm_image: PpmImage::new(), 
            file_path: file_path.into(),
        };
        
        ppm_writer.ppm_image.width = width;
        ppm_writer.ppm_image.height = height;
        ppm_writer.ppm_image.bit_depth = bit_depth;

        ppm_writer
    }

    /// Copy a given slice of bytes into the image_data container of the PpmImage.  
    pub fn set_image_data(&mut self, data: &[u8]) {
        let mut image_data = vec![0; data.len()];
        image_data.copy_from_slice(data);
        self.ppm_image.image_data = Some(image_data);
    }

    /// Attempt to write the PPM file. 
    pub fn write_output_file(&self) -> Result<(), PpmError> {
        let f = File::create(&self.file_path).map_err(|err| {
            return PpmError::FileIO(
                "failed to create file".to_string(), 
                self.file_path.clone(), err.to_string());
        });
        let mut f = f.unwrap();
        let _ = f.write("P6\n".as_bytes()).map_err(|e| {
            return PpmError::FileIO(
                "failed to write magic bytes".to_string(), 
                self.file_path.clone(), e.to_string());
        });

        let header = format!("{} {}\n{}\n", self.ppm_image.width, self.ppm_image.height, self.ppm_image.bit_depth);
        let _ = f.write(header.as_bytes()).map_err(|e| {
            return PpmError::FileIO(
                "failed to write header bytes".to_string(), 
                self.file_path.clone(), e.to_string());
        });
        
        if let Some(ref img_data) = self.ppm_image.image_data {
            let _ = f.write(&img_data[..]).map_err(|e| {
            return PpmError::FileIO(
                "failed to write image data bytes".to_string(), 
                self.file_path.clone(), e.to_string());
            });
        } else {
            return Err(PpmError::InvalidImageData);
        }

        Ok(())
    }

}


