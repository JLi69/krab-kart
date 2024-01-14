use super::{TwoplayerState, LAPS_TO_WIN, START_TIMER};
use crate::level::Level;
use crate::menu::Text;
use crate::sprite::{bitmap::BitMap, kart::PowerupType, Sprite, SpriteType};
use crate::{display, HEIGHT, WIDTH};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};
use std::collections::HashMap;

impl TwoplayerState {
    fn display_victory_text(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let mut victory_text = Text::new("", 0, 0, Color::BLACK, 32);
        if self.player_kart1.laps >= LAPS_TO_WIN {
            victory_text.text = String::from("PLAYER 1 WINS!");
            victory_text.color = Color::RED;
            victory_text.display_center(canvas, texture_creator, font)?;
        } else if self.player_kart2.laps >= LAPS_TO_WIN {
            victory_text.text = String::from("PLAYER 2 WINS!");
            victory_text.color = Color::BLUE;
            victory_text.display_center(canvas, texture_creator, font)?;
        }

        Ok(())
    }

    pub fn create_background_texture(
        &self,
        pixel_buffer: &mut [u8],
        level: &Level,
        track_textures: &HashMap<u32, BitMap>,
    ) {
        let sz = pixel_buffer.len() / 2;
        if self.player_kart1.moving() || self.start_timer == START_TIMER {
            level.display_level(
                &mut pixel_buffer[..sz],
                WIDTH,
                HEIGHT / 2,
                &self.cam1,
                track_textures,
            );
        }

        if self.player_kart2.moving() || self.start_timer == START_TIMER {
            level.display_level(
                &mut pixel_buffer[sz..],
                WIDTH,
                HEIGHT / 2,
                &self.cam2,
                track_textures,
            );
        }
    }

    pub fn display_background(
        &self,
        canvas: &mut Canvas<Window>,
        pixel_buffer: &[u8],
        background_texture: &mut Texture,
        offset_y: u32,
    ) -> Result<(), String> {
        let canvas_dimensions = canvas.output_size()?;
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        background_texture
            .update(None, pixel_buffer, WIDTH * 4)
            .map_err(|e| e.to_string())?;
        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4 + offset_y as i32,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );
        canvas
            .copy(background_texture, None, texture_rect)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn display_hud(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
        powerup_icons: &HashMap<PowerupType, Texture>,
    ) -> Result<(), String> {
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);

        display::display_start_timer(
            canvas,
            texture_creator,
            &canvas_dimensions,
            font,
            self.start_timer,
        )?;

        display::display_player_info(canvas, texture_creator, font, &self.player_kart1, 0, 0)?;

        display::display_powerup_icons(
            canvas,
            powerup_icons,
            48,
            &self.player_kart1,
            (canvas_dimensions.0 / 2) as i32,
            16,
        )?;

        display::display_player_info(
            canvas,
            texture_creator,
            font,
            &self.player_kart2,
            0,
            canvas_dimensions_half.1 as i32,
        )?;

        display::display_powerup_icons(
            canvas,
            powerup_icons,
            48,
            &self.player_kart2,
            (canvas_dimensions.0 / 2) as i32,
            canvas_dimensions_half.1 as i32 + 16,
        )?;

        self.display_victory_text(canvas, texture_creator, font)?;

        Ok(())
    }

    fn get_sprites_to_draw(&self, kart: SpriteType) -> Vec<&Sprite> {
        let mut sprites_to_draw = vec![];

        sprites_to_draw.push(&self.player_kart1.sprite);
        sprites_to_draw.push(&self.player_kart2.sprite);

        match kart {
            SpriteType::Kart1 => sprites_to_draw.push(&self.checkpoint1),
            SpriteType::Kart2 => sprites_to_draw.push(&self.checkpoint2),
            _ => {}
        }

        for powerup in &self.powerups {
            sprites_to_draw.push(&powerup.sprite);
        }

        for enemy in &self.enemies {
            sprites_to_draw.push(&enemy.sprite);
        }

        for fireball in &self.fireballs {
            sprites_to_draw.push(&fireball.sprite);
        }

        for banana in &self.bananas {
            sprites_to_draw.push(banana);
        }

        let cam = match kart {
            SpriteType::Kart1 => &self.cam1,
            SpriteType::Kart2 => &self.cam2,
            _ => return sprites_to_draw, //Returns sprites out of order
        };

        //Sort sprite vector based on distance to camera
        sprites_to_draw.sort_by(|sprite1, sprite2| {
            sprite2
                .dist2_to_camera(cam)
                .partial_cmp(&sprite1.dist2_to_camera(cam))
                .unwrap()
        });

        sprites_to_draw
    }

    pub fn display_sprites(
        &mut self,
        canvas: &mut Canvas<Window>,
        sprite_assets: &HashMap<SpriteType, Texture>,
        kart: SpriteType,
    ) -> Result<(), String> {
        let canvas_dimensions = canvas.output_size()?;
        let canvas_dimensions_half = (canvas_dimensions.0, canvas_dimensions.1 / 2);
        let canvas_texture_rect =
            display::calculate_texture_rect(&canvas_dimensions, WIDTH, HEIGHT);

        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );

        match kart {
            SpriteType::Kart1 => {
                self.player_kart1.sprite.camera_kart = self.player_kart1.knock_out <= 0.0;
            }
            SpriteType::Kart2 => {
                self.player_kart2.sprite.camera_kart = self.player_kart2.knock_out <= 0.0;
            }
            _ => {}
        }

        let sprites_to_draw = self.get_sprites_to_draw(kart);
        match kart {
            SpriteType::Kart1 => {
                display::display_sprites(
                    canvas,
                    &self.cam1,
                    &sprites_to_draw,
                    &canvas_dimensions_half,
                    texture_rect.y() / 2,
                    &(0, 0),
                    WIDTH,
                    HEIGHT / 2,
                    sprite_assets,
                )?;
            }
            SpriteType::Kart2 => {
                let offset_y = (texture_rect.height() - canvas_dimensions_half.1) as i32;
                let origin_y = canvas_dimensions_half.1 as i32;
                display::display_sprites(
                    canvas,
                    &self.cam2,
                    &sprites_to_draw,
                    &canvas_dimensions_half,
                    texture_rect.y() / 2 + offset_y,
                    &(0, origin_y),
                    WIDTH,
                    HEIGHT / 2,
                    sprite_assets,
                )?;
            }
            _ => {}
        }

        self.player_kart1.sprite.camera_kart = false;
        self.player_kart2.sprite.camera_kart = false;

        Ok(())
    }
}
