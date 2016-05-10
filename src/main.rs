#[macro_use]
extern crate glium;
extern crate glutin;
extern crate rand;

use glium::DisplayBuild;
use glium::Surface;
use glium::Program;
use std::env;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use grid::Grid;

mod shaders;
mod square;
mod grid;
mod cell;
mod seeds;

const UPDATES_PER_SECOND: u64 = 30;

#[allow(unused_variables)]
fn main() {
    let width = 1024.0;
    let height = 768.0;

    let seed = env::args().nth(1).map(|s|
        seeds::named(&s).expect("Invalid seed name! Valid seeds are random or gosper_glider")
    ).unwrap_or(seeds::random);

    let display = glutin::WindowBuilder::new()
        .with_dimensions(width as u32, height as u32)
        .with_title(format!("Hello, world!"))
        .with_vsync()
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

    // Arc is needed until thread::scoped is stable
    let grid = Arc::new(Mutex::new(Grid::new(seed, 128, 96, square_size)));

    {
        let grid = grid.clone();
        // Spawn off thread to update the grid. Main thread will be in charge of rendering
        thread::spawn(move || {
            loop {
                let sleep_duration = Duration::from_millis(1000 / UPDATES_PER_SECOND);
                std::thread::sleep(sleep_duration);
                grid.lock().unwrap().update();
            }
        });
    }

    loop {
        let instances = {
            let grid = grid.lock().unwrap();
            square::instances(&display, &grid.cells)
        };

        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);

        frame.draw((&vertices,
                    instances.per_instance().unwrap()),
                   &indices,
                   &program,
                   &uniforms,
                   &std::default::Default::default()).unwrap();

        frame.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _,
                                             Some(glutin::VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}
