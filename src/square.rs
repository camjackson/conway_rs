extern crate glium;

use glium::Display;
use glium::vertex::VertexBufferAny;
use glium::vertex::VertexBuffer;
use glium::index::IndexBuffer;
use glium::index::TrianglesList;

use grid::Grid;

pub fn geometry(display: &Display) -> (VertexBufferAny, IndexBuffer) {
    (vertices(display), indices(display))
}

fn vertices(display: &Display) -> VertexBufferAny {
    #[derive(Copy, Clone)]
    struct Vertex {
        vertex_position: [f32; 2],
        vertex_color: [f32; 3],
    }

    implement_vertex!(Vertex, vertex_position, vertex_color);

    let colour = [0.2, 0.2, 0.2];

    VertexBuffer::new(display,
        vec![
            Vertex { vertex_position: [ -0.5,  0.5], vertex_color: colour },
            Vertex { vertex_position: [  0.5,  0.5], vertex_color: colour },
            Vertex { vertex_position: [  0.5, -0.5], vertex_color: colour },
            Vertex { vertex_position: [ -0.5, -0.5], vertex_color: colour },
        ]
    ).into_vertex_buffer_any()
}

fn indices(display: &Display) -> IndexBuffer {
    IndexBuffer::new(display, TrianglesList(vec![0u16, 1, 2, 0, 2, 3]))
}

#[derive(Copy, Clone)]
struct ModelTransform {
    model_position: [f32; 2],
    model_scale: f32,
}

pub fn instances(display: &Display, scale: f32, grid: &Grid) -> VertexBufferAny {
    implement_vertex!(ModelTransform, model_position, model_scale);

    let mut data = Vec::new();
    for y in 0..(grid.height() as i16) {
        for x in 0..(grid.width() as i16) {
            if grid.get((x, y)) {
                data.push(ModelTransform{
                    model_position: [x as f32 * scale + scale / 2.0, -(y as f32 * scale + scale / 2.0)],
                    model_scale: scale,
                });
            }
        }
    }
    VertexBuffer::new(display, data).into_vertex_buffer_any()
}
