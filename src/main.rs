#![feature(const_fn)]

use std::time::Instant;
use std::path::Path;

use glium::{*, glutin::*, index::PrimitiveType, texture::{RawImage2d, SrgbTexture2d}, vertex::VertexBufferAny};
use nalgebra_glm as glm;

mod game;

fn main() {
	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_dimensions((800, 600).into())
		.with_title(env!("CARGO_PKG_NAME"));

	let context = glium::glutin::ContextBuilder::new()
		.with_gl_profile(GlProfile::Core)
		.with_vsync(false);

	let display = glium::Display::new(window, context, &events_loop).unwrap();
	{
		let window = display.gl_window();
		let window = window.window();
		window.hide_cursor(true);
		if let Err(err) = window.grab_cursor(true) {
			println!("Failed to grab the cursor: {}", err);
		}
	}

	println!("{:?}", display.get_opengl_version());
	println!("{:?}", display.get_opengl_vendor_string());
	println!("{:?}", display.get_opengl_renderer_string());
	println!();

	let vertex_buffer = match load_mesh(&display, Path::new("models/nanosuit/nanosuit.obj")) {
		Ok(vb) => vb,
		Err(err) => panic!(format!("Error loading nanosuit.obj: {:#?}", err)),
	};

	let _index_buffer = glium::IndexBuffer::new(
		&display,
		PrimitiveType::TrianglesList,
		&[0u16, 1, 2, 0, 3, 2, 0],
	)
	.unwrap();

	let program = program!(
	&display,
	420 => {
		vertex: include_str!("shaders/vertex.glsl"),
		fragment: include_str!("shaders/fragment.glsl"),
	})
	.unwrap();

	//let image = image::open("images/cyberpunk_2077_car.jpg")
	let image = image::open("images/box.png").unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = SrgbTexture2d::new(&display, image).unwrap();

	let texture = SampledSrgbTexture2d {
		tex: texture,
		sampler_behavior: uniforms::SamplerBehavior {
			minify_filter: uniforms::MinifySamplerFilter::Nearest,
			magnify_filter: uniforms::MagnifySamplerFilter::Nearest,
			.. Default::default()
		},
	};

	let cubes = [
		GenericCube { pos: glm::Vec3::new(0.0, 0.0, 0.0), texture: &texture },
		GenericCube { pos: glm::Vec3::new(2.0, 5.0, -15.0), texture: &texture },
		GenericCube { pos: glm::Vec3::new(-1.5, -2.2, -2.5), texture: &texture },
		GenericCube { pos: glm::Vec3::new(-3.8, -2.0, -12.3), texture: &texture },
		GenericCube { pos: glm::Vec3::new(2.4, -0.4, -3.5), texture: &texture },
		GenericCube { pos: glm::Vec3::new(-1.7, 3.0, -7.5), texture: &texture },
		GenericCube { pos: glm::Vec3::new(1.3, -2.0, -2.5), texture: &texture },
		GenericCube { pos: glm::Vec3::new(1.5, 2.0, -2.5), texture: &texture },
		GenericCube { pos: glm::Vec3::new(1.5, 0.2, -1.5), texture: &texture },
		GenericCube { pos: glm::Vec3::new(-1.3, 1.0, -1.5), texture: &texture },
	];

	let mut camera = Camera::default();

	let mut keys = Keys([false; 161]);

	let start_time = Instant::now();
	let mut program_time: f32;

	let mut last_frame = Instant::now();
	let mut delta_time: f32;

	let mut game = game::Game::new();

	let mut running = true;
	while running {
		delta_time = last_frame.elapsed().as_micros() as f32 / 1_000_000.0;
		last_frame = Instant::now();

		program_time = start_time.elapsed().as_micros() as f32 / 1_000_000.0;

		game.update(delta_time);

		events_loop.poll_events(|event| match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				running = false;
			}
			Event::DeviceEvent {
				event: DeviceEvent::Key(key),
				..
			} => match key.virtual_keycode {
				Some(key_code) => {
					keys[key_code] = match key.state {
						ElementState::Pressed => true,
						ElementState::Released => false,
					};
				}
				None => (),
			},
			Event::DeviceEvent {
				event: DeviceEvent::MouseMotion { delta: (dx, dy) },
				..
			} => {
				camera_process_mouse_input(&mut camera, dx as f32, dy as f32);
			}
			_ => (),
		});

		camera_process_keyboard_input(&mut camera, &keys, delta_time);

		// scaling -> rotation -> translation

		let view = camera.view();

		let window_size = display.gl_window().window().get_inner_size().unwrap();
		let projection = glm::perspective(
			radians(camera.fov),
			(window_size.width / window_size.height) as f32,
			0.1,
			100.0,
		);

		let vp = projection * view;

		let mut frame = display.draw();

		frame.clear_color_and_depth((0.2, 0.3, 0.3, 1.0), 1.0);

		let draw_params = DrawParameters {
			depth: Depth {
				test: DepthTest::IfLess,
				write: true,
				..Default::default()
			},
			..Default::default()
		};

		let mut model = glm::Mat4::identity();
		model = glm::translate(&model, &cubes[0].pos);

		let transform = vp * model;

		let uniforms = uniform! {
			transform: *transform.as_ref(),
			tex: cubes[0].texture,
		};

		frame
			.draw(
				&vertex_buffer,
				&index::NoIndices(PrimitiveType::TrianglesList),
				&program,
				&uniforms,
				&draw_params,
			)
			.unwrap();

		frame.finish().unwrap();
	}
}

