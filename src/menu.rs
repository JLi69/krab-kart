use crate::display;
use crate::events;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
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

    fn mouse_hovering(&self, events: &events::Events, dimensions: &(u32, u32)) -> bool {
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
        canvas_dimensions: &(u32, u32),
        texture_creator: &TextureCreator<WindowContext>,
        events: &events::Events,
        font: &Font,
    ) -> Result<(), String> {
        if self.mouse_hovering(events, canvas_dimensions) {
            display::display_text_center(
                canvas,
                &texture_creator,
                (canvas_dimensions.0 / 2) as i32 + self.x,
                (canvas_dimensions.1 / 2) as i32 + self.y - 24,
                &font,
                self.text.clone(),
                Color::RGB(255, 128, 0),
                24,
            )
            .map_err(|e| e.to_string())?;
        } else {
            display::display_text_center(
                canvas,
                &texture_creator,
                (canvas_dimensions.0 / 2) as i32 + self.x,
                (canvas_dimensions.1 / 2) as i32 + self.y - 24,
                &font,
                self.text.clone(),
                Color::WHITE,
                24,
            )
            .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn clicked(&self, events: &mut events::Events, dimensions: &(u32, u32)) -> bool {
        self.mouse_hovering(events, dimensions) && events.left_is_clicked()
    }
}
