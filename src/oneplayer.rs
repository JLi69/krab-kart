use crate::events::Events;
use crate::level::{
    camera_consts::{DEFAULT_CAM_FAR, DEFAULT_CAM_FOLLOW_DIST, DEFAULT_CAM_FOV, DEFAULT_CAM_NEAR},
    create_enemies, Camera, Level, CHECKPOINTS,
};
use crate::sprite::{dist_between, enemy::Enemy, kart::Kart, Sprite, SpriteType, SPRITE_SIZE};
use sdl2::keyboard::Keycode;

mod oneplayer_display;

const START_TIMER: f64 = 3.0;

pub struct SingeplayerState {
    player_kart: Kart,
    cam: Camera,
    enemies: Vec<Enemy>,
    checkpoint: Sprite,
    start_timer: f64,
    timer: f64,
}

impl SingeplayerState {
    pub fn init() -> Self {
        let mut kart = Kart::new(9.0, 35.5, SpriteType::Kart1);
        kart.move_kart(0.0);

        let camera = Camera::create_following(
            &kart.sprite,
            DEFAULT_CAM_FOLLOW_DIST,
            DEFAULT_CAM_NEAR,
            DEFAULT_CAM_FAR,
            DEFAULT_CAM_FOV,
        );

        Self {
            player_kart: kart,
            cam: camera,
            enemies: create_enemies(),
            checkpoint: Sprite::from_tuple(CHECKPOINTS[0], SpriteType::Checkpoint1)
                .set_size(SPRITE_SIZE, SPRITE_SIZE),
            start_timer: START_TIMER,
            timer: 0.0,
        }
    }

    fn update_kart(&mut self, events: &Events, level: &Level, dt: f64) {
        //Move the kart
        if self.start_timer <= 0.0 && self.player_kart.laps < 4 {
            self.player_kart
                .drive_kart(events, Keycode::Up, Keycode::Left, Keycode::Right);
            self.player_kart.move_kart(dt);
        }

        self.player_kart.sprite.camera_kart = self.player_kart.knock_out <= 0.0;
        if level.kart_at_checkpoint(&self.player_kart, self.player_kart.current_checkpoint, 1.0) {
            if self.player_kart.current_checkpoint == level.checkpoints.len() - 1 {
                self.player_kart.laps += 1;
            }

            self.player_kart.current_checkpoint += 1;
            self.player_kart.current_checkpoint %= level.checkpoints.len();
            self.checkpoint.trans_x = level.checkpoints[self.player_kart.current_checkpoint].0;
            self.checkpoint.trans_z = level.checkpoints[self.player_kart.current_checkpoint].1;
        }

        self.player_kart.apply_friction(&level);

        if self.player_kart.knock_out <= 0.0 {
            self.cam
                .follow(&self.player_kart.sprite, DEFAULT_CAM_FOLLOW_DIST);
        }

        if self.player_kart.laps == 4 {
            self.player_kart.sprite.speed = 0.0;
            self.player_kart.sprite.rotation_speed = 0.0;
        }
    }

    fn update_enemies(&mut self, dt: f64) {
        for enemy in &mut self.enemies {
            if dist_between(&enemy.sprite, &self.player_kart.sprite) < 0.2
                && self.player_kart.knock_out <= 0.0
            {
                self.player_kart.knock_out = 1.0;
            }

            enemy.update(dt);
        }
    }

    pub fn update(&mut self, events: &Events, level: &Level, dt: f64) {
        self.update_kart(events, level, dt);
        self.update_enemies(dt);

        if self.start_timer > -1.0 {
            self.start_timer -= dt;
        }

        if self.player_kart.laps < 4 && self.start_timer < 0.0 {
            self.timer += dt;
        }
    }

    pub fn done(&self) -> bool {
        self.player_kart.laps >= 4
    }
}
