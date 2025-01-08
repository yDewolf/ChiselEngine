use crate::engine::graphics::transform::Vector3;


#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub entity_id: u32,
    pub position: Vector3,
}

impl Vertex {
    pub fn from_pos(x: f32, y: f32, z: f32) -> Self {
        Self {
            entity_id: 0,
            position: Vector3::new(x, y, z)
        }
    }

    pub fn from_vec3(vec: Vector3) -> Self {
        Self {
            entity_id: 0,
            position: vec
        }
    }
}