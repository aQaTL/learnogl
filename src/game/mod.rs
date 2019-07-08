use std::time::Instant;

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
