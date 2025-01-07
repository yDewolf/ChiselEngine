#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub entity_id: u32,
    pub position: [f32; 2],
}

impl Vertex {
    pub fn from_pos(x: f32, y: f32) -> Self {
        Self {
            entity_id: 0,
            position: [x, y]
        }
    }
}