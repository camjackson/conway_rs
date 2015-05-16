#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;

mod shaders;
mod square;
mod grid;
mod cell;
mod seeds;

fn main() {
    let width = 1024.0;
    let height = 768.0;
    let display = glutin::WindowBuilder::new()
        .with_dimensions(width as u32, height as u32)
        .with_title(format!("Hello, world!"))
        .with_vsync()
        .build_glium()
        .unwrap();

    let (vertices, indices) = square::geometry(&display);
    let program = Program::from_source(&display, &shaders::load("vertex"), &shaders::load("fragment"), None).unwrap();

    let uniforms = uniform! {
        view_transform: [
            [ 1.0 / width, 0.0         , 0.0, 0.0],
            [ 0.0        , 1.0 / height, 0.0, 0.0],
            [ 0.0        , 0.0         , 1.0, 0.0],
            [-1.0        , 1.0         , 0.0, 1.0f32]
        ]
    };

    let square_size = 16.0;
    let mut grid = grid::new(128, 96, square_size);

    loop {
        let instances = square::instances(&display, &grid.cells);

        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.draw((&vertices, instances.per_instance_if_supported().unwrap()), &indices, &program, &uniforms, &std::default::Default::default()).unwrap();
        frame.finish();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                _ => ()
            }
        }

        grid.update();
    }
}
