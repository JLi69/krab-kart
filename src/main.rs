extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::{Texture, BlendMode};
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::fs::File;
use std::time::Instant;

pub struct Camera {
	pub trans_x: f64,
	pub trans_z: f64,
	pub rotation: f64,
	
	pub z_near: f64,
	pub z_far: f64,
	pub fov: f64
}

struct Sprite {
	pub trans_x: f64,
	pub trans_z: f64
}

impl Sprite {
	pub fn new(x: f64, z: f64) -> Sprite {
		Sprite {
			trans_x: x,
			trans_z: z
		}
	}
}

pub fn load_texture<'a>(path: &String, 
					texture_creator: &'a TextureCreator<WindowContext>) -> Texture<'a> {
	let decoder = png::Decoder::new(File::open(path).unwrap());
	let mut reader = decoder.read_info().unwrap();
	let mut png_buffer = vec![0u8; reader.output_buffer_size()];
	// Read the next frame. An APNG might contain multiple frames.
	let info = reader.next_frame(&mut png_buffer).unwrap();

	let mut tex = texture_creator.
		create_texture_streaming(PixelFormatEnum::ARGB8888, info.width, info.height)
		.unwrap();
	tex.set_blend_mode(BlendMode::Blend);

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
	
	tex
}

/*
 * TODO:
 * - Karts
 * - Art
 * - Banana
 * - Speed Boost
 * - Singleplayer
 * - Multiplayer (same computer)
 * - Level select
 * */

