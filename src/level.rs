use crate::sprite::{bitmap::BitMap, enemy::Enemy, kart::Kart, powerup::Powerup, Sprite};
use std::collections::HashMap;
use std::fs::File;

pub mod camera_consts {
    pub const DEFAULT_CAM_FOLLOW_DIST: f64 = 1.1;
    pub const DEFAULT_CAM_FOV: f64 = std::f64::consts::PI / 2.0;
    pub const DEFAULT_CAM_NEAR: f64 = 0.01;
    pub const DEFAULT_CAM_FAR: f64 = 1.0;
}

pub const CHECKPOINTS: [(f64, f64); 5] = [
    (11.0, 35.3),
    (34.0, 28.0),
    (32.0, 7.0),
    (13.0, 6.0),
    (11.0, 35.3),
];

pub const ENEMY_LOCATIONS: [(f64, f64); 7] = [
    (31.0, 18.0),
    (22.0, 4.0),
    (22.5, 7.0),
    (23.0, 4.0),
    (10.5, 17.0),
    (7.5, 15.5),
    (10.5, 16.0),
];

pub const ENEMY_GOAL_POS: [(f64, f64); 7] = [
    (31.0, 16.0),
    (22.0, 7.0),
    (22.5, 4.0),
    (23.0, 7.0),
    (7.5, 16.0),
    (10.5, 16.5),
    (7.5, 15.0),
];

pub const POWERUP_LOCATIONS: [(f64, f64); 11] = [
    (20.0, 35.2),
    (20.0, 36.0),
    (20.0, 34.4),
    (29.5, 22.5),
    (29.0, 22.0),
    (28.5, 21.5),
    (17.0, 5.0),
    (16.5, 4.5),
    (17.5, 4.5),
    (16.5, 5.5),
    (17.5, 5.5),
];

pub fn create_enemies() -> Vec<Enemy> {
    let mut enemies = vec![];

    for i in 0..ENEMY_LOCATIONS.len() {
        let enemy = Enemy::new(
            ENEMY_LOCATIONS[i].0,
            ENEMY_LOCATIONS[i].1,
            ENEMY_GOAL_POS[i].0,
            ENEMY_GOAL_POS[i].1,
        );
        enemies.push(enemy);
    }

    enemies
}

pub fn create_powerups() -> Vec<Powerup> {
    let mut powerups = vec![];

    for location in POWERUP_LOCATIONS {
        let powerup = Powerup::new(location.0, location.1);
        powerups.push(powerup);
    }

    powerups
}

pub struct Camera {
    pub trans_x: f64,
    pub trans_z: f64,
    pub rotation: f64,
    pub x_near1: f64,
    pub x_near2: f64,
    pub x_far1: f64,
    pub x_far2: f64,
    pub z_near: f64,
    pub z_far: f64,
}

pub struct Level {
    level_data: Vec<u8>,
    level_width: u32,
    level_height: u32,
    pub checkpoints: Vec<(f64, f64)>,
    pub level_scale: f64,
}

impl Level {
    //Loads the level from a file (a png image)
    //Does not account for sprites
    pub fn load_from_png(path: &str) -> Result<Level, String> {
        let level_file = File::open(path);

        match level_file {
            Ok(file) => {
                let decoder = png::Decoder::new(file);
                let mut reader = decoder.read_info().map_err(|e| e.to_string())?;
                let mut buff = vec![0u8; reader.output_buffer_size()];
                // Read the next frame. An APNG might contain multiple frames.
                let info = reader.next_frame(&mut buff).map_err(|e| e.to_string())?;

                Ok(Level {
                    level_data: buff,
                    level_width: info.width,
                    level_height: info.height,
                    checkpoints: Vec::from(CHECKPOINTS),
                    level_scale: 32.0,
                })
            }
            Err(msg) => {
                //Could not open level file, print error message and return error
                eprintln!("{msg}");
                eprintln!("Cannot open file: {path}");
                Err(String::from("Failed to open level file"))
            }
        }
    }

