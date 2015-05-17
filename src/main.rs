#[macro_use]
extern crate glium;
extern crate glutin;
extern crate clock_ticks;
extern crate rand;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;
use clock_ticks::precise_time_ns;

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
        .build_glium()
        .unwrap();

    let (vertices, indices) = square::geometry(&display);
    let program = Program::from_source(&display, shaders::VERTEX, shaders::FRAGMENT, None).unwrap();

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

    let mut accumulator = 0;
    let mut previous_clock = precise_time_ns();

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

        let now = precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;
        const FIXED_TIME_STAMP: u64 = 16666667; //every 16.67ms, or 60fps

        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;
            grid.update();
        }
    }
}
