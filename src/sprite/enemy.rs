use crate::sprite::{Sprite, SpriteType};

pub struct Enemy {
    pub sprite: Sprite,
    start: (f64, f64),
    end: (f64, f64),
}

pub struct Fireball {
    pub sprite: Sprite,
    pub target: SpriteType,
    pub timer: f64,
}

impl Enemy {
    pub fn new(x: f64, z: f64, endx: f64, endz: f64) -> Enemy {
        Enemy {
            sprite: Sprite::new(x, z, SpriteType::Enemy),
            start: (x, z),
            end: (endx, endz),
        }
    }

    pub fn update(&mut self, dt: f64) {
        let diffx = self.end.0 - self.sprite.trans_x;
        let diffz = self.end.1 - self.sprite.trans_z;
        let dist = (diffx * diffx + diffz * diffz).sqrt();

        if dist < 0.1 {
            let tmp = self.end;
            self.end = self.start;
            self.start = tmp;
            return;
        }

        let speedx = diffx / dist * 2.0;
        let speedz = diffz / dist * 2.0;
        self.sprite.trans_x += speedx * dt;
        self.sprite.trans_z += speedz * dt;
    }
}

impl Fireball {
    pub fn new(x: f64, z: f64, target_type: SpriteType) -> Fireball {
        Fireball {
            sprite: Sprite::new(x, z, SpriteType::Fireball),
            timer: 0.0,
            target: target_type,
        }
    }

    pub fn update(&mut self, dt: f64, target: &Sprite) {
        self.timer += dt;

        let dist = ((target.trans_x - self.sprite.trans_x)
            * (target.trans_x - self.sprite.trans_x)
            + (target.trans_z - self.sprite.trans_z) * (target.trans_z - self.sprite.trans_z))
            .sqrt();
        let xdiff = target.trans_x - self.sprite.trans_x;
        let zdiff = target.trans_z - self.sprite.trans_z;

        let speedx = xdiff / dist * 4.0;
        let speedz = zdiff / dist * 4.0;

        self.sprite.trans_x += speedx * dt;
        self.sprite.trans_z += speedz * dt;
    }
}
