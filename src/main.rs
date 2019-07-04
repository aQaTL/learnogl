#![feature(const_fn)]

use std::path::Path;
use std::time::Instant;

use crate::game::GameFaze;
use core::borrow::{Borrow, BorrowMut};
use glium::backend::Facade;
use glium::{
    glutin,
    glutin::*,
    implement_vertex,
    index::PrimitiveType,
    program,
    texture::{RawImage2d, SrgbTexture2d},
    uniforms,
    vertex::VertexBufferAny,
    Surface,
};
use imgui::im_str;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use nalgebra_glm as glm;

mod game;
mod renderer;

fn main() {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions((800, 600).into())
        .with_title(env!("CARGO_PKG_NAME"));

    let context = glium::glutin::ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_vsync(false);

    let display = glium::Display::new(window, context, &events_loop).unwrap();
    //    {
    let window = display.gl_window();
    let window = window.window();
    window.hide_cursor(true);
    if let Err(err) = window.grab_cursor(true) {
        println!("Failed to grab the cursor: {}", err);
    }
    //    }

    println!("{:?}", display.get_opengl_version());
    println!("{:?}", display.get_opengl_vendor_string());
    println!("{:?}", display.get_opengl_renderer_string());
    println!();

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

        glium::VertexBuffer::new(&display, &verticies)
            .unwrap()
            .into_vertex_buffer_any()
    };

    let index_buffer = glium::IndexBuffer::new(
        &display,
        PrimitiveType::TrianglesList,
        &[0u16, 1, 2, 0, 3, 2, 0],
    )
    .unwrap()
    .into();

    let program = program!(
    &display,
    420 => {
        vertex: include_str!("shaders/vertex.glsl"),
        fragment: include_str!("shaders/fragment.glsl"),
    })
    .unwrap();

    let mut camera = Camera::default();
    toggle_mouse_grab(&mut camera, window);

    let mut keys = Keys([KeyState::Released; 161]);

    let start_time = Instant::now();
    let mut program_time: f32;

    let mut last_frame = Instant::now();
    let mut delta_time: f32;

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut platform = WinitPlatform::init(&mut imgui);
    {
        let gl_window = display.gl_window();
        let window = gl_window.window();
        platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Rounded);
    }

    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                size_pixels: 13.0,
                ..imgui::FontConfig::default()
            }),
        }]);

    let mut gui_renderer = imgui_glium_renderer::GliumRenderer::init(&mut imgui, &display).unwrap();

    let mut game = game::Game::new(load_texture("images/box.png", &display).unwrap());

    game.set_faze(GameFaze::GameRunning);

    let mut running = true;
    while running {
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            let io = imgui.io_mut();
            platform
                .prepare_frame(io, &window)
                .expect("Failed to start frame");
            io.update_delta_time(last_frame);
        }

        delta_time = last_frame.elapsed().as_micros() as f32 / 1_000_000.0;
        last_frame = Instant::now();

        program_time = start_time.elapsed().as_micros() as f32 / 1_000_000.0;

        //Change JustPressed keys to Pressed
        keys.0
            .iter_mut()
            .filter(|key| **key == KeyState::JustPressed)
            .for_each(|mut key| *key = KeyState::Pressed);

        events_loop.poll_events(|event| {
            platform.handle_event(imgui.io_mut(), &window, &event);
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
                        keys.0[key_code as usize] = match key.state {
                            ElementState::Released => KeyState::Released,
                            ElementState::Pressed => KeyState::JustPressed,
                        };
                    }
                    None => (),
                },
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                    ..
                } => {
                    //camera_process_mouse_input(&mut camera, dx as f32, dy as f32);
                }
                _ => (),
            }
        });

        //        camera_process_movement_input(&mut camera, &keys, delta_time);
        camera_process_shortcuts(&display, &mut camera, &keys, delta_time);

        // Game update
        game.process_keyboard_input(&keys, delta_time);
        game.update(delta_time);

        let mut frame = display.draw();

        frame.clear_color_and_depth((0.2, 0.3, 0.3, 1.0), 1.0);

        match game.faze {
            GameFaze::TitleScreen => (),
            GameFaze::GameRunning => {
                renderer::render(
                    &mut frame,
                    &program,
                    &game,
                    &camera,
                    &vertex_buffer,
                    &index_buffer,
                )
                .unwrap();

                let mut ui = imgui.frame();
                ui.window(im_str!("learnogl"))
                    .size([300.0, 120.0], imgui::Condition::FirstUseEver)
                    .build(|| {
                        ui.text(im_str!("Player pos: {:?}", game.tower.player.pos));
                        ui.text(im_str!("Camera pos: {:?}", camera.pos));
                    });
                gui_renderer
                    .render(&mut frame, ui.render())
                    .expect("imgui renderer fail");
            }
            GameFaze::DeathScreen => (),
        }

        frame.finish().unwrap();
    }
}

fn load_texture<P: AsRef<Path>>(
    path: P,
    display: &glium::Display,
) -> Result<SampledSrgbTexture2d, glium::texture::TextureCreationError> {
    let image = image::open(path).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = SrgbTexture2d::new(display, image)?;
    Ok(SampledSrgbTexture2d {
        tex: texture,
        sampler_behavior: uniforms::SamplerBehavior {
            minify_filter: uniforms::MinifySamplerFilter::Nearest,
            magnify_filter: uniforms::MagnifySamplerFilter::Nearest,
            ..Default::default()
        },
    })
}

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 3],
    normal: [f32; 3],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, normal, tex_coords);

