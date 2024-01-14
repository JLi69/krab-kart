use crate::events::Events;
use crate::level::camera_consts::{
    DEFAULT_CAM_FAR, DEFAULT_CAM_FOLLOW_DIST, DEFAULT_CAM_FOV, DEFAULT_CAM_NEAR,
};
use crate::level::{create_enemies, create_powerups, Camera, Level, CHECKPOINTS};
use crate::sprite::SPRITE_SIZE;
use crate::sprite::{
    dist_between, enemy::Enemy, enemy::Fireball, kart::Kart, kart::PowerupType, powerup::Powerup,
    xz_diff_norm, Sprite, SpriteType,
};
use sdl2::keyboard::Keycode;

mod twoplayer_display;

const POWERUP_SPAWN_DIST: f64 = 0.3;
const START_TIMER: f64 = 3.0;
const LAPS_TO_WIN: u32 = 4;

fn knockout_kart(kart: &mut Kart, spr: &Sprite, dist: f64, time: f64) -> bool {
    if dist_between(spr, &kart.sprite) < dist {
        if kart.knock_out <= 0.0 {
            kart.knock_out = time;
        }
        return true;
    }

    false
}

fn pickup_powerup(powerup: &mut Powerup, kart: &mut Kart, dist: f64) {
    if dist_between(&powerup.sprite, &kart.sprite) < dist
        && powerup.can_pickup()
        && kart.powerup_amt == 0
    {
        powerup.sprite.width = 0.0;
        powerup.sprite.height = 0.0;
        kart.pickup_powerup();
    }
}

fn kart_at_checkpoint(level: &Level, kart: &mut Kart, checkpoint: &mut Sprite) {
    if level.kart_at_checkpoint(kart, kart.current_checkpoint, 1.0) {
        if kart.current_checkpoint == level.checkpoints.len() - 1 {
            kart.laps += 1;
        }

        kart.current_checkpoint += 1;
        kart.current_checkpoint %= level.checkpoints.len();
        checkpoint.trans_x = level.checkpoints[kart.current_checkpoint].0;
        checkpoint.trans_z = level.checkpoints[kart.current_checkpoint].1;
    }
}

fn player_use_powerup(
    kart: &mut Kart,
    other_kart: &mut Kart,
    bananas: &mut Vec<Sprite>,
    fireballs: &mut Vec<Fireball>,
) {
    match kart.use_powerup() {
        PowerupType::SpeedBoost => {
            kart.sprite.speed += 1.0;
        }
        PowerupType::Banana => {
            let banana = Sprite::new(
                kart.sprite.trans_x - POWERUP_SPAWN_DIST * kart.sprite.rotation.sin(),
                kart.sprite.trans_z - POWERUP_SPAWN_DIST * kart.sprite.rotation.cos(),
                SpriteType::Banana,
            )
            .set_size(0.06, 0.06);
            bananas.push(banana);
        }
        PowerupType::Fireball => {
            let (xdiff, ydiff) = xz_diff_norm(&other_kart.sprite, &kart.sprite);
            fireballs.push(Fireball::new(
                kart.sprite.trans_x + POWERUP_SPAWN_DIST * xdiff,
                kart.sprite.trans_z + POWERUP_SPAWN_DIST * ydiff,
                other_kart.sprite.sprite_type,
            ));
        }
        _ => {}
    }
}

pub struct TwoplayerState {
    player_kart1: Kart,
    player_kart2: Kart,
    cam1: Camera,
    cam2: Camera,
    powerups: Vec<Powerup>,
    enemies: Vec<Enemy>,
    bananas: Vec<Sprite>,
    fireballs: Vec<Fireball>,
    checkpoint1: Sprite,
    checkpoint2: Sprite,
    start_timer: f64,
}

impl TwoplayerState {
    pub fn init() -> Self {
        let mut kart1 = Kart::new(9.0, 35.5, SpriteType::Kart1);
        let mut kart2 = Kart::new(9.0, 35.1, SpriteType::Kart2);
        kart1.move_kart(0.0);
        kart2.move_kart(0.0);

        let c1 = Camera::create_following(
            &kart1.sprite,
            DEFAULT_CAM_FOLLOW_DIST,
            DEFAULT_CAM_NEAR,
            DEFAULT_CAM_FAR,
            DEFAULT_CAM_FOV,
        );

        let c2 = Camera::create_following(
            &kart2.sprite,
            DEFAULT_CAM_FOLLOW_DIST,
            DEFAULT_CAM_NEAR,
            DEFAULT_CAM_FAR,
            DEFAULT_CAM_FOV,
        );

        Self {
            player_kart1: kart1,
            player_kart2: kart2,
            cam1: c1,
            cam2: c2,
            powerups: create_powerups(),
            enemies: create_enemies(),
            bananas: vec![],
            fireballs: vec![],
            checkpoint1: Sprite::from_tuple(CHECKPOINTS[0], SpriteType::Checkpoint1)
                .set_size(SPRITE_SIZE, SPRITE_SIZE),
            checkpoint2: Sprite::from_tuple(CHECKPOINTS[0], SpriteType::Checkpoint2)
                .set_size(SPRITE_SIZE, SPRITE_SIZE),
            start_timer: START_TIMER,
        }
    }

