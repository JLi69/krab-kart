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
pub const SPRITE_SIZE: f64 = 0.1;

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

fn camera_translate(x: f64, z: f64, cam: &Camera) -> (f64, f64) {
    let translated_x = x - cam.trans_x;
    let translated_z = z - cam.trans_z;
    (translated_x, translated_z)
}

fn camera_rotate(x: f64, z: f64, cam: &Camera) -> (f64, f64) {
    let rotated_x = x * cam.rotation.cos() - z * cam.rotation.sin();
    let rotated_z = z * cam.rotation.cos() + x * (cam.rotation).sin();
    (rotated_x, rotated_z)
}

fn screen_dimensions(w: f64, h: f64, z: f64) -> (f64, f64) {
    (w / z, h / z)
}

fn screen_position(x: f64, z: f64, cam: &Camera, buff_w: usize, buff_h: usize) -> (f64, f64) {
    let buff_w = buff_w as f64;
    let buff_h = buff_h as f64;

    let depth = (cam.z_far - cam.z_near) / (z - cam.z_near);
    let spr_screen_y = buff_h * depth * 5.0 / 8.0 + buff_h * 3.0 / 8.0;

    let startx = cam.x_near1 + (cam.x_far1 - cam.x_near1) / depth;
    let endx = cam.x_near2 + (cam.x_far2 - cam.x_near2) / depth;
    let spr_screen_x = (x - startx) / (endx - startx) * buff_w;

    (spr_screen_x, spr_screen_y)
}

//Returns true of the aspect ratio of the canvas is less than the aspect ratio
//of the pixel buffer size
fn cmp_aspect(canvas_dimensions: (u32, u32), buff_w: usize, buff_h: usize) -> bool {
    let (canv_w, canv_h) = canvas_dimensions;
    (canv_h * buff_w as u32) / (buff_h as u32) > canv_w
}

fn get_rect_x_offset(canvas_dimensions: (u32, u32), buff_w: usize, buff_h: usize) -> i32 {
    let (canv_w, canv_h) = (canvas_dimensions.0 as i32, canvas_dimensions.1 as i32);
    let buff_w = buff_w as i32;
    let buff_h = buff_h as i32;
    -(canv_h * buff_w / buff_h) / 2 + canv_w / 2
}

fn get_rect_y_offset(canvas_dimensions: (u32, u32), buff_w: usize, buff_h: usize) -> i32 {
    let (canv_w, canv_h) = (canvas_dimensions.0 as i32, canvas_dimensions.1 as i32);
    let buff_w = buff_w as i32;
    let buff_h = buff_h as i32;
    -(canv_w * buff_h / buff_w) / 2 + canv_h / 2
}

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
    pub rotation_speed: f64,
    pub camera_kart: bool, //Stores if it is the kart that is focused on in the camera
    pub width: f64,
    pub height: f64,
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
            rotation_speed: 0.0,
            width: DEFAULT_SPRITE_SIZE,
            height: DEFAULT_SPRITE_SIZE,
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
            rotation_speed: 0.0,
            width: DEFAULT_SPRITE_SIZE,
            height: DEFAULT_SPRITE_SIZE,
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
        buff_dimensions: (usize, usize),
        canvas_dimensions: (u32, u32),
        canvas_origin: (i32, i32),
        sprite_assets: &HashMap<SpriteType, Texture>,
    ) -> Result<(), String> {
        let (buff_w, buff_h) = buff_dimensions;
        let (cam_x, cam_z) = camera_translate(self.trans_x, self.trans_z, cam);
        let (trans_sprite_x, trans_sprite_z) = camera_rotate(cam_x, cam_z, cam);
        //Scale sprite based on how far it is from the camera
        let (sprite_w, sprite_h) = screen_dimensions(self.width, self.height, trans_sprite_z);
        //Get the sprite's position on the screen
        let (spr_screen_x, spr_screen_y) =
            screen_position(trans_sprite_x, trans_sprite_z, cam, buff_w, buff_h);

        let (canv_w, canv_h) = (canvas_dimensions.0 as f64, canvas_dimensions.1 as f64);
        let (origin_x, origin_y) = canvas_origin;
        let sprite_rect = if cmp_aspect(canvas_dimensions, buff_w, buff_h) {
            let x_offset = get_rect_x_offset(canvas_dimensions, buff_w, buff_h);
            let aspect = buff_w as f64 / buff_h as f64;

            let w = sprite_w * canv_h * aspect;
            let h = sprite_h * canv_h * aspect;
            let x = (canv_h * aspect * spr_screen_x / buff_w as f64 - w / 2.0) as i32;
            let y = (canv_h * spr_screen_y / buff_h as f64 - h) as i32;

            Rect::new(x + x_offset + origin_x, y + origin_y, w as u32, h as u32)
        } else {
            let y_offset = get_rect_y_offset(canvas_dimensions, buff_w, buff_h);
            let aspect_inv = buff_h as f64 / buff_w as f64;

            let w = sprite_w * canv_w;
            let h = sprite_h * canv_w;
            let x = (canv_w * spr_screen_x / buff_w as f64 - w / 2.0) as i32;
            let y = (canv_w * aspect_inv * spr_screen_y / buff_h as f64 - h) as i32;

            Rect::new(x + origin_x, y + y_offset + origin_y, w as u32, h as u32)
        };

        if sprite_h > 0.005 {
            if let Some(tex) = sprite_assets.get(&self.sprite_type) {
                let frame = self.get_rotation_frame(cam);
                let tex_rect = Rect::new(frame * 32, 0, 32, 32);
                canv.copy(tex, tex_rect, sprite_rect)
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
}
