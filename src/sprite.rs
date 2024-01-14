use crate::level::Camera;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use std::collections::HashMap;
use std::f64::consts::PI;

pub mod bitmap;
pub mod enemy;
pub mod kart;
pub mod load_assets;
pub mod powerup;

const DEFAULT_SPRITE_SIZE: f64 = 24.0 / 256.0;
const MAX_SPEED: f64 = 4.0;
const MAX_ROTATION_SPEED: f64 = 0.4;

pub const SPRITE_SIZE: f64 = 0.1;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum SpriteType {
    Kart1,
    Kart2,
    Checkpoint1,
    Checkpoint2,
    Banana,
    Enemy,
    Fireball,
    Powerup,
}

pub struct Sprite {
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
    pub sprite_type: SpriteType,
}

impl Sprite {
    pub fn new(x: f64, z: f64, spr_type: SpriteType) -> Sprite {
        Sprite {
            trans_x: x,
            trans_z: z,
            rotation: 0.0,
            width: DEFAULT_SPRITE_SIZE,
            height: DEFAULT_SPRITE_SIZE,
            speed: 0.0,
            max_speed: MAX_SPEED,
            friction: 0.0,
            acceleration: 0.0,
            rotation_speed: 0.0,
            max_rotation_speed: MAX_ROTATION_SPEED,
            frame: 0,
            frame_count: 1,
            camera_kart: false,
            sprite_type: spr_type,
        }
    }

    pub fn from_tuple(pos: (f64, f64), spr_type: SpriteType) -> Sprite {
        let (x, z) = pos;
        Sprite {
            trans_x: x,
            trans_z: z,
            rotation: 0.0,
            width: DEFAULT_SPRITE_SIZE,
            height: DEFAULT_SPRITE_SIZE,
            speed: 0.0,
            max_speed: MAX_SPEED,
            friction: 0.0,
            acceleration: 0.0,
            rotation_speed: 0.0,
            max_rotation_speed: 0.4,
            frame: 0,
            frame_count: 1,
            camera_kart: false,
            sprite_type: spr_type,
        }
    }

    pub fn set_size(mut self, w: f64, h: f64) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn set_framecount(mut self, frame_count: i32) -> Self {
        self.frame_count = frame_count;
        self
    }

    pub fn set_rotation(mut self, rotation: f64) -> Self {
        self.rotation = rotation;
        self
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
        sprite_assets: &HashMap<SpriteType, Texture>,
    ) -> Result<(), String> {
        let trans_sprite_x = (self.trans_x - cam.trans_x) * (cam.rotation).cos()
            - (self.trans_z - cam.trans_z) * (cam.rotation).sin();
        let trans_sprite_z = (self.trans_z - cam.trans_z) * (cam.rotation).cos()
            + (self.trans_x - cam.trans_x) * (cam.rotation).sin();

        //Scale sprite based on how far it is from the camera
        let sprite_w = self.width / (trans_sprite_z);
        let sprite_h = self.height / (trans_sprite_z);

        let spr_screen_y = pixel_buff_height as f64 * (cam.z_far - cam.z_near)
            / ((trans_sprite_z - cam.z_near) * 8.0 / 5.0)
            + pixel_buff_height as f64 / 8.0 * 3.0;
        let depth = (cam.z_far - cam.z_near) / (trans_sprite_z - cam.z_near);

        let startx = cam.x_near1 + (cam.x_far1 - cam.x_near1) / depth;
        let endx = cam.x_near2 + (cam.x_far2 - cam.x_near2) / depth;
        let spr_screen_x = (trans_sprite_x - startx) / (endx - startx) * pixel_buff_width as f64;

        let sprite_rect = if canvas_dimensions.1 * pixel_buff_width as u32
            / pixel_buff_height as u32
            > canvas_dimensions.0
        {
            let x_offset = -(canvas_dimensions.1 as i32 * pixel_buff_width as i32
                / pixel_buff_height as i32)
                / 2
                + canvas_dimensions.0 as i32 / 2_i32;

            Rect::new(
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
            )
        } else {
            let y_offset = -(canvas_dimensions.0 as i32 * pixel_buff_height as i32
                / pixel_buff_width as i32)
                / 2
                + canvas_dimensions.1 as i32 / 2_i32;

            Rect::new(
                (canvas_dimensions.0 as f64 * (spr_screen_x) / (pixel_buff_width as f64)
                    - sprite_w / 2.0 * (canvas_dimensions.0 as f64)) as i32
                    + canvas_origin.0,
                ((canvas_dimensions.0 as f64 * pixel_buff_height as f64 / pixel_buff_width as f64)
                    * (spr_screen_y)
                    / (pixel_buff_height as f64)
                    - sprite_h * (canvas_dimensions.0 as f64)) as i32
                    + y_offset
                    + canvas_origin.1,
                (sprite_w * canvas_dimensions.0 as f64) as u32,
                (sprite_h * canvas_dimensions.0 as f64) as u32,
            )
        };

        if sprite_h > 0.005 {
            if let Some(tex) = sprite_assets.get(&self.sprite_type) {
                canv.copy(
                    tex,
                    Rect::new(self.get_rotation_frame(cam) * 32, 0, 32, 32),
                    sprite_rect,
                )
                .map_err(|e| e.to_string())?;
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
    fn get_kart_rotation_frame(&self) -> i32 {
        if self.rotation_speed < -0.05 {
            //Turning to the left
            return self.frame_count - 1;
        } else if self.rotation_speed > 0.05 {
            //Turning to the right
            return 1;
        } else if self.rotation_speed == 0.0 {
            //Not turning
            return 0;
        }

        0
    }

    pub fn get_rotation_frame(&self, cam: &Camera) -> i32 {
        if self.camera_kart {
            return self.get_kart_rotation_frame();
        }

        let mut trans_angle = self.rotation - cam.rotation + PI / (self.frame_count as f64 * 2.0);
        while trans_angle < 0.0 {
            trans_angle += PI * 2.0;
        }
        while trans_angle >= PI * 2.0 {
            trans_angle -= PI * 2.0;
        }
        (trans_angle / (PI / (self.frame_count as f64 / 2.0))).floor() as i32
    }

    //Returns distance to camera squared
    pub fn dist2_to_camera(&self, cam: &Camera) -> f64 {
        (self.trans_x - cam.trans_x) * (self.trans_x - cam.trans_x)
            + (self.trans_z - cam.trans_z) * (self.trans_z - cam.trans_z)
    }

    pub fn update(&mut self, dt: f64) {
        if self.speed > self.max_speed {
            self.acceleration = -(self.speed - self.max_speed) * 0.5;
        }

        self.speed += (self.acceleration - self.friction) * dt;

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
}

//Distance between two sprites
pub fn dist_between(spr1: &Sprite, spr2: &Sprite) -> f64 {
    ((spr1.trans_x - spr2.trans_x) * (spr1.trans_x - spr2.trans_x)
        + (spr1.trans_z - spr2.trans_z) * (spr1.trans_z - spr2.trans_z))
        .sqrt()
}

pub fn xz_diff_norm(spr1: &Sprite, spr2: &Sprite) -> (f64, f64) {
    let dist = dist_between(spr1, spr2);
    let xdiff = spr1.trans_x - spr2.trans_x;
    let zdiff = spr1.trans_z - spr2.trans_z;
    (xdiff / dist, zdiff / dist)
}
