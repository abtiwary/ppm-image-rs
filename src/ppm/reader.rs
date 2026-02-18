//! Read a valid P6 PPM file from a given path. 

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

use crate::ppm::ppm_image::PpmImage;
use crate::ppm::errors::PpmError;

#[derive(Clone, Debug, Default)]
pub struct PpmReader {
    pub ppm_image: PpmImage,
    pub file_path: String,
}

impl PpmReader {
    pub fn new(file_path: &str) -> Self {
        PpmReader { 
            ppm_image: PpmImage::new(), 
            file_path: file_path.into(),
        }
    }

    /// Read the contents of the header from a valid PPM file - not including the magic number. 
    ///
    /// The header includes comments and two metadata lines. 
    fn read_file_header(&mut self) -> Result<(usize, usize, usize), PpmError> {
        if !self.ppm_image.is_valid_ppm {
            return Err(PpmError::InvalidFile);
        }

        let f = File::open(&self.file_path);
        if let Err(e) = f {
            return Err(PpmError::FileIO("could not read the file".to_string(), self.file_path.clone(), e.to_string()));
        }

        let mut f_reader = BufReader::new(f.unwrap());
        let _ = f_reader.seek(SeekFrom::Start(3));

        let mut pos = 0;
        let mut metadata_lines_read = 0;

        let mut temp_buffer = String::new();

        loop {
            // peek at the next byte 
            let mut byt = vec![0; 1];
            let read_byt = f_reader.read_exact(&mut byt);
            if let Err(e) = read_byt {
                return Err(PpmError::FileIO(
                    "could not peek at the next byte".to_string(), 
                    self.file_path.clone(), e.to_string()));
            }
            
            match byt[0] {
                // 0x23 is '#', i.e. the start of a comment line 
                0x23 => {
                    let _ = f_reader.seek(SeekFrom::Current(-1));
                    let _ = f_reader.read_line(&mut temp_buffer);
                    pos += temp_buffer.len();
                    let _ = &self.ppm_image.comments.push(temp_buffer.clone());
                    temp_buffer.clear();
                },
                _ => {
                    let _ = f_reader.seek(SeekFrom::Current(-1));
                    let _ = f_reader.read_line(&mut temp_buffer);
                    pos += temp_buffer.len();
                    let _ = &self.ppm_image.metadata.push(temp_buffer.clone());
                    temp_buffer.clear();
                    metadata_lines_read += 1
                },
            }

            if metadata_lines_read == 2 {
                break;
            }
        }
        
        let dims: Vec<&str> = self.ppm_image.metadata[0].split_whitespace().collect();
        let width = dims[0].parse::<usize>().unwrap();
        let height = dims[1].parse::<usize>().unwrap();

        self.ppm_image.width = width;
        self.ppm_image.height = height;

        Ok((pos, width, height))
    }
    
    /// Attempt to read the contents of a given PPM file. 
    ///
    /// First read the magic number to decide whether the file is valid, if so read the header and
    /// finally the image data. 
    pub fn read_file(&mut self) -> Result<(), PpmError> {
        let f = File::open(&self.file_path);
        if let Err(e) = f {
            return Err(PpmError::FileIO(
                "could not read the file".to_string(),
                self.file_path.clone(), e.to_string()));
        }

        let mut f_reader = BufReader::new(f.unwrap());

        let mut magic = vec![0; 2];
        let read_magic = f_reader.read_exact(&mut magic);
        if let Err(e) = read_magic {
            return Err(PpmError::FileIO(
                "could not read the magic bytes".to_string(),
                self.file_path.clone(), e.to_string()));
        }
        let valid_ppm = PpmImage::is_valid_ppm(&magic);
        if !valid_ppm {
            return Err(PpmError::InvalidFile);
        } else {
            self.ppm_image.is_valid_ppm = true;
        }

        if let Ok((pos, width, height)) = self.read_file_header() {
            let mut image_data = vec![0; 3 * width * height];
            f_reader.seek(SeekFrom::Start(3 + pos as u64)).unwrap();
            f_reader.read_exact(&mut image_data).unwrap();
            println!("image data len = {}", &image_data.len());
            self.ppm_image.image_data = Some(image_data);
        } else {
            return Err(PpmError::FileIO(
                "could not read the image data".to_string(),
                self.file_path.clone(), "error reading the header".to_string()));
        }

        Ok(())
    }

}


