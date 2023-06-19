use crate::sprite;
use crate::sprite::kart::Kart;
use crate::{events, level};
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::collections::HashMap;

pub fn init<'a>(
    player_kart1: &mut Kart<'a>,
    player_kart2: &mut Kart<'a>,
    cam1: &mut level::Camera,
    cam2: &mut level::Camera,
    powerups: &mut Vec<sprite::powerup::Powerup<'a>>,
    enemies: &mut Vec<sprite::enemy::Enemy<'a>>,
    bananas: &mut Vec<sprite::Sprite<'a>>,
    fireballs: &mut Vec<sprite::enemy::Fireball<'a>>,
    sprite_assets: &'a HashMap<sprite::SpriteType, Texture>,
    checkpoint1: &mut sprite::Sprite<'a>,
    checkpoint2: &mut sprite::Sprite<'a>,
    first_checkpoint: &(f64, f64),
) {
    powerups.clear();
    {
        let power_up_locations = vec![
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

        for location in power_up_locations {
            let powerup = sprite::powerup::Powerup::new(location.0, location.1, &sprite_assets);
            powerups.push(powerup);
        }
    }

    //Enemies
    enemies.clear();
    {
        let enemy_locations = vec![
            (31.0, 18.0),
            (22.0, 4.0),
            (22.5, 7.0),
            (23.0, 4.0),
            (10.5, 17.0),
            (7.5, 15.5),
            (10.5, 16.0),
        ];

        let enemy_goal_pos = vec![
            (31.0, 16.0),
            (22.0, 7.0),
            (22.5, 4.0),
            (23.0, 7.0),
            (7.5, 16.0),
            (10.5, 16.5),
            (7.5, 15.0),
        ];

        for i in 0..enemy_locations.len() {
            let enemy = sprite::enemy::Enemy::new(
                enemy_locations[i].0,
                enemy_locations[i].1,
                enemy_goal_pos[i].0,
                enemy_goal_pos[i].1,
                &sprite_assets,
            );
            enemies.push(enemy);
        }
    }

    bananas.clear();
    fireballs.clear();

    *player_kart1 = sprite::kart::Kart::new(9.0, 35.5, sprite::SpriteType::Kart1, &sprite_assets);
    player_kart1.sprite.width = 0.1;
    player_kart1.sprite.height = 0.1;
    player_kart1.sprite.frame_count = 8;
    player_kart1.sprite.rotation = 3.14159 / 2.0;

    *player_kart2 = sprite::kart::Kart::new(9.0, 35.1, sprite::SpriteType::Kart2, &sprite_assets);
    player_kart2.sprite.width = 0.1;
    player_kart2.sprite.height = 0.1;
    player_kart2.sprite.rotation = 3.14159 / 2.0;
    player_kart2.sprite.frame_count = 8;

    player_kart1.move_kart(0.0);
    player_kart2.move_kart(0.0);

    cam1.follow(&player_kart1.sprite, 1.1);
    cam2.follow(&player_kart2.sprite, 1.1);

    *checkpoint1 = sprite::Sprite::new(
        first_checkpoint.0,
        first_checkpoint.1,
        sprite::SpriteType::Checkpoint1,
        &sprite_assets,
    );
    checkpoint1.width = 0.1;
    checkpoint1.height = 0.1;
    *checkpoint2 = sprite::Sprite::new(
        first_checkpoint.0,
        first_checkpoint.1,
        sprite::SpriteType::Checkpoint2,
        &sprite_assets,
    );
    checkpoint2.width = 0.1;
    checkpoint2.height = 0.1;
}

pub fn use_powerups<'a>(
    player_kart1: &mut Kart,
    player_kart2: &mut Kart,
    events: &mut events::Events,
    bananas: &mut Vec<sprite::Sprite<'a>>,
    fireballs: &mut Vec<sprite::enemy::Fireball<'a>>,
    sprite_assets: &'a HashMap<sprite::SpriteType, Texture>,
) {
    //Use powerups
    if events.key_is_pressed_once(Keycode::Down) {
        let powerup = player_kart1.use_powerup();
        match powerup {
            sprite::kart::PowerupType::SpeedBoost => {
                player_kart1.sprite.speed += 1.0;
            }
            sprite::kart::PowerupType::Banana => {
                let mut banana = sprite::Sprite::new(
                    player_kart1.sprite.trans_x - 0.3 * player_kart1.sprite.rotation.sin(),
                    player_kart1.sprite.trans_z - 0.3 * player_kart1.sprite.rotation.cos(),
                    sprite::SpriteType::Banana,
                    sprite_assets,
                );
                banana.width = 0.06;
                banana.height = 0.06;
                bananas.push(banana);
            }
            sprite::kart::PowerupType::Fireball => {
                let (xdiff, ydiff) =
                    sprite::xz_diff_norm(&player_kart2.sprite, &player_kart1.sprite);
                fireballs.push(sprite::enemy::Fireball::new(
                    player_kart1.sprite.trans_x + 0.3 * xdiff,
                    player_kart1.sprite.trans_z + 0.3 * ydiff,
                    sprite_assets,
                    sprite::SpriteType::Kart2,
                ));
            }
            _ => {}
        }
    }
    if events.key_is_pressed_once(Keycode::S) {
        let powerup = player_kart2.use_powerup();
        match powerup {
            sprite::kart::PowerupType::SpeedBoost => {
                player_kart2.sprite.speed += 1.0;
            }
            sprite::kart::PowerupType::Banana => {
                let mut banana = sprite::Sprite::new(
                    player_kart2.sprite.trans_x - 0.3 * player_kart2.sprite.rotation.sin(),
                    player_kart2.sprite.trans_z - 0.3 * player_kart2.sprite.rotation.cos(),
                    sprite::SpriteType::Banana,
                    &sprite_assets,
                );
                banana.width = 0.06;
                banana.height = 0.06;
                bananas.push(banana);
            }
            sprite::kart::PowerupType::Fireball => {
                let (xdiff, ydiff) =
                    sprite::xz_diff_norm(&player_kart1.sprite, &player_kart2.sprite);
                fireballs.push(sprite::enemy::Fireball::new(
                    player_kart2.sprite.trans_x + 0.3 * xdiff,
                    player_kart2.sprite.trans_z + 0.3 * ydiff,
                    &sprite_assets,
                    sprite::SpriteType::Kart1,
                ));
            }
            _ => {}
        }
    }
}

