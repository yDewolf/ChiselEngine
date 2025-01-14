use std::{fs::File, io::{self, BufRead}, path::Path};

use crate::engine::{geometry::mesh::Vertex, graphics::transform::Vector3};

pub fn read_obj_file(file_path: &str) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut normals: Vec<Vector3> = Vec::new();
    
    // Vec<[vert_idx, normal_idx]>
    let mut normal_setters: Vec<[usize; 2]> = Vec::new();

    let mut indices: Vec<u32> = Vec::new();
    
    if !file_path.ends_with(".obj") {
        panic!("Trying to read obj with a non .obj file");
    }

    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String

        for line in lines.flatten() {
            // Vertices
            if line.starts_with('v') {
                let splitted: Vec<&str> = line.split(' ').collect();
                let pos = Vector3::new(
                    splitted[1].parse::<f32>().unwrap(),
                    splitted[2].parse::<f32>().unwrap(), 
                    splitted[3].parse::<f32>().unwrap()
                );

                vertices.push(Vertex::from_vec3(pos, Vector3::new(0.0, 1.0, 0.0)));
            }

            if line.starts_with("vn") {
                let splitted: Vec<&str> = line.split(' ').collect();
                let normal = Vector3::new(
                    splitted[1].parse::<f32>().unwrap(),
                    splitted[2].parse::<f32>().unwrap(), 
                    splitted[3].parse::<f32>().unwrap()
                );

                normals.push(normal);
                println!("Normals Len: {}", normals.len());
            }

            // Indices
            if line.starts_with('f') {{
                let splitted: Vec<&str> = line.split(' ').collect();
                for i in 1..4 {
                    let slice = splitted[i];
                    if slice.contains("//") {
                        let normal_split: Vec<&str> = slice.split("//").collect();

                        let vert_idx = normal_split[0].parse::<u32>().unwrap() - 1;
                        let normal_idx = normal_split[1].parse::<u32>().unwrap() - 1;

                        normal_setters.push([vert_idx as usize, normal_idx as usize]);
                        indices.push(vert_idx);
                    } else {
                        indices.push(splitted[i].parse::<u32>().unwrap() - 1);
                    }

                }
            }}
        }
        
    }

    for setter in normal_setters {
        // let vertex = &vertices[setter[0]];
        vertices[setter[0]].set_normal(normals[setter[1]].clone());
    }

    return (vertices, indices);
}

pub fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(file_path)?;
        Ok(io::BufReader::new(file).lines())
}
