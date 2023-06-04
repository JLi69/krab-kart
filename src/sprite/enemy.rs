use crate::sprite::{Sprite, SpriteType};
use std::collections::HashMap;
use sdl2::render::Texture;

pub struct Enemy<'a> {
	pub sprite: Sprite<'a>,
	start: (f64, f64),
	end: (f64, f64)
}

impl<'a> Enemy<'a> {
	pub fn new(x: f64, 
		   z: f64,
		   endx: f64,
		   endz: f64,
		   sprite_assets: &'a HashMap<SpriteType, Texture<'a>>
		   ) -> Enemy {
		Enemy { 
			sprite: Sprite::new(x, z, SpriteType::Enemy, sprite_assets),
			start: (x, z),
			end: (endx, endz)
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
