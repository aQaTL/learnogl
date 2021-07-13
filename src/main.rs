use gl::{types::*, *};
use glutin::{Api, ContextWrapper, GlProfile, GlRequest, PossiblyCurrent};
use std::os::raw::c_void;
use std::str::from_utf8;
use std::time::Instant;
use std::{mem::size_of_val, ptr};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

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

uniform vec4 triangleColor;

void main()
{
	FragColor = triangleColor;
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
					app.window_ctx.window().request_redraw();
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
			DeleteVertexArrays(1, &mut self.vao as *mut _);
			DeleteBuffers(1, &mut self.vbo as *mut _);
		}
	}
}

unsafe fn redraw(app: &mut App) {
	ClearColor(0.2, 0.3, 0.3, 1.0);
	Clear(COLOR_BUFFER_BIT);

	let elapsed_millis = app.start_time.elapsed().as_micros();
	let green = (elapsed_millis as f32).sin() / 2.0 + 0.5;
	dbg!(green);

	let vertex_color_location =
		GetUniformLocation(app.shader.0, "triangleColor\0".as_ptr() as *const GLchar);

	// Render
	app.shader.bind();

	Uniform4f(vertex_color_location, 1.0, green, 0.0, 0.5);

	BindVertexArray(app.vao);
	DrawArrays(TRIANGLES, 0, 3);

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
			&(source.as_ptr() as *const GLchar) as *const *const GLchar,
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
			panic!(
				"[Shader compile]: Failed to compile {} shader: {}",
				match s_type {
					VERTEX_SHADER => "vertex",
					FRAGMENT_SHADER => "fragment",
					_ => unimplemented!(),
				},
				from_utf8(&info_log).unwrap()
			);
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
