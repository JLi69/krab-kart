use crate::level::Camera;
use crate::sprite::Sprite;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

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
