use crate::sprite::{Sprite, SpriteType};
use sdl2::render::Texture;
use std::collections::HashMap;

pub struct Powerup<'a> {
    pub sprite: Sprite<'a>,
    pub size: f64,
}

impl<'a> Powerup<'a> {
    pub fn new(x: f64, z: f64, sprite_assets: &'a HashMap<SpriteType, Texture<'a>>) -> Powerup {
        let spr = Sprite::new(x, z, SpriteType::Powerup, sprite_assets);

        Powerup {
            size: spr.width,
            sprite: spr,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if self.sprite.width < self.size {
            self.sprite.width += dt * 0.1;
        }

        if self.sprite.height < self.size {
            self.sprite.height += dt * 0.1;
        }

        if self.sprite.width > self.size {
            self.sprite.width = self.size;
        }

        if self.sprite.height > self.size {
            self.sprite.height = self.size;
        }
    }
}
