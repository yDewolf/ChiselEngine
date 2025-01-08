use crate::engine::graphics::transform::{Vector2, Vector3};
use crate::engine::graphics::opengl::gl_objects::{Ibo, Vao, Vbo};

use crate::utils::file_utils;

#[derive(Clone)]
#[repr(C)]
pub struct Vertex {
    pub entity_id: u32,
    
    pub position: Vector3,
    pub normal: Vector3,
    pub tex_coords: Vector2,
}

impl Vertex {
    pub fn from_pos(x: f32, y: f32, z: f32) -> Self {
        Self {
            entity_id: 0,
            position: Vector3::new(x, y, z),
            normal: Vector3::new(0.0, 1.0, 0.0),
            tex_coords: Vector2::new(0.0, 0.0)
        }
    }

    pub fn from_vec3(vec: Vector3, normal: Vector3) -> Self {
        Self {
            entity_id: 0,
            position: vec,
            normal: normal,
            tex_coords: Vector2::new(0.0, 0.0)
        }
    }

    pub fn set_normal(mut self, normal: Vector3) {
        self.normal = normal;
    }
}

pub struct Texture {
    pub id: u32,
}

// Mesh only, not instance
#[allow(dead_code)]
pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    textures: Vec<Texture>, // Will be used later
    
    vbo: Vbo,
    vao: Vao,
    ibo: Ibo
    // ebo: GLuint
}

impl Mesh {
    pub fn setup_mesh(mut vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        let normals = calculate_normals(&vertices, &indices);
        for idx in 0..indices.len() {
            let normal_idx = idx / 3;
            if normal_idx >= normals.len() {
                break;
            }

            vertices[indices[idx] as usize].normal = normals[normal_idx].clone();
        }
        
        let vbo = Vbo::gen();
        vbo.set(&vertices);

        let vao = Vao::gen();
        vao.set();

        let ibo = Ibo::gen();
        ibo.set(&indices);

        unsafe {
            gl::BindVertexArray(vao.id);
        }

        Mesh {
            vertices,
            indices,
            textures,
            vbo,
            vao,
            ibo
        }
    }

    pub fn from_obj_file(file_path: &str) -> Self {
        let (vertices, indices) = file_utils::read_obj_file(file_path);
        let textures: Vec<Texture> = Vec::new();

        return Mesh::setup_mesh(vertices, indices, textures)
    }

    
    pub fn draw(&self) {
        unsafe {
            // gl::BindVertexArray(self.vao.id);
            gl::DrawElements(gl::TRIANGLES, self.indices().len() as i32, gl::UNSIGNED_INT, 0 as *const _);
            // gl::BindVertexArray(0);
        }
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices 
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

pub fn calculate_normals(vertices: &Vec<Vertex>, indices: &Vec<u32>) -> Vec<Vector3> {
    let mut normals: Vec<Vector3> = Vec::new();
    for offset in 0..indices.len() / 3 {
        if offset + 1 >= vertices.len() {
            break;
        }

        let vert_1 = &vertices[offset];
        let vert_2 = &vertices[offset + 1];

        let vert_normal = Vector3::new(
            vert_1.position.y() * vert_2.position.z() - vert_1.position.z() * vert_2.position.y(),
            vert_1.position.z() * vert_2.position.x() - vert_1.position.x() * vert_2.position.z(),
            vert_1.position.x() * vert_2.position.y() - vert_1.position.y() * vert_2.position.x()
        );

        normals.push(vert_normal);
    }

    return normals
}
