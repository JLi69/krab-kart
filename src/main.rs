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
            sprite::BitMap::from_png("assets/icon.png").map_err(|e| e.to_string())?;
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

    let mut cam1 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 1.2, 3.14159 / 2.0);
    let mut cam2 = level::Camera::new(0.0, 0.0, 0.0, 0.01, 1.2, 3.14159 / 2.0);

    let mut player_kart1 = sprite::Sprite::new(9.0, 35.5);
    player_kart1.width = 0.1;
    player_kart1.height = 0.1;
    player_kart1.frame_count = 8;
    player_kart1.rotation = 3.14159 / 2.0;

    let mut player_kart2 = sprite::Sprite::new(9.0, 35.1);
    player_kart2.width = 0.1;
    player_kart2.height = 0.1;
    player_kart2.rotation = 3.14159 / 2.0;
    player_kart2.frame_count = 8;

    let krab_texture1 = sprite::load_texture("assets/images/kart1.png", &texture_creator)
        .map_err(|e| e.to_string())?;
    player_kart1.set_texture(Some(&krab_texture1));
    let krab_texture2 = sprite::load_texture("assets/images/kart2.png", &texture_creator)
        .map_err(|e| e.to_string())?;
    player_kart2.set_texture(Some(&krab_texture2));

    let checkpoint1_texture =
        sprite::load_texture("assets/images/kart-checkpoint1.png", &texture_creator)
            .map_err(|e| e.to_string())?;
    let checkpoint2_texture =
        sprite::load_texture("assets/images/kart-checkpoint2.png", &texture_creator)
            .map_err(|e| e.to_string())?;

    let mut level = level::Level::load_from_png("assets/level.png").map_err(|e| e.to_string())?;
    level.level_scale = 32.0;
    level.checkpoints = vec![
        (11.0, 35.3),
        (34.0, 28.0),
        (32.0, 7.0),
        (13.0, 6.0),
        (11.0, 35.3),
    ];

    let mut checkpoint1 = sprite::Sprite::new(level.checkpoints[0].0, level.checkpoints[0].1);
    checkpoint1.set_texture(Some(&checkpoint1_texture));
    checkpoint1.width = 0.15;
    checkpoint1.height = 0.15;
    let mut checkpoint2 = sprite::Sprite::new(level.checkpoints[0].0, level.checkpoints[0].1);
    checkpoint2.set_texture(Some(&checkpoint2_texture));
    checkpoint2.width = 0.15;
    checkpoint2.height = 0.15;

    //BGRA
    let mut track_textures = HashMap::<u32, sprite::BitMap>::new();
    track_textures.insert(
        0x707070ff,
        sprite::BitMap::from_png("assets/images/road.png").map_err(|e| e.to_string())?,
    );
    track_textures.insert(
        0x00ff00ff,
        sprite::BitMap::from_png("assets/images/grass.png").map_err(|e| e.to_string())?,
    );
    track_textures.insert(
        0x00ffffff,
        sprite::BitMap::from_png("assets/images/speedboost.png").map_err(|e| e.to_string())?,
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

    while !events.can_quit {
        let start_frame = Instant::now();

        canvas.clear();
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        let sz = pixel_buffer.len() / 2;
        level.display_level(
            &mut pixel_buffer[0..sz],
            WIDTH,
            HEIGHT / 2,
            &cam1,
            &track_textures,
        );
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
            sprites_to_draw.push(&mut checkpoint1);

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

        level.display_level(
            &mut pixel_buffer[sz..],
            WIDTH,
            HEIGHT / 2,
            &cam2,
            &track_textures,
        );
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
            sprites_to_draw.push(&mut checkpoint2);

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

        display::display_player_info(
            &mut canvas,
            &texture_creator,
            &font,
            &player_kart1,
            kart1_laps,
            0,
            0,
        )
        .map_err(|e| e.to_string())?;

        display::display_player_info(
            &mut canvas,
            &texture_creator,
            &font,
            &player_kart2,
            kart2_laps,
            0,
            canvas_dimensions_half.1 as i32,
        )
        .map_err(|e| e.to_string())?;

        display::display_text_right_justify(
            &mut canvas,
            &texture_creator,
            canvas_dimensions.0 as i32 - 16,
            16,
            &font,
            format!("FPS: {}", (fps.round())),
            Color::WHITE,
            8,
        )
        .map_err(|e| e.to_string())?;

        canvas.present();
        events.update();

        //Move the karts
        player_kart1.drive_kart(&events, Keycode::Up, Keycode::Left, Keycode::Right);
        player_kart2.drive_kart(&events, Keycode::W, Keycode::A, Keycode::D);

        player_kart1.move_sprite(sec_per_frame);
        player_kart2.move_sprite(sec_per_frame);

        if level.at_checkpoint(
            player_kart1.trans_x,
            player_kart1.trans_z,
            current_checkpoint_kart1,
            1.5,
        ) {
            if current_checkpoint_kart1 == level.checkpoints.len() - 1 {
                kart1_laps += 1;
            }

            current_checkpoint_kart1 += 1;
            current_checkpoint_kart1 %= level.checkpoints.len();
            checkpoint1.trans_x = level.checkpoints[current_checkpoint_kart1].0;
            checkpoint1.trans_z = level.checkpoints[current_checkpoint_kart1].1;
        }

        if level.at_checkpoint(
            player_kart2.trans_x,
            player_kart2.trans_z,
            current_checkpoint_kart2,
            1.5,
        ) {
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

        cam1.follow(&player_kart1, 1.1);
        cam2.follow(&player_kart2, 1.1);

        //Update FPS counter
        fps_update_timer += sec_per_frame;
        if fps_update_timer > 1.0 {
            fps = 1.0 / sec_per_frame;
            fps_update_timer = 0.0;
        }

        sec_per_frame = start_frame.elapsed().as_secs_f64();
    }

    Ok(())
}
