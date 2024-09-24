use crate::events::Events;
use crate::level::Level;
use crate::sprite::{Sprite, SpriteType, SPRITE_SIZE};
use sdl2::keyboard::Keycode;

const MAX_SPEED: f64 = 4.0;
const MAX_ROTATION_SPEED: f64 = 0.4;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum PowerupType {
    Empty,
    SpeedBoost,
    Fireball,
    Banana,
}

const POWERUP_TYPES: [PowerupType; 3] = [
    PowerupType::SpeedBoost,
    PowerupType::Fireball,
    PowerupType::Banana,
];

pub struct Kart {
    pub sprite: Sprite,
    pub knock_out: f64, //if the kart is knocked out, this is set to a nonzero value
    pub powerup: PowerupType,
    pub powerup_amt: u16,
    pub laps: u32,
    pub current_checkpoint: usize,
    pub speed: f64,
    rotation_before_knockout: f64,
    acceleration: f64,
    friction: f64,
    max_speed: f64,
    max_rotation_speed: f64,
}

impl Kart {
    pub fn knocked_out(&self) -> bool {
        self.knock_out > 0.0
    }

    pub fn new(x: f64, z: f64, spr_type: SpriteType) -> Kart {
        Kart {
            sprite: Sprite::new(x, z, spr_type)
                .set_size(SPRITE_SIZE, SPRITE_SIZE)
                .set_framecount(8)
                .set_rotation(std::f64::consts::PI / 2.0),
            knock_out: 0.0,
            rotation_before_knockout: 0.0,
            powerup: PowerupType::Empty,
            laps: 0,
            current_checkpoint: 0,
            powerup_amt: 0,
            speed: 0.0,
            max_speed: MAX_SPEED,
            friction: 0.0,
            acceleration: 0.0,
            max_rotation_speed: MAX_ROTATION_SPEED,
        }
    }

    //Apply friction and maximum speed to the kart
    pub fn apply_friction(&mut self, level: &Level) {
        let color = level.sample_color(self.sprite.trans_x, self.sprite.trans_z);
        //Friction
        if color[0] == 0 && color[1] >= 128 && color[2] == 0 {
            //Green = grass, slippery
            //set maximum speed as well
            self.max_speed = 0.5;
            self.friction = 0.1;
        } else if color[0] == 0 && color[1] >= 128 && color[2] >= 128 {
            self.max_speed = 6.0;
            self.speed = 6.0;
        } else {
            //Everything else is road
            self.max_speed = 3.0;
            self.friction = 0.4;
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
            if self.speed < 0.5 {
                self.speed = 0.5;
            }

            self.acceleration = 0.7;
        } else {
            //Stop accelecration once key is released
            self.acceleration = 0.0;
        }

        //Rotate left and rotate right
        if events.key_is_pressed(left_key) {
            self.sprite.rotation_speed = -self.speed;
        } else if events.key_is_pressed(right_key) {
            self.sprite.rotation_speed = self.speed;
        } else {
            //None of these keys pressed, don't rotate
            self.sprite.rotation_speed = 0.0;
        }
    }

    //Move the kart
    pub fn move_kart(&mut self, dt: f64) {
        //Knocked out
        if self.knocked_out() {
            self.knock_out -= dt;
            self.acceleration = 0.0;
            self.speed = 0.0;
            self.sprite.rotation_speed = 10.0;
            self.sprite.rotation += self.sprite.rotation_speed * dt;

            while self.sprite.rotation < 0.0 {
                self.sprite.rotation += std::f64::consts::PI * 2.0;
            }
            while self.sprite.rotation >= std::f64::consts::PI * 2.0 {
                self.sprite.rotation -= std::f64::consts::PI * 2.0;
            }

            if !self.knocked_out() {
                self.sprite.rotation = self.rotation_before_knockout;
            }

            return;
        }

        self.rotation_before_knockout = self.sprite.rotation;
        self.update(dt);
    }

    fn update(&mut self, dt: f64) {
        if self.speed > self.max_speed {
            self.acceleration = -(self.speed - self.max_speed) * 0.5;
        }

        self.speed += (self.acceleration - self.friction) * dt;

        let min = -self.max_rotation_speed;
        let max = self.max_rotation_speed;
        self.sprite.rotation_speed = self.sprite.rotation_speed.clamp(min, max);

        if self.speed < 0.0 {
            self.speed = 0.0;
        }

        self.sprite.trans_x += self.sprite.rotation.sin() * self.speed * dt;
        self.sprite.trans_z += self.sprite.rotation.cos() * self.speed * dt;
        self.sprite.rotation += self.sprite.rotation_speed * dt;
    }

    pub fn moving(&self) -> bool {
        self.sprite.rotation_speed != 0.0 || self.speed != 0.0
    }

    pub fn pickup_powerup(&mut self) {
        self.powerup_amt = rand::random::<u16>() % 3 + 1;
        self.powerup = POWERUP_TYPES[rand::random::<usize>() % 3];
    }

    pub fn use_powerup(&mut self) -> PowerupType {
        if self.knocked_out() {
            return PowerupType::Empty;
        }

        if self.powerup == PowerupType::Empty {
            self.powerup_amt = 0;
            return PowerupType::Empty;
        }

        if self.powerup_amt > 0 {
            self.powerup_amt -= 1;
            return self.powerup;
        }

        self.powerup = PowerupType::Empty;
        PowerupType::Empty
    }
}