pub fn update_karts(
    player_kart1: &mut Kart,
    player_kart2: &mut Kart,
    checkpoint1: &mut sprite::Sprite,
    checkpoint2: &mut sprite::Sprite,
    cam1: &mut level::Camera,
    cam2: &mut level::Camera,
    events: &events::Events,
    level: &level::Level,
    sec_per_frame: f64,
    start_timer: f64,
) {
    //Move the karts
    if start_timer <= 0.0 && player_kart1.laps < 4 && player_kart2.laps < 4 {
        player_kart1.drive_kart(&events, Keycode::Up, Keycode::Left, Keycode::Right);
        player_kart2.drive_kart(&events, Keycode::W, Keycode::A, Keycode::D);
        player_kart1.move_kart(sec_per_frame);
        player_kart2.move_kart(sec_per_frame);
    }

    if level.kart_at_checkpoint(&player_kart1, player_kart1.current_checkpoint, 1.5) {
        if player_kart1.current_checkpoint == level.checkpoints.len() - 1 {
            player_kart1.laps += 1;
        }

        player_kart1.current_checkpoint += 1;
        player_kart1.current_checkpoint %= level.checkpoints.len();
        checkpoint1.trans_x = level.checkpoints[player_kart1.current_checkpoint].0;
        checkpoint1.trans_z = level.checkpoints[player_kart1.current_checkpoint].1;
    }

    if level.kart_at_checkpoint(&player_kart2, player_kart2.current_checkpoint, 1.5) {
        if player_kart2.current_checkpoint == level.checkpoints.len() - 1 {
            player_kart2.laps += 1;
        }

        player_kart2.current_checkpoint += 1;
        player_kart2.current_checkpoint %= level.checkpoints.len();
        checkpoint2.trans_x = level.checkpoints[player_kart2.current_checkpoint].0;
        checkpoint2.trans_z = level.checkpoints[player_kart2.current_checkpoint].1;
    }

    player_kart1.apply_friction(&level);
    player_kart2.apply_friction(&level);

    if player_kart1.knock_out <= 0.0 {
        cam1.follow(&player_kart1.sprite, 1.1);
    }
    if player_kart2.knock_out <= 0.0 {
        cam2.follow(&player_kart2.sprite, 1.1);
    }

    if player_kart1.laps == 4 || player_kart2.laps == 4 {
        player_kart1.sprite.speed = 0.0;
        player_kart1.sprite.rotation_speed = 0.0;
        player_kart2.sprite.speed = 0.0;
        player_kart2.sprite.rotation_speed = 0.0;
    }
}

