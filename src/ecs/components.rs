use iota::*;
iota! {
	pub const NONE: u64 = 1 << iota;
			| POSITION
			| SIZE
			| VELOCITY
			| SPRITE
			| PLAYER
			| JUMP_STATE
}

pub mod entity {
	use super::*;
	pub const PLAYER: u64 = POSITION | SIZE | VELOCITY | SPRITE | super::PLAYER | JUMP_STATE;
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
	pub tex_idx: usize,
}

#[derive(Copy, Clone, Default)]
pub struct Player {
	pub state: JumpState,
	pub max_jump_count: u8,
}

#[derive(Copy, Clone)]
pub enum JumpState {
	Standing,
	//jump count
	Jumping(u8),
}

impl Default for JumpState {
	fn default() -> Self {
		JumpState::Standing
	}
}

impl Default for Velocity {
	fn default() -> Self {
		Velocity {
			velocity: [0.0; 3].into(),
			acceleration: [0.0; 3].into(),
		}
	}
}
