extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::BlendMode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;

use std::time::Instant;

mod sprite;
mod level;

/*
 * TODO:
 * - Karts
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

	let mut pixel_buffer: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

	let mut sec_per_frame: f64 = 0.0;
	let mut can_quit = false;

	let mut cam = level::Camera::new(0.0, 0.0, 0.0, 0.01, 0.4, 3.14159 / 3.0);
	let mut player_kart = sprite::Sprite::new(4.5, 15.0);
	player_kart.rotation = 3.14159 / 2.0 * 3.0;
	let krab_texture = 
		sprite::load_texture(String::from("res/kart1.png"), &texture_creator).unwrap();	
	let level1 =
		level::Level::load_from_file(String::from("res/level1.png")).unwrap();

	while !can_quit {
		let start_frame = Instant::now();	

		canvas.clear();
		let canvas_dimensions = canvas.output_size()?;
		level1.display_level(&mut pixel_buffer, WIDTH, HEIGHT, &cam);
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
	
		//Draw player kart
		player_kart.display(&mut canvas, &cam, WIDTH, HEIGHT, &canvas_dimensions, &krab_texture);

		canvas.present();

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} => { can_quit = true },
				//Test key input, delete later
				Event::KeyDown { keycode: Some(Keycode::Up), .. } => { player_kart.speed = 2.0; },	
				Event::KeyDown { keycode: Some(Keycode::Down), .. } => { player_kart.speed = -1.0; },
				Event::KeyDown { keycode: Some(Keycode::Left), .. } => { player_kart.rotation_speed = -1.5; },
				Event::KeyDown { keycode: Some(Keycode::Right), .. } => { player_kart.rotation_speed = 1.5; },
				Event::KeyUp { keycode: Some(Keycode::Up), .. } => { player_kart.speed = 0.0; },	
				Event::KeyUp { keycode: Some(Keycode::Down), .. } => { player_kart.speed = 0.0; },
				Event::KeyUp { keycode: Some(Keycode::Left), .. } => { player_kart.rotation_speed = 0.0; },
				Event::KeyUp { keycode: Some(Keycode::Right), .. } => { player_kart.rotation_speed = 0.0; },
				_ => {}
			}
		}

		player_kart.trans_x += player_kart.rotation.sin() * player_kart.speed * sec_per_frame;
		player_kart.trans_z += player_kart.rotation.cos() * player_kart.speed * sec_per_frame;
		player_kart.rotation += player_kart.rotation_speed * sec_per_frame;

		if player_kart.rotation_speed < 0.0 {
			player_kart.frame = 7;	
		} else if player_kart.rotation_speed > 0.0 {
			player_kart.frame = 1;	
		} else {
			player_kart.frame = 0;	
		}

		cam.trans_x = player_kart.trans_x - player_kart.rotation.sin() * 0.75;
		cam.trans_z = player_kart.trans_z - player_kart.rotation.cos() * 0.75;
		cam.rotation = player_kart.rotation;

		sec_per_frame = start_frame.elapsed().as_secs_f64();
		//Output frames per second
		//println!("FPS: {}", 1.0 / sec_per_frame);
	}

	Ok(())
}
