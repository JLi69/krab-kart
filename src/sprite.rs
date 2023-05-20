use crate::events::Events;
use crate::level::{Camera, Level};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use std::fs::File;

pub struct Sprite<'a> {
    pub trans_x: f64,
    pub trans_z: f64,
    pub rotation: f64,
    pub speed: f64,
    pub camera_kart: bool, //Stores if it is the kart that is focused on in the camera
    pub rotation_speed: f64,
    pub max_rotation_speed: f64,
    pub width: f64,
    pub height: f64,
    pub acceleration: f64,
    pub friction: f64,
    pub max_speed: f64,
    pub frame: i32,
    pub frame_count: i32, //Number of rotation frames that the sprite has
    pub spr_texture: Option<&'a Texture<'a>>,
}

//Uses png crate to load bytes from a PNG file and then
//copies those bytes into a texture
//Works only on PNG images
pub fn load_texture<'a>(
    path: String,
    texture_creator: &'a TextureCreator<WindowContext>,
) -> Result<Texture<'a>, String> {
    let img_file = File::open(&path);

    match img_file {
        Ok(file) => {
            let decoder = png::Decoder::new(file);
            let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
            //Bytes read in from the image, copoy this into the SDL texture
            let mut png_buffer = vec![0u8; reader.output_buffer_size()];
            // Read the next frame. An APNG might contain multiple frames.
            let info = reader
                .next_frame(&mut png_buffer)
                .map_err(|e| e.to_string())?;

            //Create empty SDL texture
            let mut tex = texture_creator
                .create_texture_streaming(PixelFormatEnum::ARGB8888, info.width, info.height)
                .map_err(|e| e.to_string())?;
            //Enable Alpha so we can have transparency
            tex.set_blend_mode(BlendMode::Blend);

            //Write pixels into texture
            tex.with_lock(None, |pixels: &mut [u8], _pitch: usize| {
                for i in 0..(reader.output_buffer_size() as usize) {
                    //pixels[i] = png_buffer[i];
                    if i % 4 == 0 {
                        pixels[i] = png_buffer[i + 2];
                        pixels[i + 1] = png_buffer[i + 1];
                        pixels[i + 2] = png_buffer[i];
                        pixels[i + 3] = png_buffer[i + 3];
                    }
                }
            })
            .map_err(|e| e.to_string())?;

            Ok(tex)
        }
        Err(msg) => {
            eprintln!("{msg}");
            eprintln!("Cannot open image: {path}");
            Err(String::from("Failed to load texture"))
        }
    }
}

