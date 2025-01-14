use crate::engine::{geometry::mesh::Vertex, graphics::transform::Vector3};

use super::file_utils;

pub struct Obj {
    path: String,

    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,

    normals: Vec<Vector3>,
}

impl Obj {
    pub fn from_path(file_path: &str) -> Self {
        let (vertices, indices, normals) = Obj::get_obj_file_data(file_path);

        Obj {
            path: String::from(file_path),

            vertices,
            indices,
            normals
        }
    }

    fn get_obj_file_data(file_path: &str) -> (Vec<Vertex>, Vec<u32>, Vec<Vector3>) {
        let mut vertex_pool: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut normal_pool: Vec<Vector3> = Vec::new();

        let lines = file_utils::read_lines(file_path).unwrap();
        
        for line in lines.flatten() {
            let splitted: Vec<&str> = line.split(' ').collect();
            match splitted[0] {
                "v" => {
                    vertex_pool.push( Vertex::from_pos(
                            splitted[1].parse::<f32>().unwrap(),
                            splitted[2].parse::<f32>().unwrap(), 
                            splitted[3].parse::<f32>().unwrap()
                        )
                    );
                }

                "vn" => {
                    normal_pool.push(
                        Vector3::new(
                            splitted[1].parse::<f32>().unwrap(),
                            splitted[2].parse::<f32>().unwrap(), 
                            splitted[3].parse::<f32>().unwrap()
                        )
                    );
                }

                "f" => {
                    for idx in 1..splitted.len() {
                        if splitted[idx].contains("//") {
                            let slice: Vec<&str> = splitted[idx].split("//").collect();

                            let vert_idx = slice[0].parse::<u32>().unwrap() - 1;
                            let normal_idx = slice[1].parse::<u32>().unwrap() - 1;

                            vertex_pool[vert_idx as usize].add_normal(normal_pool[normal_idx as usize]);

                            indices.push(vert_idx);

                        } else if splitted[idx].contains("/") {
                            
                        } else {
                            let vert_idx = splitted[idx].parse::<u32>().unwrap();

                            indices.push(vert_idx);
                        }
                    }
                }
                
                _ => ()
            }
        }

        for vert_idx in 0..vertex_pool.len() {
            vertex_pool[vert_idx].normalize_normal();
        }

        (vertex_pool, indices, normal_pool)
    }
}