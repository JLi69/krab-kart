use crate::events::Events;
use crate::menu::{Button, Text};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::Font;
use sdl2::video::{Window, WindowContext};

pub struct PauseMenu {
    pub paused: bool,
    goto_menu: Button,
    goto_game: Button,
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {
            paused: false,
            goto_menu: Button::new(0, 0, "Main Menu"),
            goto_game: Button::new(0, 48, "Return to Game"),
        }
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<WindowContext>,
        font: &Font,
        events: &Events,
    ) -> Result<(), String> {
        if !self.paused {
            return Ok(());
        }

        let (canv_w, canv_h) = canvas.output_size()?;

        canvas.set_draw_color(Color::RGBA(128, 128, 128, 200));
        canvas
            .fill_rect(Rect::new(0, 0, canv_w, canv_h))
            .map_err(|e| e.to_string())?;

        let text = Text::new(
            "PAUSED",
            canv_w as i32 / 2,
            (canv_h as i32) / 4 - 64,
            Color::WHITE,
            64,
        );
        text.display_center(canvas, texture_creator, font)?;

        self.goto_menu
            .display(canvas, texture_creator, events, font)?;
        self.goto_game
            .display(canvas, texture_creator, events, font)?;

        Ok(())
    }

    pub fn handle_click(&mut self, events: &mut Events, canvas_dimensions: (u32, u32)) -> bool {
        if !self.paused {
            return false;
        }

        if self.goto_game.clicked(events, canvas_dimensions) {
            self.paused = false;
        } else if self.goto_menu.clicked(events, canvas_dimensions) {
            self.paused = false;
            return true;
        }

        false
    }

    pub fn listen_for_escape(&mut self, events: &mut Events) {
        if events.key_is_pressed_once(sdl2::keyboard::Keycode::Escape) {
            self.paused = !self.paused;
        }
    }
}