impl<'a> Sprite<'a> {
    pub fn new(x: f64, z: f64) -> Sprite<'a> {
        Sprite {
            trans_x: x,
            trans_z: z,
            rotation: 0.0,
            width: 24.0 / 256.0, //Default width and height are both 32 / 256
            height: 24.0 / 256.0,
            speed: 0.0,
            max_speed: 4.0,
            friction: 0.0,
            acceleration: 0.0,
            rotation_speed: 0.0,
            max_rotation_speed: 0.4,
            frame: 0,
            frame_count: 1,
            spr_texture: None,
            camera_kart: false,
        }
    }

    pub fn set_texture(&mut self, tex: Option<&'a Texture>) {
        self.spr_texture = tex;
    }

    //canvas origin is top left corner of canvas
    pub fn display(
        &self,
        canv: &mut Canvas<Window>,
        cam: &Camera,
        pixel_buff_width: usize,
        pixel_buff_height: usize,
        canvas_dimensions: &(u32, u32),
        canvas_origin: &(i32, i32),
    ) -> Result<(), String> {
        //Test graphic draw sprite
        let trans_sprite_x = (self.trans_x - cam.trans_x) * (cam.rotation).cos()
            - (self.trans_z - cam.trans_z) * (cam.rotation).sin();
        let trans_sprite_z = (self.trans_z - cam.trans_z) * (cam.rotation).cos()
            + (self.trans_x - cam.trans_x) * (cam.rotation).sin();

        //Scale sprite based on how far it is from the camera
        let sprite_w = self.width / trans_sprite_z;
        let sprite_h = self.height / trans_sprite_z;

        let spr_screen_y = pixel_buff_height as f64 * (cam.z_far - cam.z_near)
            / ((trans_sprite_z - cam.z_near) * 8.0 / 5.0)
            + pixel_buff_height as f64 / 8.0 * 3.0;
        let depth = (cam.z_far - cam.z_near) / (trans_sprite_z - cam.z_near);

        let startx = cam.x_near1 + (cam.x_far1 - cam.x_near1) / depth;
        let endx = cam.x_near2 + (cam.x_far2 - cam.x_near2) / depth;
        let spr_screen_x = (trans_sprite_x - startx) / (endx - startx) * pixel_buff_width as f64;

        let sprite_rect: Rect;

        if canvas_dimensions.1 * pixel_buff_width as u32 / pixel_buff_height as u32
            > canvas_dimensions.0
        {
            let x_offset = -(canvas_dimensions.1 as i32 * pixel_buff_width as i32
                / pixel_buff_height as i32)
                / 2
                + canvas_dimensions.0 as i32 / 2 as i32;

            sprite_rect = Rect::new(
                ((canvas_dimensions.1 as f64 * pixel_buff_width as f64 / pixel_buff_height as f64)
                    * (spr_screen_x)
                    / (pixel_buff_width as f64)
                    - sprite_w / 2.0
                        * (canvas_dimensions.1 as f64 * pixel_buff_width as f64
                            / pixel_buff_height as f64)) as i32
                    + x_offset
                    + canvas_origin.0,
                (canvas_dimensions.1 as f64 * (spr_screen_y) / (pixel_buff_height as f64)
                    - sprite_h
                        * (canvas_dimensions.1 as f64 * pixel_buff_width as f64
                            / pixel_buff_height as f64)) as i32
                    + canvas_origin.1,
                (sprite_w * canvas_dimensions.1 as f64 * pixel_buff_width as f64
                    / pixel_buff_height as f64) as u32,
                (sprite_h * canvas_dimensions.1 as f64 * pixel_buff_width as f64
                    / pixel_buff_height as f64) as u32,
            );
        } else {
            let y_offset = -(canvas_dimensions.0 as i32 * pixel_buff_height as i32
                / pixel_buff_width as i32)
                / 2
                + canvas_dimensions.1 as i32 / 2 as i32;

            sprite_rect = Rect::new(
                (canvas_dimensions.0 as f64 * (spr_screen_x) / (pixel_buff_width as f64)
                    - sprite_w / 2.0 * (canvas_dimensions.0 as f64)) as i32
                    + canvas_origin.0,
                ((canvas_dimensions.0 as f64 * pixel_buff_height as f64 / pixel_buff_width as f64)
                    * (spr_screen_y)
                    / (pixel_buff_height as f64)
                    - sprite_h * (canvas_dimensions.0 as f64)) as i32
                    + y_offset
                    + canvas_origin.1,
                (sprite_w as f64 * canvas_dimensions.0 as f64) as u32,
                (sprite_h as f64 * canvas_dimensions.0 as f64) as u32,
            );
        }

        if self.dist2_to_camera(cam) < 16.0 * 16.0 {
            match self.spr_texture {
                Some(tex) => {
                    canv.copy(tex, Rect::new(self.frame * 32, 0, 32, 32), sprite_rect)
                        .map_err(|e| e.to_string())?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    //This will set the rotation frame of the sprite
    //if it is the kart that a camera is following
    //if it is rotatiing is to the left, it will turn left (frame 1)
    //if it is to the right, it will turn to the right (frame frame_count - 1)
    //if it is not rotating, just set it to the default frame
    //this is to create a sense of rotation when driving the kart and give the
    //player feedback on where they are turning as if we just used set_rotation_frame,
    //as the camera follows the sprite it will always appear to be the same rotation
    fn set_kart_rotation_frame(&mut self) {
        if self.rotation_speed < -0.05 {
            //Turning to the left
            self.frame = self.frame_count - 1;
        } else if self.rotation_speed > 0.05 {
            //Turning to the right
            self.frame = 1;
        } else if self.rotation_speed == 0.0 {
            //Not turning
            self.frame = 0;
        }
    }

    pub fn set_rotation_frame(&mut self, cam: &Camera) {
        if self.camera_kart {
            self.set_kart_rotation_frame();
            return;
        }

        let mut trans_angle =
            self.rotation - cam.rotation + 3.141459 / (self.frame_count as f64 * 2.0);
        while trans_angle < 0.0 {
            trans_angle += 3.14159 * 2.0;
        }
        while trans_angle >= 3.14159 * 2.0 {
            trans_angle -= 3.14159 * 2.0;
        }
        self.frame = (trans_angle / (3.14159 / (self.frame_count as f64 / 2.0))).floor() as i32;
    }

    //Move the sprite
    pub fn move_sprite(&mut self, dt: f64) {
        self.speed += (self.acceleration - self.friction) * dt;

        if self.speed > self.max_speed {
            self.speed = self.max_speed;
        }

        if self.rotation_speed > self.max_rotation_speed {
            self.rotation_speed = self.max_rotation_speed;
        }

        if self.speed < 0.0 {
            self.speed = 0.0;
        }

        self.trans_x += self.rotation.sin() * self.speed * dt;
        self.trans_z += self.rotation.cos() * self.speed * dt;
        self.rotation += self.rotation_speed * dt;
    }

    //Returns distance to camera squared
    pub fn dist2_to_camera(&self, cam: &Camera) -> f64 {
        (self.trans_x - cam.trans_x) * (self.trans_x - cam.trans_x)
            + (self.trans_z - cam.trans_z) * (self.trans_z - cam.trans_z)
    }

    //Drives kart with keyboard
    //acceleration key: acclerate forward
    //left key: turn left
    //right key: turn right
    pub fn drive_kart(
        &mut self,
        events: &Events,
        acceleration_key: Keycode,
        left_key: Keycode,
        right_key: Keycode,
    ) {
        //Accelerate kart
        if events.key_is_pressed(acceleration_key) {
            //Set kart's speed to be a minimum of 0.5
            if self.speed < 0.5 {
                self.speed = 0.5;
            }

            self.acceleration = 0.6;
        } else {
            //Stop accelecration once key is released
            self.acceleration = 0.0;
        }

        //Rotate left and rotate right
        if events.key_is_pressed(left_key) {
            self.rotation_speed = -self.speed;
        } else if events.key_is_pressed(right_key) {
            self.rotation_speed = self.speed;
        } else {
            //None of these keys pressed, don't rotate
            self.rotation_speed = 0.0;
        }
    }

    //Apply friction and maximum speed to the kart
    pub fn apply_friction(&mut self, level: &Level) {
        let color = level.sample_color(self.trans_x, self.trans_z);
        //Friction
        if color[0] == 0 && color[1] >= 128 && color[2] == 0 {
            //Green = grass, slippery
            //set maximum speed as well
            self.max_speed = 0.75;
            self.friction = 0.1;
        } else {
            //Everything else is road
            self.max_speed = 3.0;
            self.friction = 0.4;
        }
    }
}
