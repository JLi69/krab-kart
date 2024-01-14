use super::{SingeplayerState, START_TIMER};
use crate::display;
use crate::level::Level;
use crate::menu::Text;
use crate::sprite::{bitmap::BitMap, Sprite, SpriteType};
use crate::{HEIGHT, WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

impl SingeplayerState {
    pub fn create_background_texture(
        &self,
        pixel_buffer: &mut [u8],
        level: &Level,
        track_textures: &HashMap<u32, BitMap>,
        background_texture: &mut Texture,
    ) -> Result<(), String> {
        //Display Level
        if self.player_kart.moving() || self.start_timer == START_TIMER {
            level.display_level(pixel_buffer, WIDTH, HEIGHT / 2, &self.cam, track_textures);
        }

        background_texture
            .update(None, pixel_buffer, WIDTH * 4)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn display_timer(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let mut timer_text = Text::new("", 0, 0, Color::WHITE, 32);
        let canvas_dimensions = canvas.output_size()?;
        timer_text.x = 0;
        timer_text.y = -(canvas_dimensions.1 as i32) / 8 * 3 - 32;
        //Display timer
        let minutes = (self.timer / 60.0).floor();
        let seconds = (self.timer - 60.0 * minutes).floor();
        if seconds < 10.0 {
            timer_text.text = format!("{}:0{}", minutes, seconds);
        } else {
            timer_text.text = format!("{}:{}", minutes, seconds);
        }

        timer_text.display_center(canvas, texture_creator, font)?;
        Ok(())
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        background_texture: &Texture,
    ) -> Result<(), String> {
        //Display background
        let canvas_dimensions = canvas.output_size()?;
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4 * 3,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );

        canvas
            .copy(&background_texture, None, texture_rect)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn display_hud(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let canvas_dimensions = canvas.output_size()?;

        //Display the timer
        self.display_timer(canvas, texture_creator, font)?;

        //Display player info
        display::display_player_info(canvas, texture_creator, font, &self.player_kart, 0, 0)?;

        //Print DONE when player reaches 4 laps
        if self.done() {
            let victory_text = Text::new("DONE!", 0, 0, Color::WHITE, 32);
            victory_text.display_center(canvas, texture_creator, font)?;
        }

        display::display_start_timer(
            canvas,
            texture_creator,
            &canvas_dimensions,
            font,
            self.start_timer,
        )?;

        Ok(())
    }

    fn get_sprites_to_draw(&self) -> Vec<&Sprite> {
        let mut sprites_to_draw = vec![];
        sprites_to_draw.push(&self.player_kart.sprite);
        sprites_to_draw.push(&self.checkpoint);

        for enemy in &self.enemies {
            sprites_to_draw.push(&enemy.sprite);
        }

        //Sort sprite vector based on distance to camera
        sprites_to_draw.sort_by(|sprite1, sprite2| {
            sprite2
                .dist2_to_camera(&self.cam)
                .partial_cmp(&sprite1.dist2_to_camera(&self.cam))
                .unwrap()
        });

        sprites_to_draw
    }

    pub fn display_sprites(
        &self,
        canvas: &mut Canvas<Window>,
        sprite_assets: &HashMap<SpriteType, Texture>,
    ) -> Result<(), String> {
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4 * 3,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );

        let origin_y = texture_rect.y() / 2;
        let sprites_to_draw = self.get_sprites_to_draw();
        display::display_sprites(
            canvas,
            &self.cam,
            &sprites_to_draw,
            &canvas_dimensions_half,
            origin_y + texture_rect.height() as i32 / 2 - canvas_dimensions_half.1 as i32,
            &(0, canvas_dimensions_half.1 as i32),
            WIDTH,
            HEIGHT / 2,
            sprite_assets,
        )?;

        Ok(())
    }
}
