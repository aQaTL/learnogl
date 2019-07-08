use std::time::Instant;

//pub struct Tower {
//	pub floors: u32,
//	pub stairs: u32,
//	pub width: f32,
//	pub gravity: f32,
//
//	pub scrolling_speed: f32,
//	pub current_level: f32,
//
//	pub platforms: Vec<Platform>,
//
//	pub player: Player,
//}
//
//pub struct Platform {
//	pub width: f32,
//}
//
//impl Tower {
//	pub fn new(player_tex: SampledSrgbTexture2d) -> Self {
//		Tower {
//			floors: 10,
//			stairs: 40,
//			width: 100.0,
//			gravity: 3.0,
//
//			scrolling_speed: 10.0,
//			current_level: 0.0,
//
//			platforms: vec![],
//
//			player: Player {
//				pos: glm::Vec3::new(0.0, 0.0, 0.0),
//				size: glm::Vec3::new(5.0, 5.0, 0.0),
//				velocity: glm::Vec3::new(0.0, 0.0, 0.0),
//				facing: Turn::South,
//				state: PlayerState::Standing,
//
//				max_jump_count: 2,
//				jump_accel: 40.0,
//				movement_accel: 100.0,
//
//				tex: player_tex,
//			},
//		}
//	}
//}

//impl Game {
//	pub fn new(player_tex: SampledSrgbTexture2d) -> Self {
//		Game {
//			faze: GameFaze::TitleScreen,
//			tower: Tower::new(player_tex),
//			score: 0.0,
//			clock: GameClock::default(),
//			debug_mode: false,
//			update_fn: Game::update_title_screen,
//		}
//	}

//	pub fn process_keyboard_input(&mut self, keys: &Keys) {
//		use glium::glutin::VirtualKeyCode::*;
//		if keys[Space] {
//			match self.tower.player.state {
//				PlayerState::Standing => {
//					self.tower.player.state = PlayerState::Jumping(1);
//					self.tower.player.velocity[1] = self.tower.player.jump_accel;
//				}
//				PlayerState::Jumping(ref mut count)
//					if *count < self.tower.player.max_jump_count =>
//				{
//					*count += 1;
//					self.tower.player.velocity[1] = self.tower.player.jump_accel;
//				}
//				_ => (),
//			}
//		}
//		if keys[Up] {
//			self.tower.player.pos.y += self.tower.player.movement_accel * self.clock.delta_time;
//		}
//		if keys[Down] {
//			self.tower.player.pos.y -= self.tower.player.movement_accel * self.clock.delta_time;
//		}
//		if keys[Left] {
//			//self.tower.player.velocity.x -= self.tower.player.movement_accel * delta_time;
//			self.tower.player.pos.x -= self.tower.player.movement_accel * self.clock.delta_time;
//			self.tower.player.facing = Turn::West;
//		}
//		if keys[Right] {
//			//self.tower.player.velocity.x += self.tower.player.movement_accel * delta_time;
//			self.tower.player.pos.x += self.tower.player.movement_accel * self.clock.delta_time;
//			self.tower.player.facing = Turn::East;
//		}
//	}
//
//	pub fn update(&mut self) {
//		(self.update_fn)(self);
//	}
//
//	fn update_title_screen(&mut self) {}
//
//	fn update_game(&mut self) {
//		self.tower.player.pos += self.tower.player.velocity * self.clock.delta_time;
//		self.tower.player.pos;
//	}
//
//	fn update_death_screen(&mut self) {}
//
//	pub fn set_faze(&mut self, faze: GameFaze) {
//		self.update_fn = match faze {
//			GameFaze::TitleScreen => Game::update_title_screen,
//			GameFaze::GameRunning => Game::update_game,
//			GameFaze::DeathScreen => Game::update_death_screen,
//		};
//		self.faze = faze;
//	}
//}

pub struct GameClock {
	pub start_time: Instant,
	pub program_time: f32,

	pub last_frame: Instant,
	pub delta_time: f32,
}

impl GameClock {
	pub(crate) fn update(&mut self) {
		self.delta_time = self.last_frame.elapsed().as_micros() as f32 / 1_000_000.0;
		self.last_frame = Instant::now();

		self.program_time = self.start_time.elapsed().as_micros() as f32 / 1_000_000.0;
	}
}

impl Default for GameClock {
	fn default() -> Self {
		GameClock {
			start_time: Instant::now(),
			program_time: 0.0,

			last_frame: Instant::now(),
			delta_time: 0.0,
		}
	}
}
