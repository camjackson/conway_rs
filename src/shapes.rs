extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;

static SQUARE_SIZE: f32 = 20.0;

pub fn squares(display: &Display) -> (VertexBufferAny, VertexBuffer<Location>, IndexBuffer) {
	(square(display), instances(display), ib(display))
}

fn square(display: &Display) -> VertexBufferAny {
	#[derive(Copy, Clone)]
	struct Vertex {
		position: [f32; 2],
		color: [f32; 3],
	}

	implement_vertex!(Vertex, position, color);

	let size = SQUARE_SIZE;

	glium::VertexBuffer::new(display,
		vec![
			Vertex { position: [  0.0,  0.0], color: [1.0, 0.1, 0.1] },
			Vertex { position: [  0.0, size], color: [0.1, 1.0, 0.1] },
			Vertex { position: [ size, size], color: [0.1, 0.1, 1.0] },
			Vertex { position: [ size,  0.0], color: [0.1, 1.0, 1.0] },
		]
	).into_vertex_buffer_any()
}

#[derive(Copy, Clone)]
pub struct Location {
	world_position: [f32; 2],
}

fn instances(display: &Display) -> VertexBuffer<Location> {
	implement_vertex!(Location, world_position);

	let grid_width = 102;
	let grid_height = 76;

	let mut data = Vec::new();
	for x in (0i32 .. grid_width) {
		for y in (0i32 .. grid_height) {
			data.push(Location {
				world_position: [
					((x - grid_width / 2) as f32) * SQUARE_SIZE,
					((y - grid_height / 2) as f32) * SQUARE_SIZE
				]
			});
		}
	}

	glium::vertex::VertexBuffer::new(display, data)
}

fn ib(display: &Display) -> IndexBuffer {
	glium::IndexBuffer::new(display, glium::index::TrianglesList(vec![0u16, 1, 2, 0, 2, 3]))
}
