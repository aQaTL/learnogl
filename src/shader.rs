use std::ffi::{CStr, CString};
use std::str;

use gl::types::*;

pub struct Shader(pub(crate) u32);

#[derive(Debug)]
pub enum ShaderError {
	ProvidedStringContainsNullByte(std::ffi::NulError),
	FailedToCompileVertexShader(String),
	FailedToCompileFragmentShader(String),
	FailedToLinkProgram(String),
}

impl Drop for Shader {
	fn drop(&mut self) {
		unsafe { gl::DeleteProgram(self.0) }
	}
}

impl Shader {
	pub fn new(vertex_source: &str, fragment_source: &str) -> Result<Self, ShaderError> {
		let vertex_source =
			CString::new(vertex_source).map_err(ShaderError::ProvidedStringContainsNullByte)?;
		let fragment_source =
			CString::new(fragment_source).map_err(ShaderError::ProvidedStringContainsNullByte)?;
		unsafe { new_shader(&vertex_source, &fragment_source).map(Shader) }
	}

	pub fn bind(&self) {
		unsafe {
			gl::UseProgram(self.0);
		}
	}

	pub fn uniform4f(&self, name: &'static str, [a, b, c, d]: [f32; 4]) {
		debug_assert!(
			name.as_bytes()[name.len() - 1] == b'\0',
			"Uniform name doesn't have null byte terminator"
		);

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

unsafe fn new_shader(vertex_source: &CStr, fragment_source: &CStr) -> Result<u32, ShaderError> {
	let compile_shader = |source: &CStr, shader_type| {
		let shader = gl::CreateShader(shader_type);
		gl::ShaderSource(
			shader,
			1,
			&(source.as_ptr() as *const GLchar) as *const *const GLchar,
			&(source.to_bytes_with_nul().len() as i32),
		);
		gl::CompileShader(shader);

		let mut success = false;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success as *mut _ as *mut _);
		if !success {
			let info_log = get_shader_info_log(shader);
			match shader_type {
				gl::VERTEX_SHADER => {
					return Err(ShaderError::FailedToCompileVertexShader(info_log))
				}
				gl::FRAGMENT_SHADER => {
					return Err(ShaderError::FailedToCompileFragmentShader(info_log))
				}
				_ => unimplemented!(),
			}
		}
		Ok(shader)
	};

	let vertex_shader = compile_shader(vertex_source, gl::VERTEX_SHADER)?;
	let fragment_shader = compile_shader(fragment_source, gl::FRAGMENT_SHADER)?;

	let shader = gl::CreateProgram();
	gl::AttachShader(shader, vertex_shader);
	gl::AttachShader(shader, fragment_shader);
	gl::LinkProgram(shader);

	let mut success = false;
	gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success as *mut _ as *mut _);
	if !success {
		return Err(ShaderError::FailedToLinkProgram(get_shader_info_log(
			shader,
		)));
	}

	gl::DeleteShader(vertex_shader);
	gl::DeleteShader(fragment_shader);

	Ok(shader)
}

unsafe fn get_shader_info_log(shader: u32) -> String {
	let mut info_log_length: i32 = 0;
	gl::GetShaderiv(
		shader,
		gl::INFO_LOG_LENGTH,
		&mut info_log_length as *mut i32,
	);

	let mut info_log: Vec<u8> = Vec::with_capacity(info_log_length as usize);
	let mut info_log_written: i32 = 0;
	gl::GetShaderInfoLog(
		shader,
		info_log_length,
		&mut info_log_written,
		info_log.as_mut_ptr() as *mut GLchar,
	);
	info_log.set_len(info_log_written as usize);
	String::from_utf8_unchecked(info_log)
}
