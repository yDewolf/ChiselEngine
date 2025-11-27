use crate::engine::graphics::transform::{Vector2, Vector3};
use crate::engine::graphics::opengl::gl_objects::{Ibo, Vao, Vbo};

use crate::utils::{obj_parser};

#[derive(Clone, Copy)]
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

    pub fn set_normal(&mut self, normal: Vector3) {
        self.normal = normal;
    }

    pub fn add_normal(&mut self, normal: Vector3) {
        self.normal = self.normal + normal;
    }

    pub fn normalize_normal(&mut self) {
        self.normal = self.normal.normalized();
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
    indice_len: usize,

    textures: Vec<Texture>, // Will be used later
    
    vbo: Vbo,
    vao: Vao,
    ibo: Ibo
    // ebo: GLuint
}

impl Mesh {
    pub fn setup_mesh(mut vertices: Vec<Vertex>, indices: Vec<u32>, textures: Vec<Texture>) -> Self {
        // let vert = update_normals(vertices, &indices);
        let vert = vertices;
        
        for i in 0..indices.len() / 3 {
            println!("{}, {}, {}", indices[i * 3], indices[i * 3 + 1], indices[i * 3 + 2]);
        }

        let vbo = Vbo::generate();
        vbo.set(&vert);

        let vao = Vao::generate();
        vao.set();

        let ibo = Ibo::generate();
        ibo.set(&indices);

        // unsafe {
        //     gl::BindVertexArray(vao.id);
        // }

        Mesh {
            vertices: vert,
            indice_len: indices.len() / 2,
            indices,
            textures,
            vbo,
            vao,
            ibo,
        }
    }

    pub fn from_obj_file(file_path: &str) -> Self {
        // let (vertices, indices) = file_utils::read_obj_file(file_path);
        let obj = obj_parser::Obj::from_path(file_path);

        let textures: Vec<Texture> = Vec::new();

        return Mesh::setup_mesh(obj.vertices, obj.indices, textures)
    }

    
    pub fn draw(&mut self) {
        // self.indice_len += 1;
        // if self.indice_len > self.indices().len() {
        //     self.indice_len = 0;
        // }
        unsafe {
            // gl::BindVertexArray(self.vao.id);
            gl::DrawElements(gl::TRIANGLES, (self.indice_len) as i32, gl::UNSIGNED_INT, 0 as *const _);
            gl::DrawElements(gl::LINE_STRIP, (self.indice_len) as i32, gl::UNSIGNED_INT, 0 as *const _);
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

pub fn update_normals(mut vertices: Vec<Vertex>, indices: &Vec<u32>) -> Vec<Vertex> {
    let mut normals: Vec<Vector3> = Vec::new();

    for offset in 0..indices.len() / 3 {
        let vert_1 = vertices[indices[offset + 1] as usize].position.clone(); //- vertices[indices[offset] as usize].position.clone();
        let vert_2 = vertices[indices[offset + 2] as usize].position.clone(); //- vertices[indices[offset] as usize].position.clone();

        let vert_normal = vert_1.cross(&vert_2);

        vertices[indices[offset] as usize].normal = vertices[indices[offset] as usize].normal.clone() + vert_normal.clone();
        vertices[indices[offset + 1] as usize].normal = vertices[indices[offset + 1] as usize].normal.clone() + vert_normal.clone();
        vertices[indices[offset + 2] as usize].normal = vertices[indices[offset + 2] as usize].normal.clone() + vert_normal.clone();

        // vertices[indices[offset] as usize].normal = vert_normal.clone();
        // vertices[indices[offset + 1] as usize].normal = vert_normal.clone();
        // vertices[indices[offset + 2] as usize].normal = vert_normal.clone();

        normals.push(vert_normal);
    }

    for idx in 0..vertices.len() {
        vertices[idx].normal = vertices[idx].normal.normalized();
    }

    println!("{}", normals.len());
    // println!("{:?}", normals);

    return vertices
}