#[derive(Copy, Clone)]
struct Vertex {
	pos: [f32; 3],
	normal: [f32; 3],
	tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, normal, tex_coords);

#[allow(dead_code)]
const fn radians(degrees: f32) -> f32 {
	degrees * (std::f32::consts::PI / 180.0)
}

struct Keys([bool; 161]);

impl std::ops::Index<VirtualKeyCode> for Keys {
	type Output = bool;

	fn index(&self, key: VirtualKeyCode) -> &Self::Output {
		&self.0[key as usize]
	}
}

impl std::ops::IndexMut<VirtualKeyCode> for Keys {
	fn index_mut(&mut self, key: VirtualKeyCode) -> &mut Self::Output {
		&mut self.0[key as usize]
	}
}

struct Camera {
	pos: glm::Vec3,
	front: glm::Vec3,
	up: glm::Vec3,

	movement_speed: f32,
	sensitivity: f32,

	yaw: f32,
	pitch: f32,

	fov: f32,
}

impl Camera {
	fn view(&self) -> glm::Mat4 {
		glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
	}
}

impl Default for Camera {
	fn default() -> Self {
		Camera {
			pos: glm::Vec3::new(0.0, 0.0, 3.0),
			front: glm::Vec3::new(0.0, 0.0, -1.0),
			up: glm::Vec3::new(0.0, 1.0, 0.0),

			movement_speed: 2.0,
			sensitivity: 0.1,

			yaw: -90.0,
			pitch: 0.0,

			fov: 45.0,
		}
	}
}

fn camera_process_mouse_input(camera: &mut Camera, mut dx: f32, mut dy: f32) {
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

fn camera_process_keyboard_input(camera: &mut Camera, keys: &Keys, delta_time: f32) {
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

pub struct SampledSrgbTexture2d {
	pub tex: SrgbTexture2d,
	pub sampler_behavior: uniforms::SamplerBehavior,
}

impl uniforms::AsUniformValue for &SampledSrgbTexture2d {
	fn as_uniform_value(&self) -> uniforms::UniformValue {
		uniforms::UniformValue::SrgbTexture2d(&self.tex, Some(self.sampler_behavior))
	}
}

pub struct GenericCube<'a> {
	pub pos: glm::Vec3,
	pub texture: &'a SampledSrgbTexture2d,
}

fn load_mesh(display: &glium::Display, path: &Path) -> Result<VertexBufferAny, tobj::LoadError> {
	#[derive(Copy, Clone)]
	struct Vertex {
		pos: [f32; 3],
		normals: [f32; 3],
		tex_coords: [f32; 2],
	}
	implement_vertex!(Vertex, pos, normals, tex_coords);

	let mut min_pos = [std::f32::INFINITY; 3];
	let mut pax_pos = [std::f32::NEG_INFINITY; 3];
	let mut vertex_data = Vec::new();

	match tobj::load_obj(path) {
		Ok((models, mats)) => {
			for model in &models {
				println!("Loading model: {}", model.name);

				let mesh = &model.mesh;
				vertex_data.reserve(mesh.indices.len());
				for idx in &mesh.indices {
					let i = *idx as usize;
					let pos = [
						mesh.positions[i * 3],
						mesh.positions[i * 3 + 1],
						mesh.positions[i * 3 + 2],
					];

					let normals = if !mesh.normals.is_empty() {
						[
							mesh.normals[i * 3],
							mesh.normals[i * 3 + 1],
							mesh.normals[i * 3 + 2],
						]
					} else {
						[0.0, 0.0, 0.0]
					};

					let tex_coords = if !mesh.texcoords.is_empty() {
						[
							mesh.texcoords[i * 2],
							mesh.texcoords[i * 2 + 1],
						]
					} else {
						[0.0, 0.0]
					};

					vertex_data.push(Vertex { pos, normals, tex_coords });
				}
			}

			let vb = glium::VertexBuffer::new(display, &vertex_data).unwrap();
			Ok(vb.into_vertex_buffer_any())
		}
		Err(e) => return Err(e),
	}
}
