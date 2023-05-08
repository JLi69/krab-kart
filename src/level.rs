use std::fs::File;

pub struct Camera {
	pub trans_x: f64,
	pub trans_z: f64,
	pub rotation: f64,
	
	pub fov: f64,

	pub x_near1: f64,
	pub x_near2: f64,
	pub x_far1: f64,
	pub x_far2: f64,
	pub z_near: f64,
	pub z_far: f64
}

pub struct Level {
	level_data: Vec<u8>,
	level_width: u32,
	level_height: u32
}

impl Level {
	//Loads the level from a file (a png image)
	pub fn load_from_file(path: String) -> Result<Level, String> {
		let level_file = File::open(path.clone());

		match level_file {
			Ok(file) => {
				let decoder = png::Decoder::new(file);
				let mut reader = decoder.read_info().unwrap();
				let mut buff = vec![0u8; reader.output_buffer_size()];
				// Read the next frame. An APNG might contain multiple frames.
				let info = reader.next_frame(&mut buff).unwrap();
			
				Ok(Level { 
					level_data: buff,
					level_width: info.width, level_height: info.height
				})
			},
			Err(msg) => {
				//Could not open level file, print error message and return error
				eprintln!("{msg}");
				eprintln!("Cannot open file: {path}");
				Err(String::from("Failed to open level file"))
			}
		}	
	}

	pub fn display_level(&self,
						 pixel_buffer: &mut [u8],
						 buff_width: usize,
						 buff_height: usize,
						 cam: &Camera) {
		//Update the pixel buffer
		for y in 0..buff_height {
			
			let depth = 
				(y as f64 - buff_height as f64 / 8.0 * 3.0) / (buff_height / 8 * 5) as f64;
			let startx = cam.x_near1 + (cam.x_far1 - cam.x_near1)  / depth;
			let endx = cam.x_near2 + (cam.x_far2 - cam.x_near2) / depth;

			let sample_z = cam.z_near + (cam.z_far - cam.z_near) / depth; 

			for x in 0..buff_width {
				let offset = y * 4 * buff_width + x * 4;
				pixel_buffer[offset] = 255;
				pixel_buffer[offset + 1] = 128;	
				pixel_buffer[offset + 2] = 32;
				pixel_buffer[offset + 3] = 255;
			
				let sample_x = startx + (endx - startx) * (x as f64 / buff_width as f64);	

				let rotated_x = sample_x * (-cam.rotation).cos() - (-cam.rotation).sin() * sample_z;
				let rotated_z = sample_x * (-cam.rotation).sin() + (-cam.rotation).cos() * sample_z;
				let trans_x = (rotated_x + cam.trans_x) * 32.0;
				let trans_z = (rotated_z + cam.trans_z) * 32.0;

				let ind = ((trans_x).floor() * 3.0 + (trans_z).floor() * 3.0 * self.level_width as f64) as usize;

				//Ground
				if y > buff_height / 8 * 3 {
					pixel_buffer[offset] = 0;
					pixel_buffer[offset + 1] = 255;	
					pixel_buffer[offset + 2] = 0;


					if trans_x < 0.0 || (trans_x) as u32 >= self.level_width ||
					   trans_z < 0.0 || (trans_z) as u32 >= self.level_height {
						continue;
					}
					

					if ind < self.level_data.len() {
						pixel_buffer[offset] = self.level_data[ind + 2];
						pixel_buffer[offset + 1] = self.level_data[ind + 1];	
						pixel_buffer[offset + 2] = self.level_data[ind];
					}
				}
			}
		}
	}
}

impl Camera {
	//Creates camera, cam_fov and cam_rot are in radians
	pub fn new(x: f64, z: f64,
			   cam_rot: f64,
			   near: f64, far: f64,
			   cam_fov: f64) -> Camera {
		let cam_x_near1 = (-cam_fov / 2.0).sin() * near;
		let cam_x_near2 = (cam_fov / 2.0).sin() * near;
		let cam_x_far1 = (-cam_fov / 2.0).sin() * far;
		let cam_x_far2 = (cam_fov / 2.0).sin() * far;	
		let cam_z_near = (cam_fov / 2.0).cos() * near;
		let cam_z_far = (cam_fov / 2.0).cos() * far;

		Camera {
			trans_x: x,
			trans_z: z,
			rotation: cam_rot,
			fov: cam_fov, 
		
			x_near1: cam_x_near1,
			x_near2: cam_x_near2,
			x_far1: cam_x_far1,
			x_far2: cam_x_far2,
			z_near: cam_z_near,
			z_far: cam_z_far
		}
	}
}
