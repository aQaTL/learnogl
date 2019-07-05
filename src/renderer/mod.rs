use glium::index::IndexBufferAny;
use glium::uniform;
use glium::vertex::VertexBufferAny;
use glium::{
	implement_vertex, index::PrimitiveType, Depth, DepthTest, Display, DrawParameters, Surface,
};

use crate::game::Game;
use crate::input::Camera;

pub fn create_rect_vb(
	display: &Display,
) -> Result<(VertexBufferAny, IndexBufferAny), Box<dyn std::error::Error>> {
	let vertex_buffer = {
		#[derive(Copy, Clone)]
		struct Vertex {
			pos: [f32; 3],
			tex_coords: [f32; 2],
		}
		implement_vertex!(Vertex, pos, tex_coords);

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

		glium::VertexBuffer::new(display, &verticies)?.into_vertex_buffer_any()
	};

	let index_buffer = glium::IndexBuffer::new(
		display,
		PrimitiveType::TrianglesList,
		&[0u16, 1, 2, 0, 3, 2, 0],
	)?
	.into();

	Ok((vertex_buffer, index_buffer))
}

pub fn render(
	frame: &mut glium::Frame,
	program: &glium::Program,
	game: &Game,
	camera: &Camera,
	vertex_buffer: &VertexBufferAny,
	index_buffer: &IndexBufferAny,
) -> Result<(), glium::DrawError> {
	// scaling -> rotation -> translation (function order is reversed!)
	let view = camera.view();

	//let (width, height) = frame.get_dimensions();
	let (width, height) = (800.0, 600.0);

	//let projection = glm::perspective(radians(camera.fov), (width / height) as f32, 0.1, 100.0);

	let projection = glm::ortho(0.0, width as f32, 0.0, height as f32, 0.1, 100.0);

	let vp = projection * view;

	let draw_params = DrawParameters {
		depth: Depth {
			test: DepthTest::IfLess,
			write: true,
			..Default::default()
		},
		..Default::default()
	};

	{
		//Drawing player
		let player = &game.tower.player;
		let mut model = glm::Mat4::identity();
		model = glm::translate(&model, &player.pos);
		model = glm::translate(
			&model,
			&glm::Vec3::new(player.size.x / 2.0, player.size.y / 2.0, 0.0),
		);
		model = glm::scale(&model, &(player.size / 2.0));

		let transform = vp * model;

		let uniforms = uniform! {
			transform: *transform.as_ref(),
			tex: &game.tower.player.tex,
		};

		frame
			.draw(
				vertex_buffer,
				index_buffer,
				program,
				&uniforms,
				&draw_params,
			)
			.unwrap();
	}

	Ok(())
}
