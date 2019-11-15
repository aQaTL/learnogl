use gl::{types::*, *};
use glutin::{Api, GlProfile, GlRequest};
use std::os::raw::c_void;
use std::str::from_utf8;
use std::{
	mem::{size_of, size_of_val},
	ptr,
};
use winit::{Event, WindowEvent};

#[deny(unsafe_code)]
mod game;

static mut RUNNING: bool = true;

fn main() {
	unsafe { main_() }
}

unsafe fn main_() {
	let mut event_loop = winit::EventsLoop::new();

	let wb = winit::WindowBuilder::new().with_dimensions((1280, 720).into());

	let window_ctx = glutin::ContextBuilder::new()
		.with_gl_profile(GlProfile::Core)
		.with_gl(GlRequest::Specific(Api::OpenGl, (4, 3)))
		.with_vsync(true)
		.build_windowed(wb, &event_loop)
		.expect("Failed to initialise OpenGL")
		.make_current()
		.unwrap();

	load_with(|s| window_ctx.get_proc_address(s) as *const _);
	gl::Viewport::load_with(|s| window_ctx.get_proc_address(s) as *const _);

	DebugMessageCallback(Some(debug_msg_callback), std::ptr::null_mut());

	let vs = "#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
";

	let fs = "#version 330 core
out vec4 FragColor;

void main()
{
	FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}
\0";

	let shader = new_shader(vs, fs);

	let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

	let mut vao = 0u32;
	GenVertexArrays(1, &mut vao as *mut _);
	BindVertexArray(vao);

	let mut vbo: u32 = 0;
	GenBuffers(1, &mut vbo as *mut _);

	BindBuffer(ARRAY_BUFFER, vbo);
	BufferData(
		ARRAY_BUFFER,
		size_of_val(&vertices) as isize,
		vertices.as_ptr() as *const _,
		STATIC_DRAW,
	);

	VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, ptr::null());
	EnableVertexAttribArray(0);

	BindBuffer(ARRAY_BUFFER, 0);
	BindVertexArray(0);

	Viewport(0, 0, 1280, 720);

	while RUNNING {
		event_loop.poll_events(|event: Event| match event {
			Event::WindowEvent {
				event: WindowEvent::CloseRequested,
				..
			} => RUNNING = false,
			Event::WindowEvent {
				event: WindowEvent::Resized(new_size),
				..
			} => {
				let (width, height): (u32, u32) = new_size.into();
				gl::Viewport(0, 0, width as i32, height as i32);
			}
			_ => {}
		});

		ClearColor(0.2, 0.3, 0.3, 1.0);
		Clear(COLOR_BUFFER_BIT);

		// Render
		shader.bind();
		BindVertexArray(vao);
		DrawArrays(TRIANGLES, 0, 3);

		if let Err(err) = window_ctx.swap_buffers() {
			println!("swap_buffers err: {:?}", err);
		}
	}

	drop(shader);
	DeleteVertexArrays(1, &mut vao as *mut _);
	DeleteBuffers(1, &mut vbo as *mut _);
}

extern "system" fn debug_msg_callback(
	_source: u32,
	_gltype: u32,
	_id: u32,
	severity: u32,
	length: i32,
	message: *const i8,
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
			UseProgram(self.0);
		}
	}
}

unsafe fn new_shader(vertex_source: &str, fragment_source: &str) -> Shader {
	let compile_shader = |source: &str, s_type| {
		let shader = CreateShader(s_type);
		ShaderSource(
			shader,
			1,
			&(source.as_ptr() as *const i8) as *const _,
			&(source.len() as i32),
		);
		CompileShader(shader);

		let (mut success, mut info_log) = (false, [0u8; 512]);
		GetShaderiv(shader, COMPILE_STATUS, &mut success as *mut _ as *mut _);
		if !success {
			GetShaderInfoLog(
				shader,
				512,
				ptr::null_mut(),
				info_log.as_mut_ptr() as *mut _,
			);
			panic!(format!(
				"[Shader compile]: Failed to compile {} shader: {}",
				match s_type {
					VERTEX_SHADER => "vertex",
					FRAGMENT_SHADER => "fragment",
					_ => unimplemented!(),
				},
				from_utf8(&info_log).unwrap()
			));
		}
		shader
	};

	let vertex_shader = compile_shader(vertex_source, VERTEX_SHADER);
	let fragment_shader = compile_shader(fragment_source, FRAGMENT_SHADER);

	let shader = CreateProgram();
	AttachShader(shader, vertex_shader);
	AttachShader(shader, fragment_shader);
	LinkProgram(shader);

	let (mut success, mut info_log) = (false, [0u8; 512]);
	GetProgramiv(shader, LINK_STATUS, &mut success as *mut _ as *mut _);
	if !success {
		GetProgramInfoLog(
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

	DeleteShader(vertex_shader);
	DeleteShader(fragment_shader);

	Shader(shader)
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { DeleteProgram(self.0) }
	}
}
