use crate::level::Camera;
use crate::sprite::Sprite;
use crate::sprite::kart::{PowerupType, Kart};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

pub fn display_sprites(
    canvas: &mut Canvas<Window>,
    cam: &Camera,
    sprites: &mut Vec<&mut Sprite>,
    canvas_dimensions: &(u32, u32),
    offset_y: i32,
    canvas_origin: &(i32, i32),
    pix_buff_w: usize,
    pix_buff_h: usize,
) -> Result<(), String> {
    //Sort sprite vector based on distance to camera
    sprites.sort_by(|sprite1, sprite2| {
        sprite2
            .dist2_to_camera(cam)
            .partial_cmp(&sprite1.dist2_to_camera(cam))
            .unwrap()
    });

    //Draw the sprites
    if canvas_dimensions.1 * pix_buff_w as u32 / pix_buff_h as u32 > canvas_dimensions.0 {
        for spr in sprites {
            spr.set_rotation_frame(cam);
            spr.display(
                canvas,
                cam,
                pix_buff_w,
                pix_buff_h,
                canvas_dimensions,
                canvas_origin,
            )
            .map_err(|e| e.to_string())?;
        }
    } else {
        for spr in sprites {
            spr.set_rotation_frame(cam);
            spr.display(
                canvas,
                cam,
                pix_buff_w,
                pix_buff_h,
                canvas_dimensions,
                &(canvas_origin.0, canvas_origin.1 + offset_y),
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

pub fn display_text_left_justify(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    x: i32,
    y: i32,
    font: &Font,
    text: String,
    col: Color,
    char_sz: u32,
) -> Result<(), String> {
    let font_surface = font
        .render(text.as_str())
        .solid(col)
        .map_err(|e| e.to_string())?;
    let font_texture = texture_creator
        .create_texture_from_surface(&font_surface)
        .map_err(|e| e.to_string())?;
    canvas
        .copy(
            &font_texture,
            None,
            Rect::new(x, y, char_sz * text.len() as u32, char_sz * 2),
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn display_text_center(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    x: i32,
    y: i32,
    font: &Font,
    text: String,
    col: Color,
    char_sz: u32,
) -> Result<(), String> {
    let font_surface = font
        .render(text.as_str())
        .solid(col)
        .map_err(|e| e.to_string())?;
    let font_texture = texture_creator
        .create_texture_from_surface(&font_surface)
        .map_err(|e| e.to_string())?;
    canvas
        .copy(
            &font_texture,
            None,
            Rect::new(
                x - char_sz as i32 * text.len() as i32 / 2,
                y,
                char_sz * text.len() as u32,
                char_sz * 2,
            ),
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn display_text_right_justify(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    x: i32,
    y: i32,
    font: &Font,
    text: String,
    col: Color,
    char_sz: u32,
) -> Result<(), String> {
    let font_surface = font
        .render(text.as_str())
        .solid(col)
        .map_err(|e| e.to_string())?;
    let font_texture = texture_creator
        .create_texture_from_surface(&font_surface)
        .map_err(|e| e.to_string())?;
    canvas
        .copy(
            &font_texture,
            None,
            Rect::new(
                x - text.len() as i32 * char_sz as i32,
                y,
                char_sz * text.len() as u32,
                char_sz * 2,
            ),
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn calculate_texture_rect(
    canvas_dimensions: &(u32, u32),
    pix_buff_w: usize,
    pix_buff_h: usize,
) -> Rect {
    if canvas_dimensions.1 * pix_buff_w as u32 / pix_buff_h as u32 > canvas_dimensions.0 {
        Rect::from_center(
            Point::new(
                canvas_dimensions.0 as i32 / 2,
                canvas_dimensions.1 as i32 / 2,
            ),
            canvas_dimensions.1 * pix_buff_w as u32 / pix_buff_h as u32,
            canvas_dimensions.1,
        )
    } else {
        Rect::from_center(
            Point::new(
                canvas_dimensions.0 as i32 / 2,
                canvas_dimensions.1 as i32 / 2,
            ),
            canvas_dimensions.0,
            canvas_dimensions.0 * pix_buff_h as u32 / pix_buff_w as u32,
        )
    }
}

//Draws text for the player kart
pub fn display_player_info(
    canvas: &mut Canvas<Window>,
    texture_creator: &TextureCreator<WindowContext>,
    font: &Font,
    kart: &Sprite,
    laps: u32,
    x: i32,
    y: i32,
) -> Result<(), String> {
    //Display speed
    display_text_left_justify(
        canvas,
        &texture_creator, x + 16,
        y + 16,
        &font,
        format!("speed: {}", ((kart.speed * 100.0 * 20.0).round() / 100.0)),
        Color::WHITE,
        16,
    )
    .map_err(|e| e.to_string())?;

    display_text_left_justify(
        canvas,
        &texture_creator,
        x + 16,
        y + 40,
        &font,
        format!("laps: {}", laps),
        Color::WHITE,
        16,
    )
    .map_err(|e| e.to_string())?;

    display_text_left_justify(
        canvas,
        &texture_creator,
        x + 16,
        y + 80,
        &font,
        format!(
            "pos: {}, {}",
            ((kart.trans_x * 100.0).round() / 100.0),
            ((kart.trans_z * 100.0).round() / 100.0)
        ),
        Color::WHITE,
        8,
    )
    .map_err(|e| e.to_string())?;

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

	for i in 0..kart.powerup_amt {
		match icons.get(&kart.powerup) {
			Some(tex) => {
				canvas.copy(tex, None, Rect::new(x + i as i32 * icon_sz as i32 / 4 * 3 - icon_sz as i32 * kart.powerup_amt as i32 / 8 * 3, y, icon_sz, icon_sz))
					.map_err(|e| e.to_string())?; 
			},
			_ => {}
		}
	}

	Ok(())
}
