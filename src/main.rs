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

	let context = glium::glutin::ContextBuilder::new().with_gl_profile(GlProfile::Core);

	let display = glium::Display::new(window, context, &events_loop).unwrap();

	println!("{:?}", display.get_opengl_version());
	println!("{:?}", display.get_opengl_vendor_string());
	println!("{:?}", display.get_opengl_renderer_string());
	println!();

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

	let vertex_buffer = glium::VertexBuffer::new(&display, &verticies).unwrap();
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

	/*
	let mut transform: [[f32; 4]; 4] = [
		[0.5, 0.0, 0.0, 0.0],
		[0.0, 0.5, 0.0, 0.0],
		[0.0, 0.0, 0.5, 0.0],
		[-0.2, 0.4, 0.0, 1.0],
	];
	*/

	let mut keys: [bool; 100] = [false; 100];

	let mut last_frame = Instant::now();
	let mut delta_time: f32;

	let mut running = true;
	while running {
		delta_time = last_frame.elapsed().as_micros() as f32 / 1_000_000.0;
		last_frame = Instant::now();

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
			} => {
				if key.scancode >= keys.len() as u32 {
					println!("Unknown scancode: {:?}", key);
					return;
				}

				keys[key.scancode as usize] = match key.state {
					ElementState::Pressed => true,
					ElementState::Released => false,
				};
			}
			_ => (),
		});

		// scaling -> rotation -> translation

		let mut model = glm::Mat4::identity();
		model = glm::rotate(&model, radians(-55.0), &glm::Vec3::new(1.0, 0.0, 0.0));

		let mut view = glm::Mat4::identity();
		view = glm::translate(&view, &glm::Vec3::new(0.0, 0.0, -3.0));

		let window_size = display.gl_window().window().get_inner_size().unwrap();
		let projection = glm::perspective(
			radians(45.0),
			(window_size.width / window_size.height) as f32,
			0.1,
			100.0,
		);

		let transform = projection * view * model;

		let mut frame = display.draw();

		frame.clear_color(0.2, 0.3, 0.3, 1.0);

		let uniforms = uniform! {
			transform: *transform.as_ref(),
			tex: texture,
		};

		let draw_params = DrawParameters {
			..Default::default()
		};

		frame
			.draw(
				&vertex_buffer,
				&index_buffer,
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
	tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, tex_coords);

#[allow(dead_code)]
const fn radians(degrees: f32) -> f32 {
	degrees * (std::f32::consts::PI / 180.0)
}
