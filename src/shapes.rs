extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::index::IndexBuffer;


pub fn square(display: &Display) -> (VertexBufferAny, IndexBuffer) {
	(vb(display), ib(display))
}

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

fn vb(display: &Display) -> VertexBufferAny {
	let black = [0.1, 0.1, 0.1];

	glium::VertexBuffer::new(display,
		vec![
			Vertex { position: [ 0.0, 0.0], color: black },
			Vertex { position: [ 0.0, 1.0], color: black },
			Vertex { position: [ 1.0, 1.0], color: black },
			Vertex { position: [ 1.0, 0.0], color: black },
		]
	).into_vertex_buffer_any()
}

fn ib(display: &Display) -> IndexBuffer {
	glium::IndexBuffer::new(display, glium::index::TrianglesList(vec![0u16, 1, 2, 0, 2, 3]))
}
