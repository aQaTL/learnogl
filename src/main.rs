extern crate glutin;
extern crate winit;

use gl::types::*;
use glutin::{Api, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

use std::os::raw::c_void;
use std::time::Instant;

use crate::shader::Shader;
use glbuffer::{Vao, VertexAttrib, VertexLayout};

mod game;
mod glbuffer;
mod shader;

fn main() {
	unsafe { main_() }
}

type WindowContext = ContextWrapper<PossiblyCurrent, Window>;

unsafe fn main_() {
	let event_loop = EventLoop::new();

	let wb = WindowBuilder::new().with_inner_size(PhysicalSize::<u32>::from((1280_u32, 720_u32)));

	let window_ctx: WindowContext = glutin::ContextBuilder::new()
		.with_gl_profile(GlProfile::Core)
		.with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
		.with_vsync(false)
		.build_windowed(wb, &event_loop)
		.expect("Failed to initialise OpenGL")
		.make_current()
		.unwrap();

	gl::load_with(|s| window_ctx.get_proc_address(s) as *const _);
	gl::Viewport::load_with(|s| window_ctx.get_proc_address(s) as *const _);

	gl::DebugMessageCallback(Some(debug_msg_callback), std::ptr::null_mut());

	let shader = Shader::new(
		include_str!("shaders/vs.glsl"),
		include_str!("shaders/fs.glsl"),
	)
	.unwrap();

	gl::Viewport(0, 0, 1280, 720);

	let mut app = App {
		shader,
		window_ctx,

		start_time: Instant::now(),

		polygon_mode: false,
	};

	event_loop.run(move |event: Event<()>, window_target, control_flow| {
		*control_flow = app.process_events(event, window_target);
	});
}

struct App {
	shader: Shader,
	window_ctx: WindowContext,

	start_time: Instant,

	polygon_mode: bool,
}

impl App {
	fn process_events(
		&mut self,
		event: Event<()>,
		_window: &EventLoopWindowTarget<()>,
	) -> ControlFlow {
		match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => {
				return ControlFlow::Exit;
			}
			Event::WindowEvent {
				event: WindowEvent::Resized(new_size),
				..
			} => {
				let (width, height): (u32, u32) = new_size.into();
				unsafe {
					gl::Viewport(0, 0, width as i32, height as i32);
				}
			}
			Event::MainEventsCleared => {
				self.update();
				// self.window_ctx.window().request_redraw();
			}
			Event::DeviceEvent {
				event:
					DeviceEvent::Key(KeyboardInput {
						state: ElementState::Pressed,
						virtual_keycode: Some(VirtualKeyCode::Space),
						..
					}),
				..
			} => {
				println!("pressed space");
				self.redraw();
			}
			Event::RedrawRequested(_) => {
				self.render();
			}
			Event::DeviceEvent {
				event:
					DeviceEvent::Key(KeyboardInput {
						state: ElementState::Pressed,
						virtual_keycode: Some(VirtualKeyCode::F9),
						..
					}),
				..
			} => {
				self.polygon_mode = !self.polygon_mode;
				if self.polygon_mode {
					unsafe {
						gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
					}
				} else {
					unsafe {
						gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
					}
				}
				self.redraw();
			}

			_ => {}
		}

		ControlFlow::Wait
	}

	fn redraw(&self) {
		self.window_ctx.window().request_redraw();
	}

	/// Update app state. Timers, positions, etc.
	fn update(&mut self) {}

	fn render(&mut self) {
		unsafe { render(self) }
	}
}

#[repr(C)]
struct TriangleVertex {
	position: Vec3,
}

#[repr(C)]
struct Vec3(f32, f32, f32);

impl VertexLayout<1> for TriangleVertex {
	fn layout() -> [VertexAttrib; 1] {
		[VertexAttrib::new::<f32>(3)]
	}
}

unsafe fn render(app: &mut App) {
	gl::ClearColor(0.2, 0.3, 0.3, 1.0);
	gl::Clear(gl::COLOR_BUFFER_BIT);

	let elapsed_millis = app.start_time.elapsed().as_micros();
	let green = (elapsed_millis as f32).sin() / 2.0 + 0.5;
	dbg!(green);

	// Render
	app.shader.bind();

	app.shader
		.uniform4f("triangleColor\0", [1.0, green, 0.0, 0.5]);

	let vertices = [
		TriangleVertex {
			position: Vec3(-0.5, -0.5, 0.0),
		},
		TriangleVertex {
			position: Vec3(0.5, -0.5, 0.0),
		},
		TriangleVertex {
			position: Vec3(0.0, 0.5, 0.0),
		},
	];

	let vao = Vao::new_static(&vertices);
	vao.bind();

	gl::DrawArrays(gl::TRIANGLES, 0, 3);

	if let Err(err) = app.window_ctx.swap_buffers() {
		println!("swap_buffers err: {:?}", err);
	}
}

extern "system" fn debug_msg_callback(
	_source: u32,
	_gltype: u32,
	_id: u32,
	severity: u32,
	length: i32,
	message: *const GLchar,
	user_param: *mut c_void,
) {
	let (_param, msg) = unsafe {
		let _param = &mut *(user_param as *mut ());
		let msg = std::str::from_utf8_unchecked(std::slice::from_raw_parts::<u8>(
			message as *const _,
			length as usize,
		));
		(_param, msg)
	};
	println!("[DEBUG ({})]: {}", severity, msg);
}
