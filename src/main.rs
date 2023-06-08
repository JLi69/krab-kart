#![windows_subsystem = "windows"]
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::BlendMode;
use sdl2::surface::Surface;

use std::collections::HashMap;
use std::time::Instant;

mod display;
mod events;
mod level;
mod sprite;

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    let vid_subsystem = sdl_context.video().map_err(|e| e.to_string())?;
    //Create window
    let mut window = vid_subsystem
        .window("Krab Kart", 960, 540)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;
    {
        let mut icon_pixels =
            sprite::bitmap::BitMap::from_png("assets/icon.png").map_err(|e| e.to_string())?;
        let w = icon_pixels.width();
        let h = icon_pixels.height();
        let mut pixels = icon_pixels.pixels.as_mut_slice();
        let icon = Surface::from_data(&mut pixels, w, h, w * 3, PixelFormatEnum::RGB24)
            .map_err(|e| e.to_string())?;
        window.set_icon(icon);
    }

    const WIDTH: usize = 480;
    const HEIGHT: usize = 270;

    //Create canvas
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH as u32, HEIGHT as u32 / 2)
        .map_err(|e| e.to_string())?;
    texture.set_blend_mode(BlendMode::Blend);
    let mut pixel_buffer: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];

    let mut sec_per_frame: f64 = 0.0;
    let mut events = events::Events::new(&sdl_context).map_err(|e| e.to_string())?;

    let mut cam1 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 1.0, 3.14159 / 2.0);
    let mut cam2 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 1.0, 3.14159 / 2.0);

    let sprite_assets =
        sprite::load_assets::load_assets(&texture_creator).map_err(|e| e.to_string())?;
	let powerup_icons =
        sprite::load_assets::load_powerup_icon_assets(&texture_creator).map_err(|e| e.to_string())?;

    let mut player_kart1 =
        sprite::kart::Kart::new(9.0, 35.5, sprite::SpriteType::Kart1, &sprite_assets);
    player_kart1.sprite.width = 0.1;
    player_kart1.sprite.height = 0.1;
    player_kart1.sprite.frame_count = 8;
    player_kart1.sprite.rotation = 3.14159 / 2.0;

    let mut player_kart2 =
        sprite::kart::Kart::new(9.0, 35.1, sprite::SpriteType::Kart2, &sprite_assets);
    player_kart2.sprite.width = 0.1;
    player_kart2.sprite.height = 0.1;
    player_kart2.sprite.rotation = 3.14159 / 2.0;
    player_kart2.sprite.frame_count = 8;

    cam1.follow(&player_kart1.sprite, 1.1);
    cam2.follow(&player_kart2.sprite, 1.1);

    let mut level = level::Level::load_from_png("assets/level.png").map_err(|e| e.to_string())?;
    level.level_scale = 32.0;
    level.checkpoints = vec![
        (11.0, 35.3),
        (34.0, 28.0),
        (32.0, 7.0),
        (13.0, 6.0),
        (11.0, 35.3),
    ];

    let mut checkpoint1 = sprite::Sprite::new(
        level.checkpoints[0].0,
        level.checkpoints[0].1,
        sprite::SpriteType::Checkpoint1,
        &sprite_assets,
    );
    checkpoint1.width = 0.1;
    checkpoint1.height = 0.1;
    let mut checkpoint2 = sprite::Sprite::new(
        level.checkpoints[0].0,
        level.checkpoints[0].1,
        sprite::SpriteType::Checkpoint2,
        &sprite_assets,
    );
    checkpoint2.width = 0.1;
    checkpoint2.height = 0.1;

    //BGRA
    let mut track_textures = HashMap::<u32, sprite::bitmap::BitMap>::new();
    track_textures.insert(
        0x707070ff,
        sprite::bitmap::BitMap::from_png("assets/images/road.png").map_err(|e| e.to_string())?,
    );
    track_textures.insert(
        0x00ff00ff,
        sprite::bitmap::BitMap::from_png("assets/images/grass.png").map_err(|e| e.to_string())?,
    );
    track_textures.insert(
        0x00ffffff,
        sprite::bitmap::BitMap::from_png("assets/images/speedboost.png")
            .map_err(|e| e.to_string())?,
    );

    let mut current_checkpoint_kart1 = 0usize;
    let mut current_checkpoint_kart2 = 0usize;
    let mut kart1_laps = 0u32;
    let mut kart2_laps = 0u32;

    let font_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = font_ctx
        .load_font("assets/fonts/8BitOperator/8bitOperatorPlus-Regular.ttf", 32)
        .map_err(|e| e.to_string())?;

    let mut fps_update_timer = 0.0;
    let mut fps = 0.0f64;
    let mut frames = 0u32;

    player_kart1.move_kart(0.0);
    player_kart2.move_kart(0.0);

    //Add power ups
    let mut powerups = Vec::<sprite::powerup::Powerup>::new();
    {
        let power_up_locations = vec![
            (20.0, 35.2),
            (20.0, 36.0),
            (20.0, 34.4),
            (29.5, 22.5),
            (29.0, 22.0),
            (28.5, 21.5),
            (17.0, 5.0),
            (16.5, 4.5),
            (17.5, 4.5),
            (16.5, 5.5),
            (17.5, 5.5),
        ];

        for location in power_up_locations {
            let powerup = sprite::powerup::Powerup::new(location.0, location.1, &sprite_assets);
            powerups.push(powerup);
        }
    }

    //Add enemies
    let mut enemies = Vec::<sprite::enemy::Enemy>::new();
    {
        let enemy_locations = vec![
            (31.0, 18.0),
            (22.0, 4.0),
            (22.5, 7.0),
            (23.0, 4.0),
            (10.5, 17.0),
            (7.5, 15.5),
            (10.5, 16.0),
        ];

        let enemy_goal_pos = vec![
            (31.0, 16.0),
            (22.0, 7.0),
            (22.5, 4.0),
            (23.0, 7.0),
            (7.5, 16.0),
            (10.5, 16.5),
            (7.5, 15.0),
        ];

        for i in 0..enemy_locations.len() {
            let enemy = sprite::enemy::Enemy::new(
                enemy_locations[i].0,
                enemy_locations[i].1,
                enemy_goal_pos[i].0,
                enemy_goal_pos[i].1,
                &sprite_assets,
            );
            enemies.push(enemy);
        }
    }

    {
        let sz = pixel_buffer.len() / 2;

        level.display_level(
            &mut pixel_buffer[0..sz],
            WIDTH,
            HEIGHT / 2,
            &cam1,
            &track_textures,
        );

        level.display_level(
            &mut pixel_buffer[sz..],
            WIDTH,
            HEIGHT / 2,
            &cam2,
            &track_textures,
        );
    }

	let mut bananas = Vec::<sprite::Sprite>::new();
	let mut fireballs = Vec::<sprite::enemy::Fireball>::new();

    let mut start_timer = 3.0f64;

    while !events.can_quit {
        let start_frame = Instant::now();

        canvas.clear();
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        let sz = pixel_buffer.len() / 2;

        if player_kart1.moving() {
            level.display_level(
                &mut pixel_buffer[0..sz],
                WIDTH,
                HEIGHT / 2,
                &cam1,
                &track_textures,
            );
        }

        texture
            .update(None, &pixel_buffer[0..sz], WIDTH * 4)
            .map_err(|e| e.to_string())?;
        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );
        canvas
            .copy(&texture, None, texture_rect)
            .map_err(|e| e.to_string())?;

        player_kart1.sprite.camera_kart = player_kart1.knock_out <= 0.0;
        {
            let mut sprites_to_draw: Vec<&mut sprite::Sprite> = vec![];
            sprites_to_draw.push(&mut player_kart1.sprite);
            sprites_to_draw.push(&mut player_kart2.sprite);
            sprites_to_draw.push(&mut checkpoint1);

            for powerup in &mut powerups {
                sprites_to_draw.push(&mut powerup.sprite);
            }

            for enemy in &mut enemies {
                sprites_to_draw.push(&mut enemy.sprite);
            }

			for fireball in &mut fireballs {
                sprites_to_draw.push(&mut fireball.sprite);
            }

			for banana in &mut bananas {
				sprites_to_draw.push(banana);	
			}

            display::display_sprites(
                &mut canvas,
                &cam1,
                &mut sprites_to_draw,
                &canvas_dimensions_half,
                texture_rect.y() / 2,
                &(0, 0),
                WIDTH,
                HEIGHT / 2,
            )
            .map_err(|e| e.to_string())?;
        }
        player_kart1.sprite.camera_kart = false;

        let origin_y = texture_rect.y() / 2;

        if player_kart2.moving() {
            level.display_level(
                &mut pixel_buffer[sz..],
                WIDTH,
                HEIGHT / 2,
                &cam2,
                &track_textures,
            );
        }
        texture
            .update(None, &pixel_buffer[sz..], WIDTH * 4)
            .map_err(|e| e.to_string())?;
        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4 * 3,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );
        canvas
            .copy(&texture, None, texture_rect)
            .map_err(|e| e.to_string())?;

        player_kart2.sprite.camera_kart = player_kart2.knock_out <= 0.0;
        {
            let mut sprites_to_draw: Vec<&mut sprite::Sprite> = vec![];
            sprites_to_draw.push(&mut player_kart1.sprite);
            sprites_to_draw.push(&mut player_kart2.sprite);
            sprites_to_draw.push(&mut checkpoint2);

            for powerup in &mut powerups {
                sprites_to_draw.push(&mut powerup.sprite);
            }

            for enemy in &mut enemies {
                sprites_to_draw.push(&mut enemy.sprite);
            }

			for fireball in &mut fireballs {
                sprites_to_draw.push(&mut fireball.sprite);
            }

			for banana in &mut bananas {
				sprites_to_draw.push(banana);	
			}

            display::display_sprites(
                &mut canvas,
                &cam2,
                &mut sprites_to_draw,
                &canvas_dimensions_half,
                origin_y + texture_rect.height() as i32 - canvas_dimensions_half.1 as i32,
                &(0, canvas_dimensions_half.1 as i32),
                WIDTH,
                HEIGHT / 2,
            )
            .map_err(|e| e.to_string())?;
        }
        player_kart2.sprite.camera_kart = false;

        display::display_player_info(
            &mut canvas,
            &texture_creator,
            &font,
            &player_kart1.sprite,
            kart1_laps,
            0,
            0,
        )
        .map_err(|e| e.to_string())?;

		display::display_powerup_icons(
			&mut canvas,
			&powerup_icons,
			48,
			&player_kart1,
			(canvas_dimensions.0 / 2) as i32,
			16
		)
		.map_err(|e| e.to_string())?;

        display::display_player_info(
            &mut canvas,
            &texture_creator,
            &font,
            &player_kart2.sprite,
            kart2_laps,
            0,
            canvas_dimensions_half.1 as i32,
        )
        .map_err(|e| e.to_string())?;
		
		display::display_powerup_icons(
			&mut canvas,
			&powerup_icons,
			48,
			&player_kart2,
			(canvas_dimensions.0 / 2) as i32,
			canvas_dimensions_half.1 as i32 + 16
		)
		.map_err(|e| e.to_string())?;

        display::display_text_right_justify(
            &mut canvas,
            &texture_creator,
            canvas_dimensions.0 as i32 - 16,
            16,
            &font,
            format!("FPS: {}", fps.round()),
            Color::WHITE,
            8,
        )
        .map_err(|e| e.to_string())?;

        if start_timer.ceil() > 0.0 {
            display::display_text_center(
                &mut canvas,
                &texture_creator,
                canvas_dimensions.0 as i32 / 2,
                canvas_dimensions.1 as i32 / 2 - 64,
                &font,
                format!("{}", start_timer.ceil()),
                Color::WHITE,
                64,
            )
            .map_err(|e| e.to_string())?;
        } else if start_timer.ceil() == 0.0 {
            display::display_text_center(
                &mut canvas,
                &texture_creator,
                canvas_dimensions.0 as i32 / 2,
                canvas_dimensions.1 as i32 / 2 - 64,
                &font,
                String::from("GO!"),
                Color::WHITE,
                64,
            )
            .map_err(|e| e.to_string())?;
        }

        //Victory at 4 laps
        if kart1_laps == 4 {
            display::display_text_center(
                &mut canvas,
                &texture_creator,
                canvas_dimensions.0 as i32 / 2,
                canvas_dimensions.1 as i32 / 2 - 32,
                &font,
                String::from("PLAYER 1 WINS!"),
                Color::RED,
                32,
            )
            .map_err(|e| e.to_string())?;
            player_kart1.sprite.speed = 0.0;
            player_kart1.sprite.rotation_speed = 0.0;
            player_kart2.sprite.speed = 0.0;
            player_kart2.sprite.rotation_speed = 0.0;
        } else if kart2_laps == 4 {
            display::display_text_center(
                &mut canvas,
                &texture_creator,
                canvas_dimensions.0 as i32 / 2,
                canvas_dimensions.1 as i32 / 2 - 32,
                &font,
                String::from("PLAYER 2 WINS!"),
                Color::BLUE,
                32,
            )
            .map_err(|e| e.to_string())?;
            player_kart1.sprite.speed = 0.0;
            player_kart1.sprite.rotation_speed = 0.0;
            player_kart2.sprite.speed = 0.0;
            player_kart2.sprite.rotation_speed = 0.0;
        }
	
		//Check for player collision with bananas
		let mut index = 0;
		while index < bananas.len() {	
			let banana = &bananas[index];		
			index += 1;	

			if sprite::dist_between(banana, &player_kart1.sprite) < 0.1 {
				player_kart1.knock_out = 3.0;	
				bananas.remove(index - 1);
				index -= 1;	
				continue;
			}
			
			if sprite::dist_between(banana, &player_kart2.sprite) < 0.1 {	
				player_kart2.knock_out = 3.0;
				bananas.remove(index - 1);
				index -= 1;
				continue;
			}
		}
		
		//Update the fireballs
		let mut index = 0;
		while index < fireballs.len() {	
			let fireball = &mut fireballs[index];
			match fireball.target {
				sprite::SpriteType::Kart1 => {		
					fireball.update(sec_per_frame, &player_kart1.sprite);
				},
				sprite::SpriteType::Kart2 => {	
					fireball.update(sec_per_frame, &player_kart2.sprite);
				}
				_ => {}
			}
			index += 1;	

			if sprite::dist_between(&fireball.sprite, &player_kart1.sprite) < 0.1 {
				player_kart1.knock_out = 2.0;
				fireballs.remove(index - 1);
				index -= 1;	
				continue;
			}
			
			if sprite::dist_between(&fireball.sprite, &player_kart2.sprite) < 0.1 {	
				player_kart2.knock_out = 2.0;
				fireballs.remove(index - 1);
				index -= 1;
				continue;
			}

			if fireball.timer > 8.0 {
				fireballs.remove(index - 1);
				index -= 1;
				continue;
			}
		}

        //Move the karts
        if start_timer <= 0.0 && kart1_laps < 4 && kart2_laps < 4 {
            player_kart1.drive_kart(&events, Keycode::Up, Keycode::Left, Keycode::Right);
            player_kart2.drive_kart(&events, Keycode::W, Keycode::A, Keycode::D);
            player_kart1.move_kart(sec_per_frame);
            player_kart2.move_kart(sec_per_frame);
        
			//Use powerups
			if events.key_is_pressed_once(Keycode::Down) {
				let powerup = player_kart1.use_powerup();	
				match powerup {
					sprite::kart::PowerupType::SpeedBoost => {
						player_kart1.sprite.speed += 1.0;	
					},
					sprite::kart::PowerupType::Banana => {
						let mut banana = sprite::Sprite::new(
							player_kart1.sprite.trans_x - 0.3 * player_kart1.sprite.rotation.sin(),
							player_kart1.sprite.trans_z - 0.3 * player_kart1.sprite.rotation.cos(),
							sprite::SpriteType::Banana,
							&sprite_assets
						);
						banana.width = 0.06;
						banana.height = 0.06;
						bananas.push(banana);	
					},
					sprite::kart::PowerupType::Fireball => {
						let (xdiff, ydiff) = 
							sprite::xz_diff_norm(&player_kart2.sprite, &player_kart1.sprite);
						fireballs.push(sprite::enemy::Fireball::new(
							player_kart1.sprite.trans_x + 0.3 * xdiff,
							player_kart1.sprite.trans_z + 0.3 * ydiff,
							&sprite_assets,
							sprite::SpriteType::Kart2,
						));	
					},
					_ => {}
				}
			}
			if events.key_is_pressed_once(Keycode::S) {
				let powerup = player_kart2.use_powerup();	
				match powerup {
					sprite::kart::PowerupType::SpeedBoost => {
						player_kart2.sprite.speed += 1.0;	
					},
					sprite::kart::PowerupType::Banana => {
						let mut banana = sprite::Sprite::new(
							player_kart2.sprite.trans_x - 0.3 * player_kart2.sprite.rotation.sin(),
							player_kart2.sprite.trans_z - 0.3 * player_kart2.sprite.rotation.cos(),
							sprite::SpriteType::Banana,
							&sprite_assets
						);
						banana.width = 0.06;
						banana.height = 0.06;
						bananas.push(banana);
					},
					sprite::kart::PowerupType::Fireball => {
						let (xdiff, ydiff) = 
							sprite::xz_diff_norm(&player_kart1.sprite, &player_kart2.sprite);
						fireballs.push(sprite::enemy::Fireball::new(
							player_kart2.sprite.trans_x + 0.3 * xdiff,
							player_kart2.sprite.trans_z + 0.3 * ydiff,
							&sprite_assets,
							sprite::SpriteType::Kart1,
						));	
					},
					_ => {}
				}	
			}
		}

        if level.kart_at_checkpoint(&player_kart1, current_checkpoint_kart1, 1.5) {
            if current_checkpoint_kart1 == level.checkpoints.len() - 1 {
                kart1_laps += 1;
            }

            current_checkpoint_kart1 += 1;
            current_checkpoint_kart1 %= level.checkpoints.len();
            checkpoint1.trans_x = level.checkpoints[current_checkpoint_kart1].0;
            checkpoint1.trans_z = level.checkpoints[current_checkpoint_kart1].1;
        }

        if level.kart_at_checkpoint(&player_kart2, current_checkpoint_kart2, 1.5) {
            if current_checkpoint_kart2 == level.checkpoints.len() - 1 {
                kart2_laps += 1;
            }

            current_checkpoint_kart2 += 1;
            current_checkpoint_kart2 %= level.checkpoints.len();
            checkpoint2.trans_x = level.checkpoints[current_checkpoint_kart2].0;
            checkpoint2.trans_z = level.checkpoints[current_checkpoint_kart2].1;
        }

        player_kart1.apply_friction(&level);
        player_kart2.apply_friction(&level);

        if player_kart1.knock_out <= 0.0 {
            cam1.follow(&player_kart1.sprite, 1.1);
        }
        if player_kart2.knock_out <= 0.0 {
            cam2.follow(&player_kart2.sprite, 1.1);
        }

        for enemy in &mut enemies {
            if sprite::dist_between(&enemy.sprite, &player_kart1.sprite) < 0.2
                && player_kart1.knock_out <= 0.0
            {
                player_kart1.knock_out = 1.0;
            }
            if sprite::dist_between(&enemy.sprite, &player_kart2.sprite) < 0.2
                && player_kart2.knock_out <= 0.0
            {
                player_kart2.knock_out = 1.0;
            }

            enemy.update(sec_per_frame);
        }

        for powerup in &mut powerups {
            if sprite::dist_between(&powerup.sprite, &player_kart1.sprite) < 0.2
                && powerup.sprite.width >= powerup.size
				&& player_kart1.powerup_amt == 0
            {
                powerup.sprite.width = 0.0;
                powerup.sprite.height = 0.0;
				player_kart1.pickup_powerup();
            }
            if sprite::dist_between(&powerup.sprite, &player_kart2.sprite) < 0.2
                && powerup.sprite.width >= powerup.size
				&& powerup.sprite.width >= powerup.size
				&& player_kart2.powerup_amt == 0
            {
                powerup.sprite.width = 0.0;
                powerup.sprite.height = 0.0;
				player_kart2.pickup_powerup();
			}

            powerup.update(sec_per_frame);
        }

        events.update();
        canvas.present();

        frames += 1;
        //Update FPS counter
        fps_update_timer += sec_per_frame;
        if fps_update_timer >= 1.0 {
            fps = frames as f64 - 1.0;
            fps_update_timer = 0.0;
            frames = 0;
        }

        if start_timer > -1.0 {
            start_timer -= sec_per_frame;
        }

        sec_per_frame = start_frame.elapsed().as_secs_f64();
    }

    Ok(())
}
