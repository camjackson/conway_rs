#[macro_use]
extern crate glium;
extern crate glutin;
extern crate clock_ticks;

use glium::Surface;

use std::thread;

mod shaders;
mod shapes;

fn main() {
	use glium::DisplayBuild;

	let width = 1024.0;
	let height = 768.0;
	let display = glutin::WindowBuilder::new()
		.with_dimensions(width as u32, height as u32)
		.with_title(format!("Hello, world!"))
		.build_glium()
		.unwrap();

	let (vertex_buffer, index_buffer) = shapes::square(&display);
	let program = glium::Program::from_source(&display, &shaders::load("vertex"), &shaders::load("fragment"), None).unwrap();

	let mut accumulator = 0;
	let mut previous_clock = clock_ticks::precise_time_ns();

	loop {
		let uniforms = uniform! {
			transform: [
				[1.0 / width, 0.0         , 0.0, 0.0],
				[0.0        , 1.0 / height, 0.0, 0.0],
				[0.0        , 0.0         , 1.0, 0.0],
				[0.0        , 0.0         , 0.0, 1.0f32]
			]
		};

		// drawing a frame
		let mut frame = display.draw();
		frame.clear_color(1.0, 1.0, 1.0, 1.0);
		frame.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &std::default::Default::default()).unwrap();
		frame.finish();

		// polling and handling the events received by the window
		for event in display.poll_events() {
			match event {
					glutin::Event::Closed => return,
					_ => ()
				}
		}

		let now = clock_ticks::precise_time_ns();
		accumulator += now - previous_clock;
		previous_clock = now;

		const FIXED_TIME_STAMP: u64 = 16666667;
		while accumulator >= FIXED_TIME_STAMP {
			accumulator -= FIXED_TIME_STAMP;

			// if you have a game, update the state here
		}

		thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
	}
}
