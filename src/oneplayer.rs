use crate::sprite;
use crate::sprite::kart::Kart;
use crate::{events, level};
use sdl2::keyboard::Keycode;
use sdl2::render::Texture;
use std::collections::HashMap;

pub fn init<'a>(
    player_kart1: &mut Kart<'a>,
    cam1: &mut level::Camera,
    enemies: &mut Vec<sprite::enemy::Enemy<'a>>,
    sprite_assets: &'a HashMap<sprite::SpriteType, Texture>,
    checkpoint1: &mut sprite::Sprite<'a>,
    first_checkpoint: &(f64, f64),
) {
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

    *player_kart1 = sprite::kart::Kart::new(9.0, 35.5, sprite::SpriteType::Kart1, &sprite_assets);
    player_kart1.sprite.width = 0.1;
    player_kart1.sprite.height = 0.1;
    player_kart1.sprite.frame_count = 8;
    player_kart1.sprite.rotation = 3.14159 / 2.0;
    player_kart1.laps = 0;

    player_kart1.move_kart(0.0);

    cam1.follow(&player_kart1.sprite, 1.1);

    *checkpoint1 = sprite::Sprite::new(
        first_checkpoint.0,
        first_checkpoint.1,
        sprite::SpriteType::Checkpoint1,
        &sprite_assets,
    );
    checkpoint1.width = 0.1;
    checkpoint1.height = 0.1;
}

pub fn update_karts(
    player_kart1: &mut Kart,
    checkpoint1: &mut sprite::Sprite,
    cam1: &mut level::Camera,
    events: &events::Events,
    level: &level::Level,
    sec_per_frame: f64,
    start_timer: f64,
) {
    //Move the karts
    if start_timer <= 0.0 && player_kart1.laps < 4 {
        player_kart1.drive_kart(&events, Keycode::Up, Keycode::Left, Keycode::Right);
        player_kart1.move_kart(sec_per_frame);
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

    player_kart1.apply_friction(&level);

    if player_kart1.knock_out <= 0.0 {
        cam1.follow(&player_kart1.sprite, 1.1);
    }

    if player_kart1.laps == 4 {
        player_kart1.sprite.speed = 0.0;
        player_kart1.sprite.rotation_speed = 0.0;
    }
}

pub fn update_enemies(
    enemies: &mut Vec<sprite::enemy::Enemy>,
    player_kart1: &mut Kart,
    sec_per_frame: f64,
) {
    for enemy in enemies {
        if sprite::dist_between(&enemy.sprite, &player_kart1.sprite) < 0.2
            && player_kart1.knock_out <= 0.0
        {
            player_kart1.knock_out = 1.0;
        }

        enemy.update(sec_per_frame);
    }
}
