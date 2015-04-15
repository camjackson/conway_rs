#[macro_use]
extern crate glium;
extern crate glutin;
extern crate clock_ticks;

use glium::Surface;

use std::thread;

pub enum Action {
	Stop,
	Continue,
}

fn main() {
	use glium::DisplayBuild;

	let display = glutin::WindowBuilder::new()
		.with_dimensions(1024, 768)
		.with_title(format!("Hello, world!"))
		.build_glium()
		.unwrap();

	let vertex_buffer = {
		#[derive(Copy)]
		struct Vertex {
			position: [f32; 2],
			color: [f32; 3],
		}

		impl Clone for Vertex {
			fn clone(&self) -> Vertex {
				return *self
			}
		}

		implement_vertex!(Vertex, position, color);

		glium::VertexBuffer::new(&display,
			vec![
				Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
				Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
				Vertex { position: [ 0.5, -0.5], color: [1.0, 0.0, 0.0] },
			]
		)
	};

	let index_buffer = glium::IndexBuffer::new(&display,
		glium::index::TrianglesList(vec![0u16, 1, 2]));

	let program = glium::Program::from_source(&display,
		// vertex shader
		"	#version 330

			uniform mat4 matrix;

			in vec2 position;
			in vec3 color;

			out vec3 vColor;

			void main() {
				gl_Position = vec4(position, 0.0, 1.0) * matrix;
				vColor = color;
			}
		",

		// fragment shader
		"	#version 330

			in vec3 vColor;
			out vec4 color;

			void main() {
				color = vec4(vColor, 1.0);
			}
		",

		// geometry shader
		None
	).unwrap();

	let mut accumulator = 0;
	let mut previous_clock = clock_ticks::precise_time_ns();

	loop {
		let uniforms = uniform! {
			matrix: [
				[1.0, 0.0, 0.0, 0.0],
				[0.0, 1.0, 0.0, 0.0],
				[0.0, 0.0, 1.0, 0.0],
				[0.0, 0.0, 0.0, 1.0f32]
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
