extern crate glutin;
extern crate winit;

use gl::types::*;
use glutin::{Api, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::ffi::{CStr, CString, NulError};
use std::os::raw::c_void;
use std::str::from_utf8;
use std::time::Instant;
use std::{mem::size_of_val, ptr};

#[deny(unsafe_code)]
mod game;

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
		.with_vsync(true)
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

	gl::Viewport(0, 0, 1280, 720);

	let mut app = App {
		shader,
		vao,
		vbo,
		window_ctx,
		start_time: Instant::now(),
	};

	event_loop.run(
		move |event: Event<'_, ()>, _ev_loop_window_target, control_flow| {
			*control_flow = ControlFlow::Wait;

			match event {
				Event::WindowEvent {
					event: WindowEvent::CloseRequested,
					..
				} => {
					*control_flow = ControlFlow::Exit;
				}
				Event::WindowEvent {
					event: WindowEvent::Resized(new_size),
					..
				} => {
					let (width, height): (u32, u32) = new_size.into();
					gl::Viewport(0, 0, width as i32, height as i32);
				}
				Event::MainEventsCleared => {
					// app.window_ctx.window().request_redraw();
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
					app.window_ctx.window().request_redraw();
				}
				Event::RedrawRequested(_) => {
					redraw(&mut app);
				}
				_ => {}
			}
		},
	);
}

struct App {
	shader: Shader,
	vao: u32,
	vbo: u32,
	window_ctx: WindowContext,

	start_time: Instant,
}

impl Drop for App {
	fn drop(&mut self) {
		unsafe {
			gl::DeleteVertexArrays(1, &mut self.vao as *mut _);
			gl::DeleteBuffers(1, &mut self.vbo as *mut _);
		}
	}
}

unsafe fn redraw(app: &mut App) {
	gl::ClearColor(0.2, 0.3, 0.3, 1.0);
	gl::Clear(gl::COLOR_BUFFER_BIT);

	let elapsed_millis = app.start_time.elapsed().as_micros();
	let green = (elapsed_millis as f32).sin() / 2.0 + 0.5;
	dbg!(green);

	// Render
	app.shader.bind();

	app.shader
		.uniform4f("triangleColor\0", [1.0, green, 0.0, 0.5]);

	gl::BindVertexArray(app.vao);
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

struct Shader(pub(crate) u32);

impl Shader {
	fn bind(&self) {
		unsafe {
			gl::UseProgram(self.0);
		}
	}
}

unsafe fn new_shader(vertex_source: &CStr, fragment_source: &CStr) -> u32 {
	let compile_shader = |source: &CStr, s_type| {
		let shader = gl::CreateShader(s_type);
		gl::ShaderSource(
			shader,
			1,
			&(source.as_ptr() as *const GLchar) as *const *const GLchar,
			&(source.to_bytes_with_nul().len() as i32),
		);
		gl::CompileShader(shader);

		let (mut success, mut info_log) = (false, [0u8; 512]);
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success as *mut _ as *mut _);
		if !success {
			gl::GetShaderInfoLog(
				shader,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut _,
			);
			panic!(
				"[Shader compile]: Failed to compile {} shader: {}",
				match s_type {
					gl::VERTEX_SHADER => "vertex",
					gl::FRAGMENT_SHADER => "fragment",
					_ => unimplemented!(),
				},
				from_utf8(&info_log).unwrap()
			);
		}
		shader
	};

	let vertex_shader = compile_shader(vertex_source, gl::VERTEX_SHADER);
	let fragment_shader = compile_shader(fragment_source, gl::FRAGMENT_SHADER);

	let shader = gl::CreateProgram();
	gl::AttachShader(shader, vertex_shader);
	gl::AttachShader(shader, fragment_shader);
	gl::LinkProgram(shader);

	let (mut success, mut info_log) = (false, [0u8; 512]);
	gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success as *mut _ as *mut _);
	if !success {
		gl::GetProgramInfoLog(
			shader,
			512,
			ptr::null_mut(),
			info_log.as_mut_ptr() as *mut _,
		);
		println!(
			"ERROR::SHADER::PROGRAM::LINKING_FAILED: {}",
			from_utf8(&info_log).unwrap()
		);
	}

	gl::DeleteShader(vertex_shader);
	gl::DeleteShader(fragment_shader);

	shader
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteProgram(self.0) }
	}
}

#[derive(Debug)]
enum ShaderError {
	ProvidedStringContainsNullByte(NulError),
}

impl Shader {
	fn new(vertex_source: &str, fragment_source: &str) -> Result<Self, ShaderError> {
		let vertex_source =
			CString::new(vertex_source).map_err(ShaderError::ProvidedStringContainsNullByte)?;
		let fragment_source =
			CString::new(fragment_source).map_err(ShaderError::ProvidedStringContainsNullByte)?;
		Ok(Shader(unsafe {
			new_shader(&vertex_source, &fragment_source)
		}))
	}

	fn uniform4f(&self, name: &'static str, [a, b, c, d]: [f32; 4]) {
		#[cfg(debug_assertions)]
		{
			if name.as_bytes()[name.len() - 1] != b'\0' {
				eprintln!("Uniform name doesn't have null byte terminator");
			}
		}

		unsafe {
			let location = gl::GetUniformLocation(self.0, name.as_ptr() as *const GLchar);
			debug_assert!(
				location != -1,
				"Failed to get uniform location of \"{}\"",
				name
			);

			gl::Uniform4f(location, a, b, c, d);
		}
	}
}
