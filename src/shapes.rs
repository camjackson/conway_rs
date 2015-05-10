extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::index::IndexBuffer;


pub fn square(display: &Display) -> (VertexBufferAny, IndexBuffer) {
	(vb(display), ib(display))
}

#[derive(Copy, Clone)]
struct Vertex {
	position: [f32; 2],
	color: [f32; 3],
}

implement_vertex!(Vertex, position, color);

fn vb(display: &Display) -> VertexBufferAny {
	let col = [0.1, 0.1, 0.1];
	let size = 20.0;

	glium::VertexBuffer::new(display,
		vec![
			Vertex { position: [  0.0,  0.0], color: col },
			Vertex { position: [  0.0, size], color: col },
			Vertex { position: [ size, size], color: col },
			Vertex { position: [ size,  0.0], color: col },
		]
	).into_vertex_buffer_any()
}

fn ib(display: &Display) -> IndexBuffer {
	glium::IndexBuffer::new(display, glium::index::TrianglesList(vec![0u16, 1, 2, 0, 2, 3]))
}
