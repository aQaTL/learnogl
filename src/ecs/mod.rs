pub mod components;

use components::*;
use std::collections::HashMap;
use crate::SampledSrgbTexture2d;

pub struct World {
	pub entities: Vec<u64>,

	pub positions: Vec<Position>,
	pub sizes: Vec<Size>,
	pub velocities: Vec<Velocity>,
	pub sprites: Vec<Sprite>,
	pub players: Vec<Player>,

	pub textures: HashMap<String, SampledSrgbTexture2d>,
}

const INIT_CAPACITY: usize = 100;

impl World {
	pub fn new() -> Self {
		World {
			entities: vec![components::NONE; INIT_CAPACITY],
			positions: vec![[0.0, 0.0, 0.0].into(); INIT_CAPACITY],
			sizes: vec![[0.0, 0.0, 0.0].into(); INIT_CAPACITY],
			velocities: vec![Default::default(); INIT_CAPACITY],
			sprites: vec![Default::default(); INIT_CAPACITY],
			players: vec![Default::default(); INIT_CAPACITY],

			textures: HashMap::new(),
		}
	}

	pub fn new_entity(&mut self, mask: Option<u64>) -> usize {
		for (entity_idx, entity) in self.entities.iter_mut().enumerate() {
			if *entity == components::NONE {
				if let Some(mask) = mask {
					*entity = mask;
				}
				return entity_idx;
			}
		}
		//TODO maybe just log the vecs reallocation?
		panic!("no more entities left");
	}

	#[inline]
	pub fn remove_entity(&mut self, entity: u64) {
		self.entities[entity as usize] = components::NONE;
	}

//	pub fn for_each<F: FnMut>(&mut self, mask: Mask, mut f: F) {
//		for entity in self.entities.iter_mut().filter(|e| e & mask == mask) {
//			f.call_mut(self);
//		}
//	}
}

pub mod systems {
	use super::*;

	fn player_input(world: &mut World) {

	}
}