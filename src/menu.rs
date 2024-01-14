use crate::events;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub struct Button {
    // x and y are relative to the center of the screen
    pub x: i32,
    pub y: i32,
    pub text: String,
}

impl Button {
    pub fn new(buttonx: i32, buttony: i32, button_text: &str) -> Button {
        Button {
            x: buttonx,
            y: buttony,
            text: String::from(button_text),
        }
    }

    fn mouse_hovering(&self, events: &events::Events, dimensions: (u32, u32)) -> bool {
        let posx = events.mouse_state.x();
        let posy = events.mouse_state.y();
        let width = (self.text.len() * 24) as i32;

        posx < self.x + width / 2 + dimensions.0 as i32 / 2
            && posx > self.x - width / 2 + dimensions.0 as i32 / 2
            && posy < self.y + 24 + dimensions.1 as i32 / 2
            && posy > self.y - 24 + dimensions.1 as i32 / 2
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        events: &events::Events,
        font: &Font,
    ) -> Result<(), String> {
        let canvas_dimensions = canvas.output_size()?;
        let (canv_w, canv_h) = canvas_dimensions;
        let mut text = Text::new(
            self.text.as_str(),
            (canv_w / 2) as i32 + self.x,
            (canv_h / 2) as i32 + self.y - 24,
            Color::WHITE,
            24,
        );

        if self.mouse_hovering(events, canvas_dimensions) {
            text.color = Color::RGB(255, 128, 0);
        }

        text.display_center(canvas, texture_creator, font)?;

        Ok(())
    }

    pub fn clicked(&self, events: &mut events::Events, dimensions: (u32, u32)) -> bool {
        self.mouse_hovering(events, dimensions) && events.left_is_clicked()
    }
}

pub struct Text {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub font_sz: u32,
}

impl Text {
    pub fn new(string: &str, posx: i32, posy: i32, text_color: Color, text_sz: u32) -> Self {
        Self {
            text: String::from(string),
            x: posx,
            y: posy,
            color: text_color,
            font_sz: text_sz,
        }
    }

    pub fn display_center(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let font_texture = self.create_texture(font, texture_creator)?;
        let width = self.font_sz as i32 * self.text.len() as i32;
        let text_rect = Rect::new(self.x - width / 2, self.y, width as u32, self.font_sz * 2);
        canvas
            .copy(&font_texture, None, text_rect)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn display_right_justify(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let font_texture = self.create_texture(font, texture_creator)?;
        let width = self.font_sz as i32 * self.text.len() as i32;
        let text_rect = Rect::new(self.x - width, self.y, width as u32, self.font_sz * 2);
        canvas
            .copy(&font_texture, None, text_rect)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn display_left_justify(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
    ) -> Result<(), String> {
        let font_texture = self.create_texture(font, texture_creator)?;
        let width = self.font_sz * self.text.len() as u32;
        let text_rect = Rect::new(self.x, self.y, width, self.font_sz * 2);
        canvas
            .copy(&font_texture, None, text_rect)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn create_texture<'a>(
        &'a self,
        font: &Font,
        texture_creator: &'a TextureCreator<WindowContext>,
    ) -> Result<Texture, String> {
        let font_surface = font
            .render(self.text.as_str())
            .solid(self.color)
            .map_err(|e| e.to_string())?;
        let font_texture = texture_creator
            .create_texture_from_surface(&font_surface)
            .map_err(|e| e.to_string())?;
        Ok(font_texture)
    }
}
