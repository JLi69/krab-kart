use std::fs::File;

pub struct BitMap {
    pub pixels: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl BitMap {
    //path: png image path
    pub fn from_png(path: &str) -> Result<BitMap, String> {
        let png_file = File::open(&path);

        match png_file {
            Ok(file) => {
                let decoder = png::Decoder::new(file);
                let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
                //Bytes read in from the image, copoy this into the SDL texture
                let mut png_buffer = vec![0u8; reader.output_buffer_size()];
                // Read the next frame. An APNG might contain multiple frames.
                let info = reader
                    .next_frame(&mut png_buffer)
                    .map_err(|e| e.to_string())?;
                Ok(BitMap {
                    pixels: png_buffer,
                    width: info.width as usize,
                    height: info.height as usize,
                })
            }
            Err(msg) => {
                eprintln!("{msg}");
                eprintln!("failed to open: {path}");
                Err(String::from("Failed to load bitmap from png file"))
            }
        }
    }

    //x and y are in the range 0 to 1
    pub fn sample(&self, x: f64, y: f64, pixel_data: &mut [u8]) {
        //Out of range, return black
        if x < 0.0 || x > 1.0 || y < 0.0 || y > 1.0 {
            pixel_data[0] = 0;
            pixel_data[1] = 0;
            pixel_data[2] = 0;
			return;
        }

        let ind = (x * self.width as f64).floor() as usize * 3
            + self.width * 3 * (y * self.height as f64).floor() as usize;

		pixel_data[0] = self.pixels[ind + 2];
		pixel_data[1] = self.pixels[ind + 1];
		pixel_data[2] = self.pixels[ind];
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }
    pub fn height(&self) -> u32 {
        self.height as u32
    }
}
