use std::{fs::File, io::{self, BufRead}, path::Path};

use crate::engine::{geometry::mesh::Vertex, graphics::transform::Vector3};

pub fn read_obj_file(file_path: &str) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::new();
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
            // Indices
            if line.starts_with('f') {{
                let splitted: Vec<&str> = line.split(' ').collect();
                for i in 1..4 {
                    indices.push(splitted[i].parse::<u32>().unwrap() - 1);
                }

            }}
        }
        
    }

    return (vertices, indices);
}

pub fn read_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(file_path)?;
        Ok(io::BufReader::new(file).lines())
}
