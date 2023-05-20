#![windows_subsystem = "windows"]
extern crate sdl2;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::{Point, Rect};
use sdl2::render::BlendMode;

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
    let window = vid_subsystem
        .window("Krab Kart", 960, 540)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    const WIDTH: usize = 320;
    const HEIGHT: usize = 180;

    //Create canvas
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .accelerated()
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

    let mut cam1 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 0.4, 3.14159 / 3.0);
    let mut cam2 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 0.4, 3.14159 / 3.0);

    let mut player_kart1 = sprite::Sprite::new(9.0, 35.2);
    player_kart1.frame_count = 8;
    player_kart1.rotation = 3.14159 / 2.0;

    let mut player_kart2 = sprite::Sprite::new(9.0, 35.0);
    player_kart2.rotation = 3.14159 / 2.0;
    player_kart2.frame_count = 8;

    let krab_texture1 = sprite::load_texture(String::from("res/kart1.png"), &texture_creator)
        .map_err(|e| e.to_string())?;
    player_kart1.set_texture(Some(&krab_texture1));
    let krab_texture2 = sprite::load_texture(String::from("res/kart2.png"), &texture_creator)
        .map_err(|e| e.to_string())?;
    player_kart2.set_texture(Some(&krab_texture2));
    let mut level =
        level::Level::load_from_png(String::from("res/level.png")).map_err(|e| e.to_string())?;
    level.level_scale = 32.0;

    let font_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = font_ctx
        .load_font("res/fonts/8BitOperator/8bitOperatorPlus-Regular.ttf", 32)
        .map_err(|e| e.to_string())?;

    while !events.can_quit {
        let start_frame = Instant::now();

        canvas.clear();
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        let sz = pixel_buffer.len() / 2;
        level.display_level(&mut pixel_buffer[0..sz], WIDTH, HEIGHT / 2, &cam1);
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

        player_kart1.camera_kart = true;
        {
            let mut sprites_to_draw: Vec<&mut sprite::Sprite> = vec![];
            sprites_to_draw.push(&mut player_kart1);
            sprites_to_draw.push(&mut player_kart2);
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
        player_kart1.camera_kart = false;

        let origin_y = texture_rect.y() / 2;

        level.display_level(&mut pixel_buffer[sz..], WIDTH, HEIGHT / 2, &cam2);
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

        player_kart2.camera_kart = true;
        {
            let mut sprites_to_draw: Vec<&mut sprite::Sprite> = vec![];
            sprites_to_draw.push(&mut player_kart1);
            sprites_to_draw.push(&mut player_kart2);
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
        player_kart2.camera_kart = false;

        display::display_text_left_justify(
            &mut canvas,
            &texture_creator,
            16,
            16,
            &font,
            format!(
                "speed: {}",
                ((player_kart1.speed * 100.0 * 20.0).round() / 100.0)
            ),
            Color::WHITE,
            16,
        )
        .map_err(|e| e.to_string())?;

        display::display_text_left_justify(
            &mut canvas,
            &texture_creator,
            16,
            16 + canvas_dimensions_half.1 as i32,
            &font,
            format!(
                "speed: {}",
                ((player_kart2.speed * 100.0 * 20.0).round() / 100.0)
            ),
            Color::WHITE,
            16,
        )
        .map_err(|e| e.to_string())?;

        display::display_text_right_justify(
            &mut canvas,
            &texture_creator,
            canvas_dimensions.0 as i32 - 16,
            16,
            &font,
            format!("FPS: {}", ((1.0 / sec_per_frame).round())),
            Color::WHITE,
            16,
        )
        .map_err(|e| e.to_string())?;

        canvas.present();
        events.update();

        //Move the karts
        player_kart1.drive_kart(&events, Keycode::Up, Keycode::Left, Keycode::Right);
        player_kart2.drive_kart(&events, Keycode::W, Keycode::A, Keycode::D);

        player_kart1.move_sprite(sec_per_frame);
        player_kart2.move_sprite(sec_per_frame);

        player_kart1.apply_friction(&level);
        player_kart2.apply_friction(&level);

        cam1.follow(&player_kart1, 0.75);
        cam2.follow(&player_kart2, 0.75);

        sec_per_frame = start_frame.elapsed().as_secs_f64();
    }

    Ok(())
}
