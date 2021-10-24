extern crate glutin;
extern crate winit;

use gl::types::*;
use glutin::{Api, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget};
use winit::window::{Window, WindowBuilder};

use std::mem::size_of_val;
use std::os::raw::c_void;
use std::ptr;
use std::time::Instant;

use crate::shader::Shader;

#[deny(unsafe_code)]
mod game;
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
	};

	event_loop.run(move |event: Event<()>, window_target, control_flow| {
		*control_flow = app.process_events(event, window_target);
	});
}

struct App {
	shader: Shader,
	window_ctx: WindowContext,

	start_time: Instant,
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
				self.window_ctx.window().request_redraw();
			}
			Event::RedrawRequested(_) => {
				self.render();
			}
			_ => {}
		}

		ControlFlow::Wait
	}

	/// Update app state. Timers, positions, etc.
	fn update(&mut self) {}

	fn render(&mut self) {
		unsafe { render(self) }
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

	let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

	let mut vao = 0u32;
	gl::GenVertexArrays(1, &mut vao as *mut _);
	gl::BindVertexArray(vao);

	let mut vbo: u32 = 0;
	gl::GenBuffers(1, &mut vbo as *mut _);

	gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
	gl::BufferData(
		gl::ARRAY_BUFFER,
		size_of_val(&vertices) as isize,
		vertices.as_ptr() as *const _,
		gl::STATIC_DRAW,
	);

	gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, ptr::null());
	gl::EnableVertexAttribArray(0);

	gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	gl::BindVertexArray(0);

	gl::BindVertexArray(vao);
	gl::DrawArrays(gl::TRIANGLES, 0, 3);

	gl::DeleteVertexArrays(1, &mut vao as *mut _);
	gl::DeleteBuffers(1, &mut vbo as *mut _);

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
