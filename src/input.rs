use glium::glutin::{self, VirtualKeyCode, EventsLoop};

pub struct Input {
	pub camera: Camera,
	pub keys: Keys,
}

pub struct Keys(pub [KeyState; 161]);

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
	Released,
	Pressed,
	JustPressed,
}

impl Keys {
	pub fn new() -> Self {
		Keys([KeyState::Released; 161])
	}

	///Changes keys that are JustPressed to Pressed
	pub fn update(&mut self) {
		self.0
			.iter_mut()
			.filter(|key| **key == KeyState::JustPressed)
			.for_each(|key| *key = KeyState::Pressed);
	}

	pub fn is_just_pressed(&self, key: VirtualKeyCode) -> bool {
		self.0[key as usize] == KeyState::JustPressed
	}
}

impl std::ops::Index<VirtualKeyCode> for Keys {
	type Output = bool;

	fn index(&self, key: VirtualKeyCode) -> &Self::Output {
		match self.0[key as usize] {
			KeyState::Released => &false,
			KeyState::JustPressed | KeyState::Pressed => &true,
		}
	}
}

pub struct Camera {
	pub pos: glm::Vec3,
	pub front: glm::Vec3,
	pub up: glm::Vec3,

	pub movement_speed: f32,
	pub sensitivity: f32,

	pub yaw: f32,
	pub pitch: f32,

	pub fov: f32,

	pub mouse_grabbed: bool,
}

impl Camera {
	pub fn new() -> Self {
		Camera {
			pos: glm::Vec3::new(0.0, 0.0, 0.1),
			front: glm::Vec3::new(0.0, 0.0, -1.0),
			up: glm::Vec3::new(0.0, 1.0, 0.0),

			movement_speed: 2.0,
			sensitivity: 0.1,

			yaw: -90.0,
			pitch: 0.0,

			fov: 45.0,

			mouse_grabbed: false,
		}
	}

	pub(crate) fn view(&self) -> glm::Mat4 {
		glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
	}
}

pub fn toggle_mouse_grab(camera: &mut Camera, window: &glutin::Window) {
	camera.mouse_grabbed = !camera.mouse_grabbed;
	window.hide_cursor(camera.mouse_grabbed);
	if let Err(err) = window.grab_cursor(camera.mouse_grabbed) {
		println!("Failed to ungrab the cursor: {}", err);
	}
}

pub fn camera_process_mouse_input(camera: &mut Camera, mut dx: f32, mut dy: f32) {
	dx *= camera.sensitivity;
	dy *= -camera.sensitivity;

	camera.yaw += dx as f32;
	camera.pitch += dy as f32;

	match camera.pitch {
		n if n > 89.0 => camera.pitch = 89.0,
		n if n < -89.0 => camera.pitch = -89.0,
		_ => (),
	}

	let front = glm::Vec3::new(
		radians(camera.yaw).cos() * radians(camera.pitch).cos(),
		radians(camera.pitch).sin(),
		radians(camera.yaw).sin() * radians(camera.pitch).cos(),
	);
	camera.front = glm::normalize(&front);
}

pub fn camera_process_movement_input(camera: &mut Camera, keys: &Keys, delta_time: f32) {
	use glium::glutin::VirtualKeyCode::*;
	if keys[W] {
		camera.pos = camera.pos + (camera.front * (camera.movement_speed * delta_time));
	}
	if keys[S] {
		camera.pos = camera.pos - (camera.front * (camera.movement_speed * delta_time));
	}
	if keys[A] {
		camera.pos = camera.pos
			- (glm::normalize(&glm::cross::<f32, glm::U3>(&camera.front, &camera.up))
				* (camera.movement_speed * delta_time));
	}
	if keys[D] {
		camera.pos = camera.pos
			+ (glm::normalize(&glm::cross::<f32, glm::U3>(&camera.front, &camera.up))
				* (camera.movement_speed * delta_time));
	}
	if keys[Space] {
		camera.pos = camera.pos + (camera.up * (camera.movement_speed * delta_time));
	}
	if keys[LShift] {
		camera.pos = camera.pos - (camera.up * (camera.movement_speed * delta_time));
	}
}

pub fn camera_process_shortcuts(
	display: &glium::Display,
	camera: &mut Camera,
	keys: &Keys,
	_delta_time: f32,
) {
	use glium::glutin::VirtualKeyCode::*;
	if keys.is_just_pressed(Tab) {
		let window = display.gl_window();
		let window = window.window();
		toggle_mouse_grab(camera, window);
	}
}

const fn radians(degrees: f32) -> f32 {
	degrees * (std::f32::consts::PI / 180.0)
}