fn main() -> Result<(), String> {	
	// Initialize SDL2
	let sdl_context = sdl2::init().unwrap();

	let vid_subsystem = sdl_context.video().unwrap();
	
	//Create window
	let window = vid_subsystem.window("Krab Kart", 960, 540)
				.position_centered()
				.resizable()
				.build()
				.unwrap();

	const WIDTH: usize = 480;
	const HEIGHT: usize = 270;

	//Create canvas
	let mut canvas = window.into_canvas().present_vsync().accelerated().build().unwrap();
	canvas.set_blend_mode(BlendMode::Blend);
	let mut event_pump = sdl_context.event_pump().unwrap();
	let texture_creator = canvas.texture_creator();
	let mut texture = texture_creator.
		create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH as u32, HEIGHT as u32).
		unwrap();
	texture.set_blend_mode(BlendMode::Blend);

	let decoder = png::Decoder::new(File::open("res/level1.png").unwrap());
	let mut reader = decoder.read_info().unwrap();
	let mut test_level = vec![0u8; reader.output_buffer_size()];
	// Read the next frame. An APNG might contain multiple frames.
	let info = reader.next_frame(&mut test_level).unwrap();

	let mut pixel_buffer: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

	let mut sec_per_frame: f64 = 0.0;
	let mut can_quit = false;

	let mut cam = Camera {
		trans_x: 4.0,
		trans_z: 8.0,
		rotation: 3.14159 / 2.0,
		z_near: 0.01,
		z_far: 0.4,
		fov: 3.14159 / 3.0
	};

	let player_kart = Sprite::new(8.0, 8.0);
	let krab_texture = load_texture(&String::from("res/kart1.png"), &texture_creator);

	let x_near1 = (-cam.fov / 2.0).sin() * cam.z_near;
	let x_near2 = (cam.fov / 2.0).sin() * cam.z_near;
	let x_far1 = (-cam.fov / 2.0).sin() * cam.z_far;
	let x_far2 = (cam.fov / 2.0).sin() * cam.z_far;
	
	let z_near = (cam.fov / 2.0).cos() * cam.z_near;
	let z_far = (cam.fov / 2.0).cos() * cam.z_far;
	

	while !can_quit {
		let start_frame = Instant::now();

		//Update the pixel buffer
		for y in 0..HEIGHT {
			
			let depth = (y as f64 - HEIGHT as f64 / 8.0 * 3.0) / (HEIGHT / 8 * 5) as f64;
			let startx = x_near1 + (x_far1 - x_near1)  / depth;
			let endx = x_near2 + (x_far2 - x_near2) / depth;

			let sample_z = z_near + (z_far - z_near) / depth; 

			for x in 0..WIDTH {
				let offset = y * 4 * WIDTH + x * 4;
				pixel_buffer[offset] = 255;
				pixel_buffer[offset + 1] = 128;	
				pixel_buffer[offset + 2] = 32;
				pixel_buffer[offset + 3] = 255;
			
				let sample_x = startx + (endx - startx) * (x as f64 / WIDTH as f64);	

				let rotated_x = sample_x * (-cam.rotation).cos() - (-cam.rotation).sin() * sample_z;
				let rotated_z = sample_x * (-cam.rotation).sin() + (-cam.rotation).cos() * sample_z;
				let trans_x = (rotated_x + cam.trans_x) * 64.0;
				let trans_z = (rotated_z + cam.trans_z) * 64.0;

				let ind = ((trans_x).floor() * 3.0 + (trans_z).floor() * 3.0 * info.width as f64) as usize;

				//Ground
				if y > HEIGHT / 8 * 3 {
					pixel_buffer[offset] = 0;
					pixel_buffer[offset + 1] = 255;	
					pixel_buffer[offset + 2] = 0;


					if trans_x < 0.0 || (trans_x) as u32 >= info.width ||
					   trans_z < 0.0 || (trans_z) as u32 >= info.height {
						continue;
					}
					

					if ind < test_level.len() {
						pixel_buffer[offset] = test_level[ind + 2];
						pixel_buffer[offset + 1] = test_level[ind + 1];	
						pixel_buffer[offset + 2] = test_level[ind];
					}	
				}
			}
		}

		canvas.clear();
		let canvas_dimensions = canvas.output_size()?;	
		texture.update(None, &pixel_buffer, WIDTH * 4).unwrap();
		let texture_rect: Rect;
		
		//Resize the texture
		if canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32 > canvas_dimensions.0 {
			texture_rect = Rect::from_center(
					Point::new(
						canvas_dimensions.0 as i32 / 2,
						canvas_dimensions.1 as i32 / 2),
						canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32, canvas_dimensions.1);	
		} else {	
			texture_rect = Rect::from_center(
					Point::new(
						canvas_dimensions.0 as i32 / 2,
						canvas_dimensions.1 as i32 / 2),
						canvas_dimensions.0, canvas_dimensions.0 * HEIGHT as u32 / WIDTH as u32);	
		}

		canvas.copy(&texture, None, texture_rect).unwrap();	


		//Test graphic draw sprite
		let trans_sprite_x = (player_kart.trans_x - cam.trans_x) * (cam.rotation).cos()
							- (player_kart.trans_z - cam.trans_z) * (cam.rotation).sin();
		let trans_sprite_z = (player_kart.trans_z - cam.trans_z) * (cam.rotation).cos()
							+ (player_kart.trans_x - cam.trans_x) * (cam.rotation).sin();
		let sprite_w = (48.0) / trans_sprite_z;
		let sprite_h = (48.0) / trans_sprite_z;

		let spr_screen_y = HEIGHT as f64 * (z_far - z_near) / ((trans_sprite_z - z_near) * 8.0 / 5.0) + HEIGHT as f64 / 8.0 * 3.0;
		let depth = (z_far - z_near) / (trans_sprite_z - cam.z_near);
	
		let startx = x_near1 + (x_far1 - x_near1) / depth;
		let endx = x_near2 + (x_far2 - x_near2) / depth;
		let spr_screen_x = (trans_sprite_x - startx) / (endx - startx) * WIDTH as f64;

		let sprite_rect: Rect;

		if canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32 > canvas_dimensions.0 {
			let x_offset = -(canvas_dimensions.1 as i32 * WIDTH as i32 / HEIGHT as i32) / 2 + canvas_dimensions.0 as i32 / 2 as i32;

			sprite_rect = Rect::new(
				((canvas_dimensions.1 as f64 * WIDTH as f64 / HEIGHT as f64) * (spr_screen_x) / (WIDTH as f64) - sprite_w / 2.0 * (canvas_dimensions.1 as f64 / HEIGHT as f64)) as i32 + x_offset,
				(canvas_dimensions.1 as f64 * (spr_screen_y) / (HEIGHT as f64) - sprite_h * (canvas_dimensions.1 as f64 / HEIGHT as f64)) as i32,
				(sprite_w * canvas_dimensions.1 as f64 / HEIGHT as f64) as u32,
				(sprite_h * canvas_dimensions.1 as f64 / HEIGHT as f64) as u32);
		} else {	
			let y_offset = -(canvas_dimensions.0 as i32 * HEIGHT as i32 / WIDTH as i32) / 2 + canvas_dimensions.1 as i32 / 2 as i32;

			sprite_rect = Rect::new(
				(canvas_dimensions.0 as f64 * (spr_screen_x) / (WIDTH as f64) - sprite_w / 2.0 * (canvas_dimensions.0 as f64 / WIDTH as f64)) as i32,
				((canvas_dimensions.0 as f64 * HEIGHT as f64 / WIDTH as f64) * (spr_screen_y) / (HEIGHT as f64) - sprite_h * (canvas_dimensions.0 as f64 / WIDTH as f64)) as i32 + y_offset,
				(sprite_w as f64 * canvas_dimensions.0 as f64 / WIDTH as f64) as u32,
				(sprite_h as f64 * canvas_dimensions.0 as f64 / WIDTH as f64) as u32);	
		}

		if (player_kart.trans_x - cam.trans_x) * (player_kart.trans_x - cam.trans_x) +
			(player_kart.trans_z - cam.trans_z) * (player_kart.trans_z - cam.trans_z) < 8.0 * 8.0 {
			canvas.copy(&krab_texture, Rect::new(0, 0, 32, 32), sprite_rect).unwrap();	
		}

		canvas.present();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => { can_quit = true },
				//Test key input, delete later
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => {	
					cam.trans_x += cam.rotation.sin() * 0.1;
					cam.trans_z += cam.rotation.cos() * 0.1; 
				},	
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => {	
					cam.trans_x -= cam.rotation.sin() * 0.1;
					cam.trans_z -= cam.rotation.cos() * 0.1;
				},
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => { 
					cam.rotation -= 0.05;	
				},
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => { 	
					cam.rotation += 0.05;	
				},
				_ => {}
			}
		}	

		sec_per_frame = start_frame.elapsed().as_secs_f64();
		//Output frames per second
		//println!("FPS: {}", 1.0 / sec_per_frame);
	}

	Ok(())
}
