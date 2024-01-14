use crate::sprite::{Sprite, SpriteType};

pub struct Powerup {
    pub sprite: Sprite,
    pub size: f64,
}

impl Powerup {
    pub fn new(x: f64, z: f64) -> Powerup {
        let spr = Sprite::new(x, z, SpriteType::Powerup);

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

    pub fn can_pickup(&self) -> bool {
        self.sprite.width >= self.size
    }
}