    pub fn display_level(
        &self,
        pixel_buffer: &mut [u8],
        buff_width: usize,
        buff_height: usize,
        cam: &Camera,
        track_textures: &HashMap<u32, BitMap>,
    ) {
        //The sky
        for y in 0..(buff_height / 8 * 3 + 1) {
            for x in 0..buff_width {
                let offset = y * 4 * buff_width + x * 4;
                pixel_buffer[offset] = 255;
                pixel_buffer[offset + 1] = 128;
                pixel_buffer[offset + 2] = 32;
                pixel_buffer[offset + 3] = 255;
            }
        }

        //The ground
        for y in (buff_height / 8 * 3 + 1)..(buff_height) {
            let depth = (y as f64 - buff_height as f64 / 8.0 * 3.0) / (buff_height / 8 * 5) as f64;
            let startx = cam.x_near1 + (cam.x_far1 - cam.x_near1) / depth;
            let endx = cam.x_near2 + (cam.x_far2 - cam.x_near2) / depth;

            let sample_z = cam.z_near + (cam.z_far - cam.z_near) / depth;

            for x in 0..buff_width {
                let offset = y * 4 * buff_width + x * 4;

                let sample_x = startx + (endx - startx) * (x as f64 / buff_width as f64);

                let rotated_x = sample_x * (-cam.rotation).cos() - (-cam.rotation).sin() * sample_z;
                let rotated_z = sample_x * (-cam.rotation).sin() + (-cam.rotation).cos() * sample_z;
                let trans_x = (rotated_x + cam.trans_x) * self.level_scale;
                let trans_z = (rotated_z + cam.trans_z) * self.level_scale;

                let ind = (trans_x).floor() as usize * 3
                    + (trans_z).floor() as usize * 3 * self.level_width as usize;

                //Ground
                pixel_buffer[offset] = 0;
                pixel_buffer[offset + 1] = 255;
                pixel_buffer[offset + 2] = 0;
                pixel_buffer[offset + 3] = 255;

                if trans_x < 0.0
                    || (trans_x) as u32 >= self.level_width
                    || trans_z < 0.0
                    || (trans_z) as u32 >= self.level_height
                {
                    let pixel_value = (pixel_buffer[offset] as u32) << 24
                        | (pixel_buffer[offset + 1] as u32) << 16
                        | (pixel_buffer[offset + 2] as u32) << 8
                        | 0xff;

                    if let Some(bitmap) = track_textures.get(&pixel_value) {
                        bitmap.sample(
                            (trans_x / 16.0).abs().fract(),
                            (trans_z / 16.0).abs().fract(),
                            &mut pixel_buffer[offset..(offset + 3)],
                        );
                    }

                    continue;
                }

                if ind < self.level_data.len() {
                    pixel_buffer[offset] = self.level_data[ind + 2];
                    pixel_buffer[offset + 1] = self.level_data[ind + 1];
                    pixel_buffer[offset + 2] = self.level_data[ind];
                }

                let pixel_value = (pixel_buffer[offset] as u32) << 24
                    | (pixel_buffer[offset + 1] as u32) << 16
                    | (pixel_buffer[offset + 2] as u32) << 8
                    | 0xff;

                if let Some(bitmap) = track_textures.get(&pixel_value) {
                    bitmap.sample(
                        (trans_x / 16.0).abs().fract(),
                        (trans_z / 16.0).abs().fract(),
                        &mut pixel_buffer[offset..(offset + 3)],
                    );
                }
            }
        }
    }

    pub fn kart_at_checkpoint(&self, spr: &Kart, index: usize, dist: f64) -> bool {
        if index >= self.checkpoints.len() {
            return false;
        }

        ((self.checkpoints[index].0 - spr.sprite.trans_x)
            * (self.checkpoints[index].0 - spr.sprite.trans_x)
            + (self.checkpoints[index].1 - spr.sprite.trans_z)
                * (self.checkpoints[index].1 - spr.sprite.trans_z))
            .sqrt()
            < dist
    }

    pub fn sample_color(&self, x: f64, z: f64) -> [u8; 3] {
        if x < 0.0 || x > self.level_width as f64 || z < 0.0 || z > self.level_height as f64 {
            return [0, 255, 0]; //Return green (grass) by default
        }

        let ind = ((x * self.level_scale).floor() * 3.0
            + (z * self.level_scale).floor() * 3.0 * self.level_width as f64)
            as usize;

        if ind >= self.level_data.len() {
            return [0, 255, 0];
        }

        [
            self.level_data[ind + 2],
            self.level_data[ind + 1],
            self.level_data[ind],
        ]
    }
}

impl Camera {
    //Creates camera, cam_fov and cam_rot are in radians
    pub fn new(x: f64, z: f64, cam_rot: f64, near: f64, far: f64, cam_fov: f64) -> Self {
        let cam_x_near1 = (-cam_fov / 2.0).sin() * near;
        let cam_x_near2 = (cam_fov / 2.0).sin() * near;
        let cam_x_far1 = (-cam_fov / 2.0).sin() * far;
        let cam_x_far2 = (cam_fov / 2.0).sin() * far;
        let cam_z_near = (cam_fov / 2.0).cos() * near;
        let cam_z_far = (cam_fov / 2.0).cos() * far;

        Self {
            trans_x: x,
            trans_z: z,
            rotation: cam_rot,

            x_near1: cam_x_near1,
            x_near2: cam_x_near2,
            x_far1: cam_x_far1,
            x_far2: cam_x_far2,
            z_near: cam_z_near,
            z_far: cam_z_far,
        }
    }

    //Creates a camera following a sprite
    pub fn create_following(spr: &Sprite, dist: f64, near: f64, far: f64, cam_fov: f64) -> Self {
        let cam_x_near1 = (-cam_fov / 2.0).sin() * near;
        let cam_x_near2 = (cam_fov / 2.0).sin() * near;
        let cam_x_far1 = (-cam_fov / 2.0).sin() * far;
        let cam_x_far2 = (cam_fov / 2.0).sin() * far;
        let cam_z_near = (cam_fov / 2.0).cos() * near;
        let cam_z_far = (cam_fov / 2.0).cos() * far;

        Self {
            trans_x: spr.trans_x - spr.rotation.sin() * dist,
            trans_z: spr.trans_z - spr.rotation.cos() * dist,
            rotation: spr.rotation,

            x_near1: cam_x_near1,
            x_near2: cam_x_near2,
            x_far1: cam_x_far1,
            x_far2: cam_x_far2,
            z_near: cam_z_near,
            z_far: cam_z_far,
        }
    }

    //Follow a sprite
    pub fn follow(&mut self, spr: &Sprite, dist: f64) {
        self.trans_x = spr.trans_x - spr.rotation.sin() * dist;
        self.trans_z = spr.trans_z - spr.rotation.cos() * dist;
        self.rotation = spr.rotation;
    }
}
