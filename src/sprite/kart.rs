use crate::events::Events;
use crate::level::Level;
use crate::sprite::{Sprite, SpriteType};
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::collections::HashMap;

pub struct Kart<'a> {
    pub sprite: Sprite<'a>,
    pub knock_out: f64, //if the kart is knocked out, this is set to a nonzero value
    rotation_before_knockout: f64,
}

impl<'a> Kart<'a> {
    pub fn new(
        x: f64,
        z: f64,
        spr_type: SpriteType,
        sprite_assets: &'a HashMap<SpriteType, Texture<'a>>,
    ) -> Kart<'a> {
        Kart {
            sprite: Sprite::new(x, z, spr_type, sprite_assets),
            knock_out: 0.0,
            rotation_before_knockout: 0.0,
        }
    }

    //Apply friction and maximum speed to the kart
    pub fn apply_friction(&mut self, level: &Level) {
        let color = level.sample_color(self.sprite.trans_x, self.sprite.trans_z);
        //Friction
        if color[0] == 0 && color[1] >= 128 && color[2] == 0 {
            //Green = grass, slippery
            //set maximum speed as well
            self.sprite.max_speed = 0.5;
            self.sprite.friction = 0.1;
        } else if color[0] == 0 && color[1] >= 128 && color[2] >= 128 {
            self.sprite.max_speed = 6.0;
            self.sprite.speed = 6.0;
        } else {
            //Everything else is road
            self.sprite.max_speed = 3.0;
            self.sprite.friction = 0.4;
        }
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
            if self.sprite.speed < 0.5 {
                self.sprite.speed = 0.5;
            }

            self.sprite.acceleration = 0.7;
        } else {
            //Stop accelecration once key is released
            self.sprite.acceleration = 0.0;
        }

        //Rotate left and rotate right
        if events.key_is_pressed(left_key) {
            self.sprite.rotation_speed = -self.sprite.speed;
        } else if events.key_is_pressed(right_key) {
            self.sprite.rotation_speed = self.sprite.speed;
        } else {
            //None of these keys pressed, don't rotate
            self.sprite.rotation_speed = 0.0;
        }
    }

    //Move the kart
    pub fn move_kart(&mut self, dt: f64) {
        //Knocked out
        if self.knock_out > 0.0 {
            self.knock_out -= dt;
            self.sprite.acceleration = 0.0;
            self.sprite.speed = 0.0;
            self.sprite.rotation_speed = 10.0;
            self.sprite.rotation += self.sprite.rotation_speed * dt;

            while self.sprite.rotation < 0.0 {
                self.sprite.rotation += 3.14159 * 2.0;
            }
            while self.sprite.rotation >= 3.14159 * 2.0 {
                self.sprite.rotation -= 3.14159 * 2.0;
            }

            if self.knock_out <= 0.0 {
                self.sprite.rotation = self.rotation_before_knockout;
            }

            return;
        }

        self.rotation_before_knockout = self.sprite.rotation;
        self.sprite.update(dt);
    }

    pub fn moving(&self) -> bool {
        self.sprite.rotation_speed != 0.0 || self.sprite.speed != 0.0
    }
}
