
#[derive(Clone, Debug)]
pub struct PpmImage {
    pub is_valid_ppm: bool,
    pub comments: Vec<String>,
    pub metadata: Vec<String>,
    pub width: usize,
    pub height: usize,
    pub bit_depth: usize,
    pub image_data: Option<Vec<u8>>,
}

impl PpmImage {
    pub fn new() -> Self {
        PpmImage {
            is_valid_ppm: false,
            comments: Vec::new(),
            metadata: Vec::new(),
            width: 0, 
            height: 0, 
            bit_depth: 0,
            image_data: None,
        }
    }

    pub fn is_valid_ppm(magic_vec: &[u8]) -> bool {
        if magic_vec.len() != 2 {
            return false;
        }

        match magic_vec {
            [80, 54] => true,
            _ => false,    
        }
    }
}

impl Default for PpmImage {
    fn default() -> Self {
        PpmImage::new()
    }
}

