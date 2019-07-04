use crate::{game::Game, radians, Camera};
use glium::index::IndexBufferAny;
use glium::uniform;
use glium::vertex::VertexBufferAny;
use glium::{
    index::{self, PrimitiveType},
    Depth, DepthTest, DrawParameters, IndexBuffer, Surface, VertexBuffer,
};
use nalgebra_glm as glm;

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

    //    let (width, height) = frame.get_dimensions();
    let (width, height) = (800.0, 600.0);

    //    let projection = glm::perspective(radians(camera.fov), (width / height) as f32, 0.1, 100.0);

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
