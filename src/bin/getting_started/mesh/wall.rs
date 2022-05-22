#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

pub const WALL: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [-1.0, -1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    },
];
