pub struct Game {
	faze: GameFaze,
	tower: Tower,

	score: f32,

	update_fn: UpdateFn,
}

type UpdateFn = fn(&mut Game, delta_time: f32);

#[derive(Debug)]
pub enum GameFaze {
	TitleScreen,
	GameRunning,
	DeathScreen,
}

pub struct Tower {
	floors: u32,
	stairs: u32,

	width: f32,

	scrolling_speed: f32,
	current_level: f32,
}

struct Stair {
	width: f32,
}

struct Player {
}

impl Tower {
	pub fn new() -> Self {
		Tower {
			floors: 10,
			stairs: 40,

			width: 100.0,

			scrolling_speed: 10.0,
			current_level: 0.0,
		}
	}
}

impl Game {
	pub fn new() -> Self {
		Game {
			faze: GameFaze::TitleScreen,
			tower: Tower::new(),
			score: 0.0,
			update_fn: Game::update_title_screen,
		}
	}

	pub fn update(&mut self, delta_time: f32) {
		(self.update_fn)(self, delta_time);
	}

	fn update_title_screen(&mut self, delta_time: f32) {
		println!("Updating {:?}", self.faze);
	}

	fn update_game(&mut self, delta_time: f32) {
		println!("Updating {:?}", self.faze);
	}

	fn update_death_screen(&mut self, delta_time: f32) {
		println!("Updating {:?}", self.faze);
	}
}
