use crate::SampledSrgbTexture2d;
use std::rc::Rc;
use crate::game::PlayerState;

use iota::*;
iota! {
	pub const NONE: u64 = 1 << iota;
			| POSITION
			| SIZE
			| VELOCITY
			| SPRITE
			| PLAYER
}

pub type Position = glm::Vec3;

pub type Size = glm::Vec3;

#[derive(Copy, Clone)]
pub struct Velocity {
	pub velocity: glm::Vec3,
	pub acceleration: glm::Vec3,
}

#[derive(Clone, Default)]
pub struct Sprite {
	pub tex_name: String,
}

#[derive(Copy, Clone, Default)]
pub struct Player {
	pub state: PlayerState,
	pub max_jump_count: u8,
}

impl Default for Velocity {
	fn default() -> Self {
		Velocity {
			velocity: [0.0; 3].into(),
			acceleration: [0.0; 3].into(),
		}
	}
}
