pub mod components;

use components::*;
use crate::{SampledSrgbTexture2d, input, renderer};
use crate::game::GameClock;
use crate::input::KeyState;
use glium::{uniform, glutin::*, Surface, glutin};
use imgui::im_str;

pub struct World {
	pub entities: Vec<u64>,

	pub positions: Vec<Position>,
	pub sizes: Vec<Size>,
	pub velocities: Vec<Velocity>,
	pub sprites: Vec<Sprite>,
	pub players: Vec<Player>,
	pub jump_states: Vec<JumpState>,

	pub clock: GameClock,
	pub textures: Vec<SampledSrgbTexture2d>,

	pub input: input::Input,
	pub renderer: renderer::Renderer,

	pub imgui: renderer::Imgui,
}

impl World {
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

	#[inline]
	pub fn for_each(&mut self, mask: u64) -> impl Iterator<Item=(usize, &u64)> {
		self.entities.iter().filter(move |&&e| e & mask == mask).enumerate()
	}
}

trait ComponentGetter<T> {
	fn get(&self, idx: usize) -> T;
	fn get_mut(&mut self, idx: usize) -> &mut T;
}

impl ComponentGetter<Position> for World {
	#[inline]
	fn get(&self, idx: usize) -> Position {
		self.positions[idx]
	}

	#[inline]
	fn get_mut(&mut self, idx: usize) -> &mut Position {
		&mut self.positions[idx]
	}
}

impl ComponentGetter<Velocity> for World {
	#[inline]
	fn get(&self, idx: usize) -> Velocity {
		self.velocities[idx]
	}

	#[inline]
	fn get_mut(&mut self, idx: usize) -> &mut Velocity {
		&mut self.velocities[idx]
	}
}

impl World {
	pub fn update_clocks(&mut self) {
		self.clock.update();

		let window = self.renderer.display.gl_window();

		//TODO maybe this shouldn't be here
		let io = self.imgui.ctx.io_mut();
		self.imgui.platform
			.prepare_frame(io, window.window())
			.expect("Failed to start frame");
		io.update_delta_time(self.clock.last_frame);
	}

	pub fn handle_input_event(&mut self, event: glutin::Event) {
		let window = self.renderer.display.gl_window();
		self.imgui.platform.handle_event(self.imgui.ctx.io_mut(), window.window(), &event);

		match event {
			Event::DeviceEvent {
				event: DeviceEvent::MouseMotion { delta: (dx, dy) },
				..
			} => {
//					if self.debug_mode {
//				input::camera_process_mouse_input(&mut self.input.camera, dx as f32, dy as f32);
//					}
			}
			_ => (),
		}
	}

	pub fn update(&mut self) {
		input::camera_process_shortcuts(&self.renderer.display, &mut self.input.camera, &self.input.keys, self.clock.delta_time);
	}

	pub fn input(world: &mut World) {
//		for (idx, _) in world.for_each(entity::PLAYER) {
		for idx in 0..world.entities.len() {
			if world.entities[idx] & entity::PLAYER != entity::PLAYER {
				continue;
			}
			use glium::glutin::VirtualKeyCode::*;

			let jump_state = &mut world.jump_states[idx];
			let player = &world.players[idx];
			let velocity = &mut world.velocities[idx];
			let pos = &mut world.positions[idx];

			let keys = &world.input.keys;
			if keys[Space] {
				match jump_state {
					JumpState::Standing => {
						*jump_state = JumpState::Jumping(1);
//						*velocity[1] = self.tower.player.jump_accel;
					}
					JumpState::Jumping(ref mut count)
					if *count < player.max_jump_count =>
						{
							*count += 1;
							velocity.velocity.y = velocity.acceleration.y;
						}
					_ => (),
				}
			}
			if keys[Up] {
				pos.y += velocity.acceleration.y * world.clock.delta_time;
			}
			if keys[Down] {
				pos.y -= velocity.acceleration.y * world.clock.delta_time;
			}
			if keys[Left] {
				//self.tower.player.velocity.x -= self.tower.player.movement_accel * delta_time;
				pos.x -= velocity.acceleration.x * world.clock.delta_time;
			}
			if keys[Right] {
				//self.tower.player.velocity.x += self.tower.player.movement_accel * delta_time;
				pos.x += velocity.acceleration.x * world.clock.delta_time;
			}
		}
	}

	pub fn movement(world: &mut World) {
//		for (idx, _) in world.for_each(POSITION | VELOCITY) {
		let mask = POSITION | VELOCITY;
		for idx in 0..world.entities.len() {
			if world.entities[idx] & mask != mask {
				continue;
			}
			//TODO consider this:
//			let pos = &mut world.get::<Position>(idx);
			let mut pos: Position = world.get(idx);
			let mut velocity = &mut world.velocities[idx];

			velocity.velocity += velocity.acceleration;
			pos += velocity.velocity;
		}
	}

	pub fn render(world: &mut World) {
		// scaling -> rotation -> translation (function order is reversed!)
		let view = world.input.camera.view();

		//let (width, height) = frame.get_dimensions();
		let (width, height) = (800.0, 600.0);

		let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, 0.1, 100.0);

		let vp = projection * view;

		let draw_params = glium::DrawParameters {
			depth: glium::Depth {
				test: glium::DepthTest::IfLess,
				write: true,
				..Default::default()
			},
			..Default::default()
		};

		let mut frame = world.renderer.display.draw();
		frame.clear_color_and_depth((0.2, 0.3, 0.3, 1.0), 1.0);

		let ui = world.imgui.ctx.frame();
		let ui_window = ui.window(im_str!("learnogl"))
			.size([300.0, 120.0], imgui::Condition::FirstUseEver);

//		for (idx, _) in world.for_each(POSITION | SIZE | SPRITE) {
		let mask = POSITION | SIZE | SPRITE;
		for idx in 0..world.entities.len() {
			if world.entities[idx] & mask != mask {
				continue;
			}
			let pos = world.positions[idx];
			let size = world.sizes[idx];
			let texture = &world.textures[world.sprites[idx].tex_idx];

			let mut model = glm::Mat4::identity();
			model = glm::translate(&model, &pos);
			model = glm::translate(
				&model,
				&glm::Vec3::new(size.x / 2.0, size.y / 2.0, 0.0),
			);
			model = glm::scale(&model, &(size / 2.0));

			let transform = vp * model;

			let uniforms = uniform! {
				transform: *transform.as_ref(),
				tex: texture,
			};

			frame
				.draw(
					&world.renderer.rectangle_buffers.0,
					&world.renderer.rectangle_buffers.1,
					&world.renderer.program,
					&uniforms,
					&draw_params,
				)
				.unwrap();

			ui.text(im_str!("Player pos: {:?}", pos));
			ui.text(im_str!("Camera pos: {:?}", world.input.camera.pos));
		}

		ui_window.build(|| {});
		world.imgui.renderer
			.render(&mut frame, ui.render())
			.expect("imgui renderer fail");

		frame.finish().unwrap();
	}
}