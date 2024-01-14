use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator},
    ttf::Font,
    video::{Window, WindowContext},
};
use std::collections::HashMap;

use crate::{
    display,
    events::Events,
    level::{
        camera_consts::{DEFAULT_CAM_FAR, DEFAULT_CAM_FOV, DEFAULT_CAM_NEAR},
        Camera, Level,
    },
    menu::{Button, Text},
    sprite::bitmap::BitMap,
    GameScreen, HEIGHT, WIDTH,
};

const MENU_CAMERA_ROTATION_SPEED: f64 = 0.2;

pub struct MainMenuScreen {
    oneplayer_button: Button,
    twoplayer_button: Button,
    quit_button: Button,
    camera: Camera,
}

impl MainMenuScreen {
    pub fn init() -> Self {
        Self {
            oneplayer_button: Button::new(0, -32, "One Player"),
            twoplayer_button: Button::new(0, 16, "Two Player"),
            quit_button: Button::new(0, 64, "QUIT"),
            camera: Camera::new(
                9.0,
                35.5,
                std::f64::consts::PI / 2.0,
                DEFAULT_CAM_NEAR,
                DEFAULT_CAM_FAR,
                DEFAULT_CAM_FOV,
            ),
        }
    }

    pub fn update_camera(&mut self, dt: f64) {
        self.camera.rotation += dt * MENU_CAMERA_ROTATION_SPEED;
    }

    pub fn press_buttons(
        &self,
        events: &mut Events,
        canvas_dimensions: (u32, u32),
    ) -> Option<GameScreen> {
        if self.quit_button.clicked(events, canvas_dimensions) {
            events.can_quit = true;
            return None;
        } else if self.oneplayer_button.clicked(events, canvas_dimensions) {
            return Some(GameScreen::OnePlayer);
        } else if self.twoplayer_button.clicked(events, canvas_dimensions) {
            return Some(GameScreen::TwoPlayer);
        }

        None
    }

    pub fn create_background_texture(
        &self,
        level: &Level,
        pixel_buffer: &mut [u8],
        texture: &mut Texture,
        track_textures: &HashMap<u32, BitMap>,
    ) -> Result<(), String> {
        //Background
        level.display_level(
            pixel_buffer,
            WIDTH,
            HEIGHT / 2,
            &self.camera,
            track_textures,
        );

        texture
            .update(None, pixel_buffer, WIDTH * 4)
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn display(
        &self,
        canvas: &mut Canvas<Window>,
        background_texture: &Texture,
        texture_creator: &TextureCreator<WindowContext>,
        events: &Events,
        font: &Font,
    ) -> Result<(), String> {
        //Get canvas dimensions
        let canvas_dimensions = canvas.output_size()?;
        let canvas_texture_rect = display::calculate_texture_rect(canvas_dimensions, WIDTH, HEIGHT);

        //Display the background texture
        let texture_rect = Rect::from_center(
            Point::new(
                canvas_texture_rect.x() + canvas_texture_rect.width() as i32 / 2,
                canvas_texture_rect.y() + canvas_texture_rect.height() as i32 / 4 * 3,
            ),
            canvas_texture_rect.width(),
            canvas_texture_rect.height() / 2,
        );
        canvas
            .copy(background_texture, None, texture_rect)
            .map_err(|e| e.to_string())?;

        //Display buttons
        self.oneplayer_button
            .display(canvas, texture_creator, events, font)?;
        self.twoplayer_button
            .display(canvas, texture_creator, events, font)?;
        self.quit_button
            .display(canvas, texture_creator, events, font)?;

        //Display title
        let (canv_w, canv_h) = canvas_dimensions;
        let title_text = Text::new(
            "Krab Kart",
            canv_w as i32 / 2,
            (canv_h as i32) / 4 - 64,
            Color::WHITE,
            64,
        );
        title_text.display_center(canvas, texture_creator, font)?;

        Ok(())
    }
}
