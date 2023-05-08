use sdl2::rect::Rect;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, BlendMode, TextureCreator, Canvas};
use sdl2::video::{Window, WindowContext};
use std::fs::File;
use crate::level::Camera;

pub struct Sprite {
	pub trans_x: f64,
	pub trans_z: f64,
	pub rotation: f64,

	pub speed: f64,
	pub rotation_speed: f64,

	pub frame: i32
}

//Uses png crate to load bytes from a PNG file and then
//copies those bytes into a texture
//Works only on PNG images
pub fn load_texture<'a>(path: String, 
						texture_creator: &'a TextureCreator<WindowContext>
						) -> Result<Texture<'a>, String> {
	let img_file = File::open(path.clone());

	match img_file {
		Ok(..) => {
			let decoder = png::Decoder::new(File::open(path).unwrap());
			let mut reader = decoder.read_info().unwrap();
			//Bytes read in from the image, copoy this into the SDL texture
			let mut png_buffer = vec![0u8; reader.output_buffer_size()];
			// Read the next frame. An APNG might contain multiple frames.
			let info = reader.next_frame(&mut png_buffer).unwrap();
			
			//Create empty SDL texture
			let mut tex = texture_creator.
				create_texture_streaming(PixelFormatEnum::ARGB8888, info.width, info.height)
				.unwrap();
			//Enable Alpha so we can have transparency
			tex.set_blend_mode(BlendMode::Blend);
			
			//Write pixels into texture
			tex.with_lock(None, |pixels: &mut [u8], _pitch: usize| {
				for i in 0..(reader.output_buffer_size() as usize) {
					//pixels[i] = png_buffer[i];
					if i % 4 == 0 {
						pixels[i] = png_buffer[i + 2];
						pixels[i + 1] = png_buffer[i + 1];	
						pixels[i + 2] = png_buffer[i];	
						pixels[i + 3] = png_buffer[i + 3];	
					}
				}
			}).unwrap();
	
			Ok(tex)
		},
		Err(msg) => {
			eprintln!("{msg}");
			eprintln!("Cannot open image: {path}");		
			Err(String::from("Failed to load texture"))
		}
	}	
}

impl Sprite {
	pub fn new(x: f64, z: f64) -> Sprite {
		Sprite {
			trans_x: x,
			trans_z: z,
			rotation: 0.0,

			speed: 0.0,
			rotation_speed: 0.0,
			frame: 0
		}
	}

	pub fn display(&self, 
				   canv: &mut Canvas<Window>, 
				   cam: &Camera,
				   pixel_buff_width: usize,
				   pixel_buff_height: usize,
				   canvas_dimensions: &(u32, u32),
				   texture: &Texture) {
		//Test graphic draw sprite
		let trans_sprite_x = (self.trans_x - cam.trans_x) * (cam.rotation).cos()
							- (self.trans_z - cam.trans_z) * (cam.rotation).sin();
		let trans_sprite_z = (self.trans_z - cam.trans_z) * (cam.rotation).cos()
							+ (self.trans_x - cam.trans_x) * (cam.rotation).sin();
		let sprite_w = (48.0) / trans_sprite_z;
		let sprite_h = (48.0) / trans_sprite_z;

		let spr_screen_y = pixel_buff_height as f64 * (cam.z_far - cam.z_near) / ((trans_sprite_z - cam.z_near) * 8.0 / 5.0) + pixel_buff_height as f64 / 8.0 * 3.0;
		let depth = (cam.z_far - cam.z_near) / (trans_sprite_z - cam.z_near);
	
		let startx = cam.x_near1 + (cam.x_far1 - cam.x_near1) / depth;
		let endx = cam.x_near2 + (cam.x_far2 - cam.x_near2) / depth;
		let spr_screen_x = (trans_sprite_x - startx) / (endx - startx) * pixel_buff_width as f64;

		let sprite_rect: Rect;

		if canvas_dimensions.1 * pixel_buff_width as u32 / pixel_buff_height as u32 > canvas_dimensions.0 {
			let x_offset = -(canvas_dimensions.1 as i32 * pixel_buff_width as i32 / pixel_buff_height as i32) / 2 +
				canvas_dimensions.0 as i32 / 2 as i32;

			sprite_rect = Rect::new(
				 ((canvas_dimensions.1 as f64 * pixel_buff_width as f64 / pixel_buff_height as f64) * (spr_screen_x) / (pixel_buff_width as f64) - 
				 sprite_w / 2.0 * (canvas_dimensions.1 as f64 / pixel_buff_height as f64)) as i32 + x_offset,
				
				 (canvas_dimensions.1 as f64 * (spr_screen_y) / (pixel_buff_height as f64) - 
				  sprite_h * (canvas_dimensions.1 as f64 / pixel_buff_height as f64)) as i32,
				
				 (sprite_w * canvas_dimensions.1 as f64 / pixel_buff_height as f64) as u32,
				
				 (sprite_h * canvas_dimensions.1 as f64 / pixel_buff_height as f64) as u32);
		} else {	
			let y_offset = -(canvas_dimensions.0 as i32 * pixel_buff_height as i32 / pixel_buff_width as i32) / 2 +
				canvas_dimensions.1 as i32 / 2 as i32;

			sprite_rect = Rect::new(
				 (canvas_dimensions.0 as f64 * (spr_screen_x) / (pixel_buff_width as f64) - 
				 sprite_w / 2.0 * (canvas_dimensions.0 as f64 / pixel_buff_width as f64)) as i32,
				
				 ((canvas_dimensions.0 as f64 * pixel_buff_height as f64 / pixel_buff_width as f64) * (spr_screen_y) / (pixel_buff_height as f64) - 
				  sprite_h * (canvas_dimensions.0 as f64 / pixel_buff_width as f64)) as i32 + y_offset,
				
				 (sprite_w as f64 * canvas_dimensions.0 as f64 / pixel_buff_width as f64) as u32,
				
				 (sprite_h as f64 * canvas_dimensions.0 as f64 / pixel_buff_width as f64) as u32);	
		}

		if (self.trans_x - cam.trans_x) * (self.trans_x - cam.trans_x) +
			(self.trans_z - cam.trans_z) * (self.trans_z - cam.trans_z) < 8.0 * 8.0 {
			canv.copy(texture, Rect::new(self.frame * 32, 0, 32, 32), sprite_rect).unwrap();	
		}
	}
}
