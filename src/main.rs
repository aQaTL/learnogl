#![feature(const_fn)]

use std::path::Path;

use crate::game::GameFaze;
use crate::input::KeyState;
use glium::backend::Facade;
use glium::{
	glutin::*,
	program,
	texture::{RawImage2d, SrgbTexture2d},
	uniforms, Surface,
};
use imgui::im_str;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

pub mod game;
pub mod input;
pub mod renderer;

fn main() {
	let mut events_loop = glium::glutin::EventsLoop::new();

	let display = {
		let window = glium::glutin::WindowBuilder::new()
			.with_dimensions((800, 600).into())
			.with_title(env!("CARGO_PKG_NAME"));

		let context = glium::glutin::ContextBuilder::new()
			.with_gl_profile(GlProfile::Core)
			.with_vsync(false);

		glium::Display::new(window, context, &events_loop).unwrap()
	};

	let window = display.gl_window();
	let window = window.window();
	window.hide_cursor(true);
	if let Err(err) = window.grab_cursor(true) {
		println!("Failed to grab the cursor: {}", err);
	}

	println!("{:?}", display.get_opengl_version());
	println!("{:?}", display.get_opengl_vendor_string());
	println!("{:?}", display.get_opengl_renderer_string());
	println!();

	let (vertex_buffer, index_buffer) = renderer::create_rect_vb(&display).unwrap();

	let program = program!(
	&display,
	420 => {
		vertex: include_str!("shaders/vertex.glsl"),
		fragment: include_str!("shaders/fragment.glsl"),
	})
	.unwrap();

	let mut camera = input::Camera::new();
	input::toggle_mouse_grab(&mut camera, window);

	let mut keys = input::Keys::new();

	let mut imgui = imgui::Context::create();
	imgui.set_ini_filename(None);

	let mut platform = WinitPlatform::init(&mut imgui);
	{
		let gl_window = display.gl_window();
		let window = gl_window.window();
		platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
	}

	imgui
		.fonts()
		.add_font(&[imgui::FontSource::DefaultFontData {
			config: Some(imgui::FontConfig {
				size_pixels: 13.0,
				..imgui::FontConfig::default()
			}),
		}]);

	let mut gui_renderer = imgui_glium_renderer::GliumRenderer::init(&mut imgui, &display).unwrap();

	let mut game = game::Game::new(load_texture("images/box.png", &display).unwrap());

	game.set_faze(GameFaze::GameRunning);

	let mut running = true;
	while running {
		game.clock.update();

		{
			let gl_window = display.gl_window();
			let window = gl_window.window();
			let io = imgui.io_mut();
			platform
				.prepare_frame(io, &window)
				.expect("Failed to start frame");
			io.update_delta_time(game.clock.last_frame);
		}

		keys.update();

		events_loop.poll_events(|event| {
			platform.handle_event(imgui.io_mut(), &window, &event);
			match event {
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
						keys.0[key_code as usize] = match key.state {
							ElementState::Released => KeyState::Released,
							ElementState::Pressed => KeyState::JustPressed,
						};
					}
					None => (),
				},
				Event::DeviceEvent {
					event: DeviceEvent::MouseMotion { delta: (dx, dy) },
					..
				} => {
					if game.debug_mode {
						input::camera_process_mouse_input(&mut camera, dx as f32, dy as f32);
					}
				}
				_ => (),
			}
		});

		if game.debug_mode {
			input::camera_process_movement_input(&mut camera, &keys, game.clock.delta_time);
		}
		input::camera_process_shortcuts(&display, &mut camera, &keys, game.clock.delta_time);

		// Game update
		game.process_keyboard_input(&keys);
		game.update();

		let mut frame = display.draw();

		frame.clear_color_and_depth((0.2, 0.3, 0.3, 1.0), 1.0);

		match game.faze {
			GameFaze::TitleScreen => (),
			GameFaze::GameRunning => {
				renderer::render(
					&mut frame,
					&program,
					&game,
					&camera,
					&vertex_buffer,
					&index_buffer,
				)
				.unwrap();

				let ui = imgui.frame();
				ui.window(im_str!("learnogl"))
					.size([300.0, 120.0], imgui::Condition::FirstUseEver)
					.build(|| {
						ui.text(im_str!("Player pos: {:?}", game.tower.player.pos));
						ui.text(im_str!("Camera pos: {:?}", camera.pos));
					});
				gui_renderer
					.render(&mut frame, ui.render())
					.expect("imgui renderer fail");
			}
			GameFaze::DeathScreen => (),
		}

		frame.finish().unwrap();
	}
}

fn load_texture<P: AsRef<Path>, F: Facade>(
	path: P,
	facade: &F,
) -> Result<SampledSrgbTexture2d, glium::texture::TextureCreationError> {
	let image = image::open(path).unwrap().to_rgba();
	let image_dimensions = image.dimensions();
	let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
	let texture = SrgbTexture2d::new(facade, image)?;
	Ok(SampledSrgbTexture2d {
		tex: texture,
		sampler_behavior: uniforms::SamplerBehavior {
			minify_filter: uniforms::MinifySamplerFilter::Nearest,
			magnify_filter: uniforms::MagnifySamplerFilter::Nearest,
			..Default::default()
		},
	})
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
