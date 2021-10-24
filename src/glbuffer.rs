use std::ffi::c_void;
use std::{mem, ptr};

#[repr(transparent)]
pub struct Vao(u32);

impl Vao {
	pub fn new_static<T: VertexLayout<N>, const N: usize>(data: &[T]) -> Self {
		let mut vao = Vao(0);
		unsafe { gl::GenVertexArrays(1, vao.as_mut_ptr()) };
		vao.bind();

		let buffer = Buffer::new(data, BufferType::ArrayBuffer, BufferUsage::StaticDraw);

		for (
			idx,
			VertexAttrib {
				count,
				type_,
				type_size,
			},
		) in T::layout().into_iter().enumerate()
		{
			unsafe {
				gl::VertexAttribPointer(
					idx as u32,
					count as i32,
					type_ as u32,
					gl::FALSE,
					count as i32 * type_size,
					ptr::null(),
				);
				gl::EnableVertexAttribArray(idx as u32);
			}
		}

		unsafe { Vao::unbind() };
		drop(buffer);

		vao
	}

	pub fn as_mut_ptr(&mut self) -> *mut u32 {
		self as *mut Self as *mut u32
	}

	pub fn bind(&self) {
		unsafe { gl::BindVertexArray(self.0) };
	}

	pub unsafe fn unbind() {
		gl::BindVertexArray(0);
	}
}

impl Drop for Vao {
	fn drop(&mut self) {
		unsafe { gl::DeleteVertexArrays(1, self.as_mut_ptr()) };
	}
}

#[repr(transparent)]
pub struct Buffer(u32);

#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferType {
	ArrayBuffer = gl::ARRAY_BUFFER,
	AtomicCounterBuffer = gl::ATOMIC_COUNTER_BUFFER,
	CopyReadBuffer = gl::COPY_READ_BUFFER,
	CopyWriteBuffer = gl::COPY_WRITE_BUFFER,
	DispatchIndirectBuffer = gl::DISPATCH_INDIRECT_BUFFER,
	DrawIndirectBuffer = gl::DRAW_INDIRECT_BUFFER,
	ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,
	PixelPackBuffer = gl::PIXEL_PACK_BUFFER,
	PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,
	QueryBuffer = gl::QUERY_BUFFER,
	ShaderStorageBuffer = gl::SHADER_STORAGE_BUFFER,
	TextureBuffer = gl::TEXTURE_BUFFER,
	TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,
	UniformBuffer = gl::UNIFORM_BUFFER,
}

#[allow(dead_code)]
#[repr(u32)]
#[derive(Copy, Clone)]
pub enum BufferUsage {
	StreamDraw = gl::STREAM_DRAW,
	StreamRead = gl::STREAM_READ,
	StreamCopy = gl::STREAM_COPY,
	StaticDraw = gl::STATIC_DRAW,
	StaticRead = gl::STATIC_READ,
	StaticCopy = gl::STATIC_COPY,
	DynamicDraw = gl::DYNAMIC_DRAW,
	DynamicRead = gl::DYNAMIC_READ,
	DynamicCopy = gl::DYNAMIC_COPY,
}

impl Buffer {
	pub fn new<T>(data: &[T], buffer_type: BufferType, usage: BufferUsage) -> Self {
		let mut buffer = Buffer(0);
		unsafe { gl::GenBuffers(1, buffer.as_mut_ptr()) };
		unsafe { gl::BindBuffer(buffer_type as u32, buffer.0) };

		unsafe {
			gl::BufferData(
				buffer_type as u32,
				mem::size_of_val(data) as isize,
				data.as_ptr() as *const c_void,
				usage as u32,
			);
		}

		buffer
	}

	pub fn as_mut_ptr(&mut self) -> *mut u32 {
		self as *mut Self as *mut u32
	}
}

impl Drop for Buffer {
	fn drop(&mut self) {
		unsafe { gl::DeleteBuffers(1, self.as_mut_ptr()) }
	}
}

pub trait VertexLayout<const N: usize> {
	fn layout() -> [VertexAttrib; N];
}

pub struct VertexAttrib {
	count: u32,
	type_: AttribType,
	type_size: i32,
}

impl VertexAttrib {
	pub fn new<T: IntoAttribType>(count: u32) -> Self {
		VertexAttrib {
			count,
			type_: T::into_attrib_type(),
			type_size: mem::size_of::<T>() as i32,
		}
	}
}

#[repr(u32)]
pub enum AttribType {
	Float = gl::FLOAT,
}

pub trait IntoAttribType {
	fn into_attrib_type() -> AttribType;
}

impl IntoAttribType for f32 {
	fn into_attrib_type() -> AttribType {
		AttribType::Float
	}
}
