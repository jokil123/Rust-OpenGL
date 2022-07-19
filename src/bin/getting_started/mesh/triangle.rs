#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub const TRIANGLE: [Vertex; 3] = [
    Vertex {
        position: [-0.5, -0.5],
    },
    Vertex {
        position: [0.0, 0.5],
    },
    Vertex {
        position: [0.5, -0.25],
    },
];