pub fn update_fireballs(
    fireballs: &mut Vec<sprite::enemy::Fireball>,
    player_kart1: &mut Kart,
    player_kart2: &mut Kart,
    sec_per_frame: f64,
) {
    //Update the fireballs
    let mut index = 0;
    while index < fireballs.len() {
        let fireball = &mut fireballs[index];
        match fireball.target {
            sprite::SpriteType::Kart1 => {
                fireball.update(sec_per_frame, &player_kart1.sprite);
            }
            sprite::SpriteType::Kart2 => {
                fireball.update(sec_per_frame, &player_kart2.sprite);
            }
            _ => {}
        }
        index += 1;

        if sprite::dist_between(&fireball.sprite, &player_kart1.sprite) < 0.1 {
            player_kart1.knock_out = 2.0;
            fireballs.remove(index - 1);
            index -= 1;
            continue;
        }

        if sprite::dist_between(&fireball.sprite, &player_kart2.sprite) < 0.1 {
            player_kart2.knock_out = 2.0;
            fireballs.remove(index - 1);
            index -= 1;
            continue;
        }

        if fireball.timer > 8.0 {
            fireballs.remove(index - 1);
            index -= 1;
            continue;
        }
    }
}

pub fn update_bananas(
    bananas: &mut Vec<sprite::Sprite>,
    player_kart1: &mut Kart,
    player_kart2: &mut Kart,
) {
    //Check for player collision with bananas
    let mut index = 0;
    while index < bananas.len() {
        let banana = &bananas[index];
        index += 1;

        if sprite::dist_between(banana, &player_kart1.sprite) < 0.1 {
            player_kart1.knock_out = 3.0;
            bananas.remove(index - 1);
            index -= 1;
            continue;
        }

        if sprite::dist_between(banana, &player_kart2.sprite) < 0.1 {
            player_kart2.knock_out = 3.0;
            bananas.remove(index - 1);
            index -= 1;
            continue;
        }
    }
}

pub fn update_enemies(
    enemies: &mut Vec<sprite::enemy::Enemy>,
    player_kart1: &mut Kart,
    player_kart2: &mut Kart,
    sec_per_frame: f64,
) {
    for enemy in enemies {
        if sprite::dist_between(&enemy.sprite, &player_kart1.sprite) < 0.2
            && player_kart1.knock_out <= 0.0
        {
            player_kart1.knock_out = 1.0;
        }
        if sprite::dist_between(&enemy.sprite, &player_kart2.sprite) < 0.2
            && player_kart2.knock_out <= 0.0
        {
            player_kart2.knock_out = 1.0;
        }

        enemy.update(sec_per_frame);
    }
}

pub fn update_powerups(
    powerups: &mut Vec<sprite::powerup::Powerup>,
    player_kart1: &mut Kart,
    player_kart2: &mut Kart,
    sec_per_frame: f64,
) {
    for powerup in powerups {
        if sprite::dist_between(&powerup.sprite, &player_kart1.sprite) < 0.2
            && powerup.sprite.width >= powerup.size
            && player_kart1.powerup_amt == 0
        {
            powerup.sprite.width = 0.0;
            powerup.sprite.height = 0.0;
            player_kart1.pickup_powerup();
        }
        if sprite::dist_between(&powerup.sprite, &player_kart2.sprite) < 0.2
            && powerup.sprite.width >= powerup.size
            && powerup.sprite.width >= powerup.size
            && player_kart2.powerup_amt == 0
        {
            powerup.sprite.width = 0.0;
            powerup.sprite.height = 0.0;
            player_kart2.pickup_powerup();
        }

        powerup.update(sec_per_frame);
    }
}
