#![feature(const_fn)]

use glium::glutin::*;
use glium::index::PrimitiveType;
use glium::texture::{RawImage2d, SrgbTexture2d};
use glium::*;
use nalgebra_glm as glm;
use std::time::Instant;

fn main() {
	let mut events_loop = glium::glutin::EventsLoop::new();
	let window = glium::glutin::WindowBuilder::new()
		.with_dimensions((800, 600).into())
		.with_title(env!("CARGO_PKG_NAME"));

	let context = glium::glutin::ContextBuilder::new()
		.with_gl_profile(GlProfile::Core)
		.with_vsync(true);

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

	/*
	let verticies = [
		Vertex {
			pos: [-1.0, 1.0, 0.0],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [1.0, 1.0, 0.0],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [1.0, -1.0, 0.0],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [-1.0, -1.0, 0.0],
			tex_coords: [0.0, 0.0],
		},
	];
	*/

	let vertices = [
		Vertex {
			pos: [-0.5, -0.5, -0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [0.5, -0.5, -0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [0.5, 0.5, -0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [0.5, 0.5, -0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [-0.5, 0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [-0.5, -0.5, -0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [-0.5, -0.5, 0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [0.5, -0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [0.5, 0.5, 0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [0.5, 0.5, 0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [-0.5, 0.5, 0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [-0.5, -0.5, 0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [-0.5, 0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [-0.5, 0.5, -0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [-0.5, -0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [-0.5, -0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [-0.5, -0.5, 0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [-0.5, 0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [0.5, 0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [0.5, 0.5, -0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [0.5, -0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [0.5, -0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [0.5, -0.5, 0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [0.5, 0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [-0.5, -0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [0.5, -0.5, -0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [0.5, -0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [0.5, -0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [-0.5, -0.5, 0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [-0.5, -0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [-0.5, 0.5, -0.5],
			tex_coords: [0.0, 1.0],
		},
		Vertex {
			pos: [0.5, 0.5, -0.5],
			tex_coords: [1.0, 1.0],
		},
		Vertex {
			pos: [0.5, 0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [0.5, 0.5, 0.5],
			tex_coords: [1.0, 0.0],
		},
		Vertex {
			pos: [-0.5, 0.5, 0.5],
			tex_coords: [0.0, 0.0],
		},
		Vertex {
			pos: [-0.5, 0.5, -0.5],
			tex_coords: [0.0, 1.],
		},
	];

	let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
	let index_buffer = glium::IndexBuffer::new(
		&display,
		PrimitiveType::TrianglesList,
		&[0u16, 1, 2, 0, 3, 2, 0],
	)
	.unwrap();

	let program = program!(
	&display,
	430 => {
		vertex: include_str!("shaders/vertex.glsl"),
		fragment: include_str!("shaders/fragment.glsl"),
	})
	.unwrap();

	//let image = image::open("images/cyberpunk_2077_car.jpg")
	let image = image::open("images/box.png").unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = SrgbTexture2d::new(&display, image).unwrap();

	let texture = texture
		.sampled()
		.magnify_filter(uniforms::MagnifySamplerFilter::Nearest)
		.minify_filter(uniforms::MinifySamplerFilter::Nearest);

	let cube_positions = [
		glm::Vec3::new(0.0, 0.0, 0.0),
		glm::Vec3::new(2.0, 5.0, -15.0),
		glm::Vec3::new(-1.5, -2.2, -2.5),
		glm::Vec3::new(-3.8, -2.0, -12.3),
		glm::Vec3::new(2.4, -0.4, -3.5),
		glm::Vec3::new(-1.7, 3.0, -7.5),
		glm::Vec3::new(1.3, -2.0, -2.5),
		glm::Vec3::new(1.5, 2.0, -2.5),
		glm::Vec3::new(1.5, 0.2, -1.5),
		glm::Vec3::new(-1.3, 1.0, -1.5),
	];

	let mut camera_pos = glm::Vec3::new(0.0, 0.0, 3.0);
	let mut camera_front = glm::Vec3::new(0.0, 0.0, -1.0);
	let camera_up = glm::Vec3::new(0.0, 1.0, 0.0);

	let movement_speed = 2.0;
	let sensitivity = 0.1;

	let mut yaw: f32 = -90.0;
	let mut pitch: f32 = 0.0;

	let mut keys = Keys([false; 161]);

	let start_time = Instant::now();
	let mut program_time: f32;

	let mut last_frame = Instant::now();
	let mut delta_time: f32;

	let mut running = true;
	while running {
		delta_time = last_frame.elapsed().as_micros() as f32 / 1_000_000.0;
		last_frame = Instant::now();

		program_time = start_time.elapsed().as_micros() as f32 / 1_000_000.0;

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
				event: DeviceEvent::MouseMotion {
					delta: (mut dx, mut dy),
				},
				..
			} => {
				dx *= sensitivity;
				dy *= -sensitivity;

				yaw += dx as f32;
				pitch += dy as f32;

				if pitch > 89.0 {
					pitch = 89.0;
				}
				if pitch < -89.0 {
					pitch = -89.0;
				}

				let front = glm::Vec3::new(
					radians(yaw).cos() * radians(pitch).cos(),
					radians(pitch).sin(),
					radians(yaw).sin() * radians(pitch).cos(),
				);
				camera_front = glm::normalize(&front);
			}
			_ => (),
		});

		{
			use glium::glutin::VirtualKeyCode::*;
			if keys[W] {
				camera_pos.z -= movement_speed * delta_time;
			}
			if keys[S] {
				camera_pos.z += movement_speed * delta_time;
			}
			if keys[A] {
				camera_pos.x -= movement_speed * delta_time;
			}
			if keys[D] {
				camera_pos.x += movement_speed * delta_time;
			}
			if keys[Space] {
				camera_pos.y += movement_speed * delta_time
			}
			if keys[LShift] {
				camera_pos.y -= movement_speed * delta_time
			}
		}

		// scaling -> rotation -> translation

		let view = glm::look_at(&camera_pos, &(camera_pos + camera_front), &camera_up);

		let window_size = display.gl_window().window().get_inner_size().unwrap();
		let projection = glm::perspective(
			radians(45.0),
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

		for (idx, pos) in cube_positions.iter().enumerate() {
			let mut model = glm::Mat4::identity();
			model = glm::translate(&model, pos);
			model = glm::rotate(
				&model,
				program_time * radians((20 * (idx + 0)) as f32),
				&glm::Vec3::new(1.0, 0.3, 0.5),
			);

			let transform = vp * model;

			let uniforms = uniform! {
				transform: *transform.as_ref(),
				tex: texture,
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
		}

		frame.finish().unwrap();
	}
}

#[derive(Copy, Clone)]
struct Vertex {
	pos: [f32; 3],
	tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, tex_coords);

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
