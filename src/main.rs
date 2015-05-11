#[macro_use]
extern crate glium;
extern crate glutin;

use glium::Surface;

mod shaders;
mod square;
mod grid;

fn main() {
	use glium::DisplayBuild;

	let width = 1024.0;
	let height = 768.0;
	let display = glutin::WindowBuilder::new()
		.with_dimensions(width as u32, height as u32)
		.with_title(format!("Hello, world!"))
		.build_glium()
		.unwrap();

	let square_size = 32.0;

	let (vertices, indices) = square::geometry(&display, square_size);
	let program = glium::Program::from_source(&display, &shaders::load("vertex"), &shaders::load("fragment"), None).unwrap();

	let uniforms = uniform! {
		transform: [
			[1.0 / width, 0.0         , 0.0, 0.0],
			[0.0        , 1.0 / height, 0.0, 0.0],
			[0.0        , 0.0         , 1.0, 0.0],
			[0.0        , 0.0         , 0.0, 1.0f32]
		]
	};

	let grid = grid::new(64, 48, square_size);
	let instances = square::instances(&display, grid);

	loop {
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
	}
}
