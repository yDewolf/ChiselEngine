use crate::engine::graphics::transform::{Vector3};
use crate::engine::geometry::vertex::Vertex;

use crate::utils::file_utils;

// Mesh only, not instance
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>
}

impl Mesh {
    pub fn from_obj_file(file_path: &str) -> Self {
        let (vertices, indices) = file_utils::read_obj_file(file_path);

        return Mesh {
            vertices,
            indices
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices 
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}
