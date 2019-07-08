#![feature(const_fn)]
#![deny(unsafe_code)]

use std::path::Path;

use crate::game::GameClock;
use glium::backend::Facade;
use glium::{
	glutin::*,
	texture::{RawImage2d, SrgbTexture2d},
	uniforms,
};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use crate::ecs::{World, components};
use crate::input::KeyState;

pub mod game;
pub mod input;
pub mod renderer;
pub mod ecs;

fn main() {
	println!("Size of SrgbTexture2d: {}", std::mem::size_of::<glium::texture::srgb_texture2d::SrgbTexture2d>());
	println!("Size of TextureAny: {}", std::mem::size_of::<glium::texture::TextureAny>());
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

	let imgui = {
		let mut ctx = imgui::Context::create();
		ctx.set_ini_filename(None);

		let mut platform = WinitPlatform::init(&mut ctx);
		{
			let gl_window = display.gl_window();
			let window = gl_window.window();
			platform.attach_window(ctx.io_mut(), &window, HiDpiMode::Rounded);
		}

		ctx
			.fonts()
			.add_font(&[imgui::FontSource::DefaultFontData {
				config: Some(imgui::FontConfig {
					size_pixels: 13.0,
					..imgui::FontConfig::default()
				}),
			}]);

		let renderer = imgui_glium_renderer::GliumRenderer::init(&mut ctx, &display).unwrap();

		renderer::Imgui {
			ctx,
			platform,
			renderer,
		}
	};

	let input = {
		let mut camera = input::Camera::new();
		let window = display.gl_window();
		input::toggle_mouse_grab(&mut camera, window.window());
		input::Input {
			camera: camera,
			keys: input::Keys::new(),
		}
	};

	let renderer = renderer::Renderer::new(display).unwrap();

	let mut world = {
		const INIT_CAPACITY: usize = 100;
		World {
			//TODO use with_capacity
			entities: vec![components::NONE; INIT_CAPACITY],
			positions: vec![[0.0, 0.0, 0.0].into(); INIT_CAPACITY],
			sizes: vec![[0.0, 0.0, 0.0].into(); INIT_CAPACITY],
			velocities: vec![Default::default(); INIT_CAPACITY],
			sprites: vec![Default::default(); INIT_CAPACITY],
			players: vec![Default::default(); INIT_CAPACITY],
			jump_states: vec![Default::default(); INIT_CAPACITY],

			clock: GameClock::default(),

			textures: vec![],
			input: input,
			renderer: renderer,
			imgui: imgui,
		}
	};
	add_entities(&mut world);

	let mut running = true;
	while running {
		world.update_clocks();

//		if game.debug_mode {
//			input::camera_process_movement_input(&mut camera, &keys, game.clock.delta_time);
//		}
//		input::camera_process_shortcuts(&display, &mut camera, &keys, game.clock.delta_time);

		// Game update
//		game.process_keyboard_input(&keys);
//		game.update();

		//Input handling
		world.input.keys.update();
		events_loop.poll_events(|event| {
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
						world.input.keys.0[key_code as usize] = match key.state {
							ElementState::Released => KeyState::Released,
							ElementState::Pressed => KeyState::JustPressed,
						};
					}
					None => (),
				},

				_ => (),
			}

			world.handle_input_event(event);
		});

		world.update();

		ecs::World::input(&mut world);
		ecs::World::movement(&mut world);
		ecs::World::render(&mut world);
	}
}

fn handle_input() {

}

//TODO texture storage
fn add_entities(world: &mut World) {
	use crate::ecs::components::*;

	world.textures.push(
		load_texture("images/box.png", &world.renderer.display).unwrap(),
	);

	//Player
	let player = world.new_entity(Some(entity::PLAYER));
	world.positions[player] = glm::Vec3::new(0.0, 0.0, 0.0);
	world.sizes[player] = glm::Vec3::new(5.0, 5.0, 0.0);
	world.sprites[player] = Sprite { tex_idx: 0 };
	world.velocities[player] = Velocity {
		velocity: glm::Vec3::new(0.0, 0.0, 0.0),
		acceleration: [100.0, 40.0, 0.0].into(),
	};
	world.players[player] = Player {
		state: JumpState::Standing,
		max_jump_count: 2,
	};
	world.jump_states[player] = JumpState::Standing;
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
