#![windows_subsystem = "windows"]
extern crate sdl2;

use pausemenu::PauseMenu;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::BlendMode;
use sdl2::surface::Surface;
use sdl2::video::Window;
use sprite::load_assets::{load_assets, load_powerup_icon_assets};
use sprite::SpriteType;

use std::collections::HashMap;
use std::time::Instant;

mod display;
mod events;
mod level;
mod mainmenu;
mod menu;
mod oneplayer;
mod pausemenu;
mod sprite;
mod twoplayer;

use events::Events;
use level::Level;
use mainmenu::MainMenuScreen;
use oneplayer::SingeplayerState;
use sprite::bitmap::BitMap;
use twoplayer::TwoplayerState;

enum GameScreen {
    MainMenu,
    OnePlayer,
    TwoPlayer,
}

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 270;

fn create_window(sdl_context: &sdl2::Sdl) -> Result<Window, String> {
    let vid_subsystem = sdl_context.video().map_err(|e| e.to_string())?;
    let mut window = vid_subsystem
        .window("Krab Kart", 960, 540)
        .position_centered()
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut icon_pixels = BitMap::from_png("assets/icon.png").map_err(|e| e.to_string())?;
    let w = icon_pixels.width();
    let h = icon_pixels.height();
    let pixels = icon_pixels.pixels.as_mut_slice();
    let icon = Surface::from_data(pixels, w, h, w * 3, PixelFormatEnum::RGB24)
        .map_err(|e| e.to_string())?;
    window.set_icon(icon);
    Ok(window)
}

fn load_track_textures() -> Result<HashMap<u32, BitMap>, String> {
    //BGRA
    let mut track_textures = HashMap::<u32, BitMap>::new();
    track_textures.insert(
        0x707070ff,
        BitMap::from_png("assets/images/road.png").map_err(|e| e.to_string())?,
    );
    track_textures.insert(
        0x00ff00ff,
        BitMap::from_png("assets/images/grass.png").map_err(|e| e.to_string())?,
    );
    track_textures.insert(
        0x00ffffff,
        BitMap::from_png("assets/images/speedboost.png").map_err(|e| e.to_string())?,
    );
    Ok(track_textures)
}

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init().map_err(|e| e.to_string())?;
    //Create window
    let window = create_window(&sdl_context)?;

    //Create canvas
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;
    canvas.set_blend_mode(BlendMode::Blend);
    //Create texture
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::ARGB8888, WIDTH as u32, HEIGHT as u32 / 2)
        .map_err(|e| e.to_string())?;
    texture.set_blend_mode(BlendMode::Blend);
    let mut pixel_buffer: [u8; WIDTH * HEIGHT * 4] = [0; WIDTH * HEIGHT * 4];
    //Events
    let mut events = Events::new(&sdl_context)?;
    //Load fonts
    let font_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = font_ctx
        .load_font("assets/fonts/8BitOperator/8bitOperatorPlus-Regular.ttf", 32)
        .map_err(|e| e.to_string())?;
    //Load track textures
    let track_textures = load_track_textures()?;
    //Load other textures
    let sprite_assets = load_assets(&texture_creator)?;
    let powerup_assets = load_powerup_icon_assets(&texture_creator)?;
    //Load level
    let track = Level::load_from_png("assets/level.png")?;

    let mut fps_update_timer = 0.0;
    let mut fps = 0.0f64;
    let mut frames = 0u32;
    let mut sec_per_frame: f64 = 0.0;

    let mut screen = GameScreen::MainMenu;
    let mut main_menu = MainMenuScreen::init();
    let mut single_player_state = SingeplayerState::init();
    let mut two_player_state = TwoplayerState::init();

    //buttons
    let mut pause_menu = PauseMenu::new();

    while !events.can_quit {
        let start_frame = Instant::now();

        canvas.set_draw_color(Color::RGB(32, 128, 255));
        canvas.clear();
        let canvas_dimensions = canvas.output_size()?;

        match screen {
            GameScreen::MainMenu => {
                main_menu.update_camera(sec_per_frame);
                main_menu.create_background_texture(
                    &track,
                    &mut pixel_buffer,
                    &mut texture,
                    &track_textures,
                )?;
                main_menu.display(&mut canvas, &texture, &texture_creator, &events, &font)?;
                let selected_screen = main_menu.press_buttons(&mut events, &canvas_dimensions);

                if let Some(selected_screen) = selected_screen {
                    screen = selected_screen;
                    main_menu = MainMenuScreen::init();
                    single_player_state = SingeplayerState::init();
                    two_player_state = TwoplayerState::init();
                }
            }
            GameScreen::OnePlayer => {
                pause_menu.listen_for_escape(&mut events);

                single_player_state.create_background_texture(
                    &mut pixel_buffer,
                    &track,
                    &track_textures,
                    &mut texture,
                )?;
                single_player_state.display(&mut canvas, &texture)?;
                single_player_state.display_sprites(&mut canvas, &sprite_assets)?;
                single_player_state.display_hud(&mut canvas, &texture_creator, &font)?;

                if !pause_menu.paused {
                    single_player_state.update(&events, &track, sec_per_frame);
                }
            }
            GameScreen::TwoPlayer => {
                pause_menu.listen_for_escape(&mut events);

                let sz = pixel_buffer.len() / 2;
                let canvas_dimensions = canvas.output_size()?;
                let canvas_texture_rect =
                    display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);
                two_player_state.create_background_texture(
                    &mut pixel_buffer,
                    &track,
                    &track_textures,
                );
                two_player_state.display_background(
                    &mut canvas,
                    &pixel_buffer[..sz],
                    &mut texture,
                    0,
                )?;
                two_player_state.display_sprites(&mut canvas, &sprite_assets, SpriteType::Kart1)?;
                two_player_state.display_background(
                    &mut canvas,
                    &pixel_buffer[sz..],
                    &mut texture,
                    canvas_texture_rect.height() / 2,
                )?;
                two_player_state.display_sprites(&mut canvas, &sprite_assets, SpriteType::Kart2)?;
                two_player_state.display_hud(
                    &mut canvas,
                    &texture_creator,
                    &font,
                    &powerup_assets,
                )?;

                if !pause_menu.paused {
                    two_player_state.use_powerups(&mut events);
                    two_player_state.update(&track, &events, sec_per_frame);
                }
            }
        }

        pause_menu.display(&mut canvas, &texture_creator, &font, &events)?;

        if pause_menu.handle_click(&mut events, canvas.output_size()?) {
            screen = GameScreen::MainMenu;
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
        )?;

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
