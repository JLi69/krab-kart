#![windows_subsystem = "windows"]
extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use sdl2::render::BlendMode;
use sdl2::surface::Surface;

use std::collections::HashMap;
use std::time::Instant;

/*
 * "Mama mia, that's a lotta spaghetti!"
 * */

mod display;
mod events;
mod level;
mod sprite;

mod menu;
mod oneplayer;
mod twoplayer;

enum GameScreen {
    MainMenu,
    OnePlayer,
    TwoPlayer,
}

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
    let powerup_icons = sprite::load_assets::load_powerup_icon_assets(&texture_creator)
        .map_err(|e| e.to_string())?;

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

    //Add enemies
    let mut enemies = Vec::<sprite::enemy::Enemy>::new();

    let mut bananas = Vec::<sprite::Sprite>::new();
    let mut fireballs = Vec::<sprite::enemy::Fireball>::new();

    let mut start_timer = 3.0f64;

    let mut screen = GameScreen::MainMenu;

    //buttons
    let oneplayer_button = menu::Button::new(0, -32, "One Player");
    let twoplayer_button = menu::Button::new(0, 16, "Two Player");
    let quit_button = menu::Button::new(0, 64, "QUIT");

    let goto_menu = menu::Button::new(0, 0, "Main Menu");
    let goto_game = menu::Button::new(0, 48, "Return to Game");

    let mut paused = false;
    let mut timer = 0.0f64; //Timer for single player mode

    while !events.can_quit {
        let start_frame = Instant::now();

        canvas.set_draw_color(Color::RGB(32, 128, 255));
        canvas.clear();
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        match screen {
            GameScreen::MainMenu => {
                cam1.rotation += sec_per_frame * 0.2;

                //Background
                level.display_level(
                    &mut pixel_buffer[0..],
                    WIDTH,
                    HEIGHT / 2,
                    &cam1,
                    &track_textures,
                );

                texture
                    .update(None, &pixel_buffer[0..], WIDTH * 4)
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

                display::display_text_center(
                    &mut canvas,
                    &texture_creator,
                    (canvas_dimensions.0 / 2) as i32,
                    (canvas_dimensions.1 / 4) as i32 - 64,
                    &font,
                    String::from("Krab Kart"),
                    Color::WHITE,
                    64,
                )
                .map_err(|e| e.to_string())?;

                oneplayer_button
                    .display(
                        &mut canvas,
                        &canvas_dimensions,
                        &texture_creator,
                        &events,
                        &font,
                    )
                    .map_err(|e| e.to_string())?;
                twoplayer_button
                    .display(
                        &mut canvas,
                        &canvas_dimensions,
                        &texture_creator,
                        &events,
                        &font,
                    )
                    .map_err(|e| e.to_string())?;
                quit_button
                    .display(
                        &mut canvas,
                        &canvas_dimensions,
                        &texture_creator,
                        &events,
                        &font,
                    )
                    .map_err(|e| e.to_string())?;

                if quit_button.clicked(&mut events, &canvas_dimensions) {
                    events.can_quit = true;
                } else if oneplayer_button.clicked(&mut events, &canvas_dimensions) {
                    timer = 0.0;
					start_timer = 3.0;
                    screen = GameScreen::OnePlayer;
                    oneplayer::init(
                        &mut player_kart1,
                        &mut cam1,
                        &mut enemies,
                        &sprite_assets,
                        &mut checkpoint1,
                        &level.checkpoints[0],
                    );

                    level.display_level(
                        &mut pixel_buffer[0..],
                        WIDTH,
                        HEIGHT / 2,
                        &cam1,
                        &track_textures,
                    );
                } else if twoplayer_button.clicked(&mut events, &canvas_dimensions) {
                    start_timer = 3.0;
                    screen = GameScreen::TwoPlayer;
                    twoplayer::init(
                        &mut player_kart1,
                        &mut player_kart2,
                        &mut cam1,
                        &mut cam2,
                        &mut powerups,
                        &mut enemies,
                        &mut bananas,
                        &mut fireballs,
                        &sprite_assets,
                        &mut checkpoint1,
                        &mut checkpoint2,
                        &level.checkpoints[0],
                    );

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
            }
            GameScreen::OnePlayer => {
                //Display Level
                if player_kart1.moving() {
                    level.display_level(
                        &mut pixel_buffer[0..],
                        WIDTH,
                        HEIGHT / 2,
                        &cam1,
                        &track_textures,
                    );
                }

                let texture_rect = Rect::from_center(
                    Point::new(
                        canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                        canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4 * 3,
                    ),
                    canvas_texture_rect.width(),
                    canvas_texture_rect.height() / 2,
                );

                texture
                    .update(None, &pixel_buffer[0..], WIDTH * 4)
                    .map_err(|e| e.to_string())?;
                canvas
                    .copy(&texture, None, texture_rect)
                    .map_err(|e| e.to_string())?;

                display::display_start_timer(
                    &mut canvas,
                    &texture_creator,
                    &canvas_dimensions,
                    &font,
                    start_timer,
                )
                .map_err(|e| e.to_string())?;

                display::display_player_info(
                    &mut canvas,
                    &texture_creator,
                    &font,
                    &player_kart1,
                    0,
                    0,
                )
                .map_err(|e| e.to_string())?;

                //Print DONE when player reaches 4 laps
                if player_kart1.laps >= 4 {
                    display::display_text_center(
                        &mut canvas,
                        &texture_creator,
                        canvas_dimensions.0 as i32 / 2,
                        canvas_dimensions.1 as i32 / 2 - 32,
                        &font,
                        String::from("DONE!"),
                        Color::WHITE,
                        32,
                    )
                    .map_err(|e| e.to_string())?;
                }

                if player_kart1.laps < 4 && start_timer < 0.0 && !paused {
                    timer += sec_per_frame;
                }

                //Display timer
                if timer - 60.0 * (timer / 60.0).floor() < 10.0 {
                    display::display_text_center(
                        &mut canvas,
                        &texture_creator,
                        canvas_dimensions.0 as i32 / 2,
                        canvas_dimensions.1 as i32 / 8 - 32,
                        &font,
                        format!(
                            "{}:0{}",
                            (timer / 60.0).floor(),
                            (timer - 60.0 * (timer / 60.0).floor()).round()
                        ),
                        Color::WHITE,
                        32,
                    )
                    .map_err(|e| e.to_string())?;
                } else {
                    display::display_text_center(
                        &mut canvas,
                        &texture_creator,
                        canvas_dimensions.0 as i32 / 2,
                        canvas_dimensions.1 as i32 / 8 - 32,
                        &font,
                        format!(
                            "{}:{}",
                            (timer / 60.0).floor(),
                            (timer - 60.0 * (timer / 60.0).floor()).round()
                        ),
                        Color::WHITE,
                        32,
                    )
                    .map_err(|e| e.to_string())?;
                }

                player_kart1.sprite.camera_kart = player_kart1.knock_out <= 0.0;
                {
                    let mut sprites_to_draw: Vec<&mut sprite::Sprite> = vec![];
                    sprites_to_draw.push(&mut player_kart1.sprite);
                    sprites_to_draw.push(&mut checkpoint1);

                    for enemy in &mut enemies {
                        sprites_to_draw.push(&mut enemy.sprite);
                    }

					let origin_y = texture_rect.y() / 2;
                    display::display_sprites(
                        &mut canvas,
                        &cam1,
                        &mut sprites_to_draw,
                        &canvas_dimensions_half,
                        origin_y + texture_rect.height() as i32 / 2 - canvas_dimensions_half.1 as i32,
                        &(0, canvas_dimensions_half.1 as i32),
                        WIDTH,
                        HEIGHT / 2,
                    )
                    .map_err(|e| e.to_string())?;
                }
                player_kart1.sprite.camera_kart = false;

                if paused {
                    canvas.set_draw_color(Color::RGBA(128, 128, 128, 200));
                    canvas
                        .fill_rect(Rect::new(0, 0, canvas_dimensions.0, canvas_dimensions.1))
                        .map_err(|e| e.to_string())?;

                    display::display_text_center(
                        &mut canvas,
                        &texture_creator,
                        canvas_dimensions.0 as i32 / 2,
                        canvas_dimensions.1 as i32 / 4 - 64,
                        &font,
                        String::from("PAUSED"),
                        Color::WHITE,
                        64,
                    )
                    .map_err(|e| e.to_string())?;

                    goto_menu
                        .display(
                            &mut canvas,
                            &canvas_dimensions,
                            &texture_creator,
                            &events,
                            &font,
                        )
                        .map_err(|e| e.to_string())?;
                    goto_game
                        .display(
                            &mut canvas,
                            &canvas_dimensions,
                            &texture_creator,
                            &events,
                            &font,
                        )
                        .map_err(|e| e.to_string())?;

                    if goto_game.clicked(&mut events, &canvas_dimensions) {
                        paused = false;
                    } else if goto_menu.clicked(&mut events, &canvas_dimensions) {
                        player_kart1 = sprite::kart::Kart::new(
                            9.0,
                            35.5,
                            sprite::SpriteType::Kart1,
                            &sprite_assets,
                        );
                        player_kart1.sprite.rotation = 3.14159 / 2.0;
                        cam1.follow(&player_kart1.sprite, 1.1);
                        paused = false;
                        screen = GameScreen::MainMenu;
                    }
                }

                if events.key_is_pressed_once(sdl2::keyboard::Keycode::Escape) {
                    paused = !paused;
                }

                //Update the player
                if !paused {
                    oneplayer::update_enemies(&mut enemies, &mut player_kart1, sec_per_frame);

                    //Move the karts
                    oneplayer::update_karts(
                        &mut player_kart1,
                        &mut checkpoint1,
                        &mut cam1,
                        &events,
                        &level,
                        sec_per_frame,
                        start_timer,
                    );

                    if start_timer > -1.0 {
                        start_timer -= sec_per_frame;
                    }
                }
            }
            GameScreen::TwoPlayer => {
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
                    &player_kart1,
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
                    16,
                )
                .map_err(|e| e.to_string())?;

                display::display_player_info(
                    &mut canvas,
                    &texture_creator,
                    &font,
                    &player_kart2,
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
                    canvas_dimensions_half.1 as i32 + 16,
                )
                .map_err(|e| e.to_string())?;

                display::display_start_timer(
                    &mut canvas,
                    &texture_creator,
                    &canvas_dimensions,
                    &font,
                    start_timer,
                )
                .map_err(|e| e.to_string())?;

                //Victory at 4 laps
                display::display_victory_twoplayer(
                    &mut canvas,
                    &player_kart1,
                    &player_kart2,
                    &texture_creator,
                    &canvas_dimensions,
                    &font,
                )
                .map_err(|e| e.to_string())?;

                if paused {
                    canvas.set_draw_color(Color::RGBA(128, 128, 128, 200));
                    canvas
                        .fill_rect(Rect::new(0, 0, canvas_dimensions.0, canvas_dimensions.1))
                        .map_err(|e| e.to_string())?;

                    display::display_text_center(
                        &mut canvas,
                        &texture_creator,
                        canvas_dimensions.0 as i32 / 2,
                        canvas_dimensions.1 as i32 / 4 - 64,
                        &font,
                        String::from("PAUSED"),
                        Color::WHITE,
                        64,
                    )
                    .map_err(|e| e.to_string())?;

                    goto_menu
                        .display(
                            &mut canvas,
                            &canvas_dimensions,
                            &texture_creator,
                            &events,
                            &font,
                        )
                        .map_err(|e| e.to_string())?;
                    goto_game
                        .display(
                            &mut canvas,
                            &canvas_dimensions,
                            &texture_creator,
                            &events,
                            &font,
                        )
                        .map_err(|e| e.to_string())?;

                    if goto_game.clicked(&mut events, &canvas_dimensions) {
                        paused = false;
                    } else if goto_menu.clicked(&mut events, &canvas_dimensions) {
                        player_kart1 = sprite::kart::Kart::new(
                            9.0,
                            35.5,
                            sprite::SpriteType::Kart1,
                            &sprite_assets,
                        );
                        player_kart1.sprite.rotation = 3.14159 / 2.0;
                        cam1.follow(&player_kart1.sprite, 1.1);
                        paused = false;
                        screen = GameScreen::MainMenu;
                    }
                }

                if events.key_is_pressed_once(sdl2::keyboard::Keycode::Escape) {
                    paused = !paused;
                }

                if !paused {
                    twoplayer::update_enemies(
                        &mut enemies,
                        &mut player_kart1,
                        &mut player_kart2,
                        sec_per_frame,
                    );

                    //Check for player collision with bananas
                    twoplayer::update_bananas(&mut bananas, &mut player_kart1, &mut player_kart2);

                    //Update the fireballs
                    twoplayer::update_fireballs(
                        &mut fireballs,
                        &mut player_kart1,
                        &mut player_kart2,
                        sec_per_frame,
                    );

                    if start_timer <= 0.0 && player_kart1.laps < 4 && player_kart2.laps < 4 {
                        //Use powerups
                        twoplayer::use_powerups(
                            &mut player_kart1,
                            &mut player_kart2,
                            &mut events,
                            &mut bananas,
                            &mut fireballs,
                            &sprite_assets,
                        )
                    }

                    //Move the karts
                    twoplayer::update_karts(
                        &mut player_kart1,
                        &mut player_kart2,
                        &mut checkpoint1,
                        &mut checkpoint2,
                        &mut cam1,
                        &mut cam2,
                        &events,
                        &level,
                        sec_per_frame,
                        start_timer,
                    );

                    twoplayer::update_powerups(
                        &mut powerups,
                        &mut player_kart1,
                        &mut player_kart2,
                        sec_per_frame,
                    );

                    if start_timer > -1.0 {
                        start_timer -= sec_per_frame;
                    }
                }
            }
        }

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

        sec_per_frame = start_frame.elapsed().as_secs_f64();
    }

    Ok(())
}