    pub fn use_powerups(&mut self, events: &mut Events) {
        if events.key_is_pressed_once(Keycode::Down) {
            player_use_powerup(
                &mut self.player_kart1,
                &mut self.player_kart2,
                &mut self.bananas,
                &mut self.fireballs,
            );
        }

        if events.key_is_pressed_once(Keycode::S) {
            player_use_powerup(
                &mut self.player_kart2,
                &mut self.player_kart1,
                &mut self.bananas,
                &mut self.fireballs,
            );
        }
    }

    fn update_enemies(&mut self, dt: f64) {
        for enemy in &mut self.enemies {
            knockout_kart(&mut self.player_kart1, &enemy.sprite, 0.2, 1.0);
            knockout_kart(&mut self.player_kart2, &enemy.sprite, 0.2, 1.0);
            enemy.update(dt);
        }
    }

    fn update_fireballs(&mut self, dt: f64) {
        //Update the fireballs
        let mut index = 0;
        while index < self.fireballs.len() {
            let fireball = &mut self.fireballs[index];

            match fireball.target {
                SpriteType::Kart1 => fireball.update(dt, &self.player_kart1.sprite),
                SpriteType::Kart2 => fireball.update(dt, &self.player_kart2.sprite),
                _ => {}
            }

            index += 1;

            if knockout_kart(&mut self.player_kart1, &fireball.sprite, 0.1, 2.0) {
                self.fireballs.remove(index - 1);
                index -= 1;
                continue;
            }

            if knockout_kart(&mut self.player_kart2, &fireball.sprite, 0.1, 2.0) {
                self.fireballs.remove(index - 1);
                index -= 1;
                continue;
            }

            if fireball.timer > 8.0 {
                self.fireballs.remove(index - 1);
                index -= 1;
                continue;
            }
        }
    }

    fn update_bananas(&mut self) {
        //Check for player collision with bananas
        let mut index = 0;
        while index < self.bananas.len() {
            let banana = &self.bananas[index];
            index += 1;

            if knockout_kart(&mut self.player_kart1, banana, 0.1, 3.0) {
                self.bananas.remove(index - 1);
                index -= 1;
                continue;
            }

            if knockout_kart(&mut self.player_kart2, banana, 0.1, 3.0) {
                self.bananas.remove(index - 1);
                index -= 1;
                continue;
            }
        }
    }

    fn update_powerups(&mut self, dt: f64) {
        for powerup in &mut self.powerups {
            pickup_powerup(powerup, &mut self.player_kart1, 0.2);
            pickup_powerup(powerup, &mut self.player_kart2, 0.2);
            powerup.update(dt);
        }
    }

    fn race_over(&self) -> bool {
        self.player_kart1.laps == 4 || self.player_kart2.laps == 4
    }

    fn update_karts(&mut self, level: &Level, events: &Events, dt: f64) {
        //Move the karts
        if self.start_timer <= 0.0 && !self.race_over() {
            self.player_kart1
                .drive_kart(events, Keycode::Up, Keycode::Left, Keycode::Right);
            self.player_kart2
                .drive_kart(events, Keycode::W, Keycode::A, Keycode::D);
            self.player_kart1.move_kart(dt);
            self.player_kart2.move_kart(dt);
        }

        kart_at_checkpoint(level, &mut self.player_kart1, &mut self.checkpoint1);
        kart_at_checkpoint(level, &mut self.player_kart2, &mut self.checkpoint2);

        self.player_kart1.apply_friction(level);
        self.player_kart2.apply_friction(level);

        if self.player_kart1.knock_out <= 0.0 {
            self.cam1.follow(&self.player_kart1.sprite, 1.1);
        }
        if self.player_kart2.knock_out <= 0.0 {
            self.cam2.follow(&self.player_kart2.sprite, 1.1);
        }

        if self.race_over() {
            self.player_kart1.sprite.speed = 0.0;
            self.player_kart1.sprite.rotation_speed = 0.0;
            self.player_kart2.sprite.speed = 0.0;
            self.player_kart2.sprite.rotation_speed = 0.0;
        }
    }

    pub fn update(&mut self, level: &Level, events: &Events, dt: f64) {
        self.update_karts(level, events, dt);
        self.update_enemies(dt);
        self.update_bananas();
        self.update_powerups(dt);
        self.update_fireballs(dt);

        if self.start_timer > -1.0 {
            self.start_timer -= dt;
        }
    }
}