#[allow(dead_code)]
const fn radians(degrees: f32) -> f32 {
    degrees * (std::f32::consts::PI / 180.0)
}

pub struct Keys([KeyState; 161]);

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    Released,
    Pressed,
    JustPressed,
}

impl Keys {
    fn is_just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.0[key as usize] == KeyState::JustPressed
    }
}

impl std::ops::Index<VirtualKeyCode> for Keys {
    type Output = bool;

    fn index(&self, key: VirtualKeyCode) -> &Self::Output {
        match self.0[key as usize] {
            KeyState::Released => &false,
            KeyState::JustPressed | KeyState::Pressed => &true,
        }
    }
}

pub struct Camera {
    pos: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,

    movement_speed: f32,
    sensitivity: f32,

    yaw: f32,
    pitch: f32,

    fov: f32,

    mouse_grabbed: bool,
}

impl Camera {
    fn view(&self) -> glm::Mat4 {
        glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: glm::Vec3::new(0.0, 0.0, 0.1),
            front: glm::Vec3::new(0.0, 0.0, -1.0),
            up: glm::Vec3::new(0.0, 1.0, 0.0),

            movement_speed: 2.0,
            sensitivity: 0.1,

            yaw: -90.0,
            pitch: 0.0,

            fov: 45.0,

            mouse_grabbed: false,
        }
    }
}

fn toggle_mouse_grab(camera: &mut Camera, window: &glutin::Window) {
    camera.mouse_grabbed = !camera.mouse_grabbed;
    window.hide_cursor(camera.mouse_grabbed);
    if let Err(err) = window.grab_cursor(camera.mouse_grabbed) {
        println!("Failed to ungrab the cursor: {}", err);
    }
}

fn camera_process_mouse_input(camera: &mut Camera, mut dx: f32, mut dy: f32) {
    dx *= camera.sensitivity;
    dy *= -camera.sensitivity;

    camera.yaw += dx as f32;
    camera.pitch += dy as f32;

    match camera.pitch {
        n if n > 89.0 => camera.pitch = 89.0,
        n if n < -89.0 => camera.pitch = -89.0,
        _ => (),
    }

    let front = glm::Vec3::new(
        radians(camera.yaw).cos() * radians(camera.pitch).cos(),
        radians(camera.pitch).sin(),
        radians(camera.yaw).sin() * radians(camera.pitch).cos(),
    );
    camera.front = glm::normalize(&front);
}

fn camera_process_movement_input(camera: &mut Camera, keys: &Keys, delta_time: f32) {
    use glium::glutin::VirtualKeyCode::*;
    if keys[W] {
        camera.pos = camera.pos + (camera.front * (camera.movement_speed * delta_time));
    }
    if keys[S] {
        camera.pos = camera.pos - (camera.front * (camera.movement_speed * delta_time));
    }
    if keys[A] {
        camera.pos = camera.pos
            - (glm::normalize(&glm::cross::<f32, glm::U3>(&camera.front, &camera.up))
                * (camera.movement_speed * delta_time));
    }
    if keys[D] {
        camera.pos = camera.pos
            + (glm::normalize(&glm::cross::<f32, glm::U3>(&camera.front, &camera.up))
                * (camera.movement_speed * delta_time));
    }
    if keys[Space] {
        camera.pos = camera.pos + (camera.up * (camera.movement_speed * delta_time));
    }
    if keys[LShift] {
        camera.pos = camera.pos - (camera.up * (camera.movement_speed * delta_time));
    }
}

fn camera_process_shortcuts(
    display: &glium::Display,
    camera: &mut Camera,
    keys: &Keys,
    delta_time: f32,
) {
    use glium::glutin::VirtualKeyCode::*;
    if keys.is_just_pressed(Tab) {
        let window = display.gl_window();
        let window = window.window();
        toggle_mouse_grab(camera, window);
    }
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

pub struct GenericCube<'a> {
    pub pos: glm::Vec3,
    pub texture: &'a SampledSrgbTexture2d,
}

fn load_mesh(display: &glium::Display, path: &Path) -> Result<VertexBufferAny, tobj::LoadError> {
    #[derive(Copy, Clone)]
    struct Vertex {
        pos: [f32; 3],
        normals: [f32; 3],
        tex_coords: [f32; 2],
    }
    implement_vertex!(Vertex, pos, normals, tex_coords);

    let mut min_pos = [std::f32::INFINITY; 3];
    let mut pax_pos = [std::f32::NEG_INFINITY; 3];
    let mut vertex_data = Vec::new();

    match tobj::load_obj(path) {
        Ok((models, mats)) => {
            for model in &models {
                println!("Loading model: {}", model.name);

                let mesh = &model.mesh;
                vertex_data.reserve(mesh.indices.len());
                for idx in &mesh.indices {
                    let i = *idx as usize;
                    let pos = [
                        mesh.positions[i * 3],
                        mesh.positions[i * 3 + 1],
                        mesh.positions[i * 3 + 2],
                    ];

                    let normals = if !mesh.normals.is_empty() {
                        [
                            mesh.normals[i * 3],
                            mesh.normals[i * 3 + 1],
                            mesh.normals[i * 3 + 2],
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    };

                    let tex_coords = if !mesh.texcoords.is_empty() {
                        [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
                    } else {
                        [0.0, 0.0]
                    };

                    vertex_data.push(Vertex {
                        pos,
                        normals,
                        tex_coords,
                    });
                }
            }

            let vb = glium::VertexBuffer::new(display, &vertex_data).unwrap();
            Ok(vb.into_vertex_buffer_any())
        }
        Err(e) => return Err(e),
    }
}
