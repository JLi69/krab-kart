#![windows_subsystem = "windows"]
extern crate sdl2;

use sdl2::render::BlendMode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Rect, Point};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use std::time::Instant;

mod sprite;
mod level;
mod events;

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

	const WIDTH: usize = 320;
	const HEIGHT: usize = 180;

	//Create canvas
	let mut canvas = window.into_canvas()
						   .present_vsync()
						   .accelerated()
						   .build()
						   .unwrap();
	canvas.set_blend_mode(BlendMode::Blend);
	let texture_creator = canvas.texture_creator();
	let mut texture = texture_creator.
		create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH as u32, HEIGHT as u32 / 2).
		unwrap();
	texture.set_blend_mode(BlendMode::Blend);
	let mut pixel_buffer: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

	let mut sec_per_frame: f64 = 0.0;
	let mut events = events::Events::new(&sdl_context);

	let mut cam1 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 0.4, 3.14159 / 3.0);	
	let mut cam2 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 0.4, 3.14159 / 3.0);
	
	let mut player_kart1 = sprite::Sprite::new(9.0, 30.0);
	player_kart1.frame_count = 8;
	player_kart1.rotation = 3.14159 / 2.0 * 3.0;
	
	let mut player_kart2 = sprite::Sprite::new(9.0, 30.2);
	player_kart2.rotation = 3.14159 / 2.0 * 3.0;
	player_kart2.frame_count = 8;

	let krab_texture = 
		sprite::load_texture(String::from("res/kart1.png"), &texture_creator).unwrap();		
	let krab_texture2 = 
		sprite::load_texture(String::from("res/kart2.png"), &texture_creator).unwrap();
	let mut level =
		level::Level::load_from_png(String::from("res/level.png")).unwrap();
	level.level_scale = 16.0;

	let font_ctx = sdl2::ttf::init().unwrap();
	let font = font_ctx.load_font("res/fonts/8BitOperator/8bitOperatorPlus-Regular.ttf", 32).unwrap();

	while !events.can_quit {
		let start_frame = Instant::now();

		canvas.clear();
		let canvas_dimensions = canvas.output_size()?;

		let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
		
		let sz = pixel_buffer.len() / 2;
		level.display_level(&mut pixel_buffer[0..sz], WIDTH, HEIGHT / 2, &cam1);
		texture.update(None, &pixel_buffer[0..sz], WIDTH * 4).unwrap();
		let mut texture_rect: Rect;	
		//Resize the texture
		if canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32 > canvas_dimensions.0 {
			texture_rect = Rect::from_center(
					Point::new(
						canvas_dimensions.0 as i32 / 2,
						canvas_dimensions.1 as i32 / 4),
						canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32, canvas_dimensions.1 / 2);	
		} else {	
			texture_rect = Rect::from_center(
					Point::new(
						canvas_dimensions.0 as i32 / 2,
						canvas_dimensions.1 as i32 / 2 - canvas_dimensions.0 as i32 * HEIGHT as i32 / WIDTH as i32 / 4),
						canvas_dimensions.0, canvas_dimensions.0 * HEIGHT as u32 / WIDTH as u32 / 2);		
		}
		canvas.copy(&texture, None, texture_rect).unwrap();	

		if canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32 > canvas_dimensions.0 {
			player_kart2.set_rotation_frame(&cam1);	
			player_kart1.set_kart_rotation_frame();
			if player_kart2.dist2_to_camera(&cam1) > player_kart1.dist2_to_camera(&cam1) {
				player_kart2.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, 0), &krab_texture2);
				player_kart1.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, 0), &krab_texture);
			} else {
				player_kart1.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, 0), &krab_texture);
				player_kart2.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, 0), &krab_texture2);
			}	
		} else {	
			player_kart2.set_rotation_frame(&cam1);	
			player_kart1.set_kart_rotation_frame();
			if player_kart2.dist2_to_camera(&cam1) > player_kart1.dist2_to_camera(&cam1) {
				player_kart2.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, texture_rect.y() / 2), &krab_texture2);
				player_kart1.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, texture_rect.y() / 2), &krab_texture);
			} else {
				player_kart1.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, texture_rect.y() / 2), &krab_texture);
				player_kart2.display(&mut canvas, &cam1, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, texture_rect.y() / 2), &krab_texture2);
			}
		}	

		let origin_y = texture_rect.y() / 2;

		level.display_level(&mut pixel_buffer[sz..], WIDTH, HEIGHT / 2, &cam2);
		texture.update(None, &pixel_buffer[sz..], WIDTH * 4).unwrap();
		//Resize the texture
		if canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32 > canvas_dimensions.0 {
			texture_rect = Rect::from_center(
					Point::new(
						canvas_dimensions.0 as i32 / 2,
						canvas_dimensions.1 as i32 / 4 * 3),
						canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32, canvas_dimensions.1 / 2);	
		} else {	
			texture_rect = Rect::from_center(
					Point::new(
						canvas_dimensions.0 as i32 / 2,
						canvas_dimensions.1 as i32 / 2 + canvas_dimensions.0 as i32 * HEIGHT as i32 / WIDTH as i32 / 4),
						canvas_dimensions.0, canvas_dimensions.0 * HEIGHT as u32 / WIDTH as u32 / 2);	
		}
		canvas.copy(&texture, None, texture_rect).unwrap();	

		if canvas_dimensions.1 * WIDTH as u32 / HEIGHT as u32 > canvas_dimensions.0 {
			player_kart1.set_rotation_frame(&cam2);
			player_kart2.set_kart_rotation_frame();	
			if player_kart2.dist2_to_camera(&cam2) > player_kart1.dist2_to_camera(&cam2) {
				player_kart2.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, canvas_dimensions_half.1 as i32), &krab_texture2);
				player_kart1.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, canvas_dimensions_half.1 as i32), &krab_texture);
			} else {
				player_kart1.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, canvas_dimensions_half.1 as i32), &krab_texture);
				player_kart2.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, canvas_dimensions_half.1 as i32), &krab_texture2);
			}
		} else {
			player_kart1.set_rotation_frame(&cam2);
			player_kart2.set_kart_rotation_frame();	
			if player_kart2.dist2_to_camera(&cam2) > player_kart1.dist2_to_camera(&cam2) {
				player_kart2.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, origin_y + texture_rect.height() as i32), &krab_texture2);
				player_kart1.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, origin_y + texture_rect.height() as i32), &krab_texture);
			} else {
				player_kart1.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, origin_y + texture_rect.height() as i32), &krab_texture);
				player_kart2.display(&mut canvas, &cam2, WIDTH, HEIGHT / 2, &canvas_dimensions_half, &(0, origin_y + texture_rect.height() as i32), &krab_texture2);
			}
		}
		
		let font_surface = font.render(format!("speed: {}", ((player_kart1.speed * 100.0).round() / 100.0)).as_str()).solid(Color::WHITE).map_err(|e| e.to_string())?;
		let font_texture = texture_creator
			.create_texture_from_surface(&font_surface)
			.map_err(|e| e.to_string())?;
		canvas.copy(&font_texture, None, Rect::new(16, 16, 16 * format!("speed: {}", ((player_kart1.speed * 100.0).round() / 100.0)).len() as u32, 32)).unwrap();

		let font_surface = font.render(format!("speed: {}", ((player_kart2.speed * 100.0).round() / 100.0)).as_str()).solid(Color::WHITE).map_err(|e| e.to_string())?;
		let font_texture = texture_creator
			.create_texture_from_surface(&font_surface)
			.map_err(|e| e.to_string())?;
		canvas.copy(&font_texture, None, Rect::new(16, 16 + canvas_dimensions_half.1 as i32, 16 * format!("speed: {}", ((player_kart2.speed * 100.0).round() / 100.0)).len() as u32, 32)).unwrap();

		let font_surface = font.render(format!("FPS: {}", ((1.0 / sec_per_frame).round())).as_str()).solid(Color::WHITE).map_err(|e| e.to_string())?;
		let font_texture = texture_creator
			.create_texture_from_surface(&font_surface)
			.map_err(|e| e.to_string())?;
		canvas.copy(&font_texture, None, Rect::new(canvas_dimensions.0 as i32 - 16 - (16 * format!("FPS: {}", ((1.0 / sec_per_frame).round())).len()) as i32,
												   16,
												   16 * format!("FPS: {}", ((1.0 / sec_per_frame).round())).len() as u32, 32)).unwrap();

		canvas.present();
		events.update();
	
		//Move the karts
		player_kart1.drive_kart(&events, Keycode::Up, Keycode::Left, Keycode::Right);	
		player_kart2.drive_kart(&events, Keycode::W, Keycode::A, Keycode::D);
		player_kart1.move_sprite(sec_per_frame);
		player_kart2.move_sprite(sec_per_frame);

		//Sample color at position - test
		let color = level.sample_color(player_kart1.trans_x, player_kart1.trans_z);
		
		//Friction
		if color[0] == 0 && color[1] >= 128 && color[2] == 0 {	
			player_kart1.max_speed = 0.5;
			player_kart1.friction = 0.1;
		} else {	
			player_kart1.max_speed = 3.0;
			player_kart1.friction = 0.4;
		}

		let color = level.sample_color(player_kart2.trans_x, player_kart2.trans_z);
		
		//Friction
		if color[0] == 0 && color[1] >= 128 && color[2] == 0 {	
			player_kart2.max_speed = 0.5;
			player_kart2.friction = 0.1;
		} else {	
			player_kart2.max_speed = 3.0;
			player_kart2.friction = 0.4;
		}

		//println!("{} {} {}", color[0], color[1], color[2]);
	
		cam1.follow(&player_kart1, 0.75);
		cam2.follow(&player_kart2, 0.75);

		sec_per_frame = start_frame.elapsed().as_secs_f64();
		//Output frames per second
		//println!("FPS: {}", 1.0 / sec_per_frame);	
	}

	Ok(())
}
