#![feature(step_by)]
extern crate glutin;

#[macro_use]
extern crate glium;
extern crate rand;

use glium::Surface;

mod support;
mod programs;
mod rendering;
use rendering::PosOnlyVertex;


extern crate cgmath;
use cgmath::FixedArray;
use std::f32;
use rand::Rng;

#[derive(Copy, Clone, Debug)]
struct Attr {
    id: f32,
    offset: [f32; 3],
}

implement_vertex!(Attr, id, offset);


fn gen_offsets(N: usize) -> Vec<Attr> {
    let mut rng = rand::thread_rng();
    let mut offset_data = Vec::new();

    for idx in (0 .. N) {
        let (dx, dy, dc) = rng.gen::<(f32, f32, f32)>();

        let attr = Attr {
            id: idx as f32,
            offset: [dx, dy, dc],
        };
        offset_data.push(attr);
    }
    offset_data
}

fn main() {
    use glium::DisplayBuild;

    // building the display, ie. the main object
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .with_dimensions(800, 600)
        .with_title(format!("Voronoi"))
        .build_glium()
        .unwrap();

    let mut cone_data = vec![PosOnlyVertex { position: [ 0.0, 0.0, 0.0] }];
    let triangles_per = 60;
    for i in (0..360).step_by(360 / triangles_per) {
        let x = 10.0 * (i as f32 * std::f32::consts::PI / 180.0).cos();
        let y = 10.0 * (i as f32 * std::f32::consts::PI / 180.0).sin();
        cone_data.push(PosOnlyVertex { position: [ x, y, 1.0] });
    }

    let mut index_data: Vec<u16> = (0..cone_data.len() as u16).collect();
    index_data.push(1);
    let indices = glium::IndexBuffer::new(&display, glium::index::TriangleFan(index_data));
    let cone = glium::VertexBuffer::new(&display, cone_data);

    let mut n_cones = 20;
    let mut offsets = glium::vertex::VertexBuffer::new(&display, gen_offsets(n_cones));

    let pm = programs::ProgramManager::new();
    let mut program = pm.create(&display, &programs::ShaderBundle::new("simple.vs", "simple.fs", None, None, None)).unwrap();
    let mut points = pm.create(&display, &programs::ShaderBundle::new("points.vs", "points.fs", None, None, None)).unwrap();

    let points_indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    // draw parameters
    let mut params = glium::DrawParameters {
        depth_test: glium::DepthTest::IfLess,
        depth_write: true,
        backface_culling: glium::BackfaceCullingMode::CullingDisabled,
        point_size: Some(5.0),
        .. std::default::Default::default()
    };

    // the main loop
    support::start_loop(|| {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw((&cone, offsets.per_instance_if_supported().unwrap()),
                    &indices, &program, &uniform!{}, &params).unwrap();
        target.draw(&offsets, &points_indices, &points, &uniform!{}, &params).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,

                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Up)) => {
                    n_cones += 1;
                    offsets = glium::vertex::VertexBuffer::new(&display, gen_offsets(n_cones));
                },

                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Down)) => {
                    if n_cones >= 2 {
                        n_cones -= 1;
                        offsets = glium::vertex::VertexBuffer::new(&display, gen_offsets(n_cones));
                    }
                },

                _ => (),
            }
        }
        support::Action::Continue
    });
}
