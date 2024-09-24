use crate::level::Camera;
use crate::menu::Text;
use crate::sprite::kart::{Kart, PowerupType};
use crate::sprite::{Sprite, SpriteType};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

pub fn cmp_aspect(canvas_dimensions: (u32, u32), buff_w: usize, buff_h: usize) -> bool {
    let (canv_w, canv_h) = canvas_dimensions;
    canv_h * buff_w as u32 / buff_h as u32 > canv_w
}

pub fn display_sprites(
    canvas: &mut Canvas<Window>,
    cam: &Camera,
    sprites: &[&Sprite],
    canvas_dimensions: (u32, u32),
    canvas_origin: (i32, i32),
    buff_dimensions: (usize, usize),
    sprite_assets: &HashMap<SpriteType, Texture>,
) -> Result<(), String> {
    //Draw the sprites
    for spr in sprites {
        spr.display(
            canvas,
            cam,
            buff_dimensions,
            canvas_dimensions,
            (canvas_origin.0, canvas_origin.1),
            sprite_assets,
        )?;
    }

    Ok(())
}

pub fn calculate_texture_rect(canvas_dimensions: (u32, u32), buff_w: usize, buff_h: usize) -> Rect {
    let (canv_w, canv_h) = canvas_dimensions;
    let center = Point::new(canv_w as i32 / 2, canv_h as i32 / 2);
    if canv_h * buff_w as u32 / buff_h as u32 > canv_w {
        let width = canv_h * buff_w as u32 / buff_h as u32;
        Rect::from_center(center, width, canv_h)
    } else {
        let height = canv_w * buff_h as u32 / buff_w as u32;
        Rect::from_center(center, canv_w, height)
    }
}

//Draws text for the player kart
pub fn display_player_info(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    kart: &Kart,
    x: i32,
    y: i32,
) -> Result<(), String> {
    let speed_text = Text::new(
        format!("speed: {}", ((kart.speed * 100.0 * 20.0).round() / 100.0)).as_str(),
        x + 16,
        y + 16,
        Color::WHITE,
        16,
    );
    //Display speed
    speed_text.display_left_justify(canvas, texture_creator, font)?;

    let laps_text = Text::new(
        format!("laps: {}", kart.laps).as_str(),
        x + 16,
        y + 40,
        Color::WHITE,
        16,
    );
    laps_text.display_left_justify(canvas, texture_creator, font)?;

    let pos_text = Text::new(
        format!(
            "pos: {}, {}",
            ((kart.sprite.trans_x * 100.0).round() / 100.0),
            ((kart.sprite.trans_z * 100.0).round() / 100.0)
        )
        .as_str(),
        x + 16,
        y + 80,
        Color::WHITE,
        8,
    );
    pos_text.display_left_justify(canvas, texture_creator, font)?;

    Ok(())
}

pub fn display_powerup_icons(
    canvas: &mut Canvas<Window>,
    icons: &HashMap<PowerupType, Texture>,
    icon_sz: u32,
    kart: &Kart,
    x: i32,
    y: i32,
) -> Result<(), String> {
    for i in 0..(kart.powerup_amt as i32) {
        if let Some(tex) = icons.get(&kart.powerup) {
            let offset =
                i * icon_sz as i32 / 4 * 3 - icon_sz as i32 * kart.powerup_amt as i32 / 8 * 3;
            let rect = Rect::new(x + offset, y, icon_sz, icon_sz);
            canvas.copy(tex, None, rect).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub fn display_start_timer(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    canvas_dimensions: (u32, u32),
    font: &Font,
    start_timer: f64,
) -> Result<(), String> {
    let (canv_w, canv_h) = canvas_dimensions;
    if start_timer.ceil() > 0.0 {
        let text = Text::new(
            format!("{}", start_timer.ceil()).as_str(),
            canv_w as i32 / 2,
            canv_h as i32 / 2 - 64,
            Color::WHITE,
            64,
        );

        text.display_center(canvas, texture_creator, font)?;
    } else if start_timer.ceil() == 0.0 {
        let text = Text::new(
            "GO!",
            canv_w as i32 / 2,
            canv_h as i32 / 2 - 64,
            Color::WHITE,
            64,
        );

        text.display_center(canvas, texture_creator, font)?;
    }

    Ok(())
}
