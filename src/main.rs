use sdl2::event::{Event, WindowEvent};

pub mod engine;
pub mod utils;

use engine::graphics::opengl::winsdl::Winsdl;
use engine::graphics::opengl::gl_objects::{create_program, Ibo, Uniform, Vao, Vbo};
use engine::graphics::transform::{Mat3, Mat4};
use engine::builtin::nodes;

use engine::geometry::mesh::{self, Mesh};

fn main() {
    let mesh = Mesh::from_obj_file("assets/test_models/cube.obj");
    println!("{}", mesh.indices().len());

    println!("Hello, world!");

    let mut winsdl = Winsdl::new(800, 600).unwrap();

    let mut max_uniforms: gl::types::GLint = 0;
    unsafe { gl::GetIntegerv(gl::MAX_UNIFORM_LOCATIONS, &mut max_uniforms); }
    println!("Maximum number of uniforms {}", max_uniforms);

    let program = create_program().unwrap();
    program.set();  

    let vbo = Vbo::gen();
    vbo.set(mesh.vertices());

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(mesh.indices());

    let mut model_matrix: Mat4 = Mat4::new();
    let mut view_matrix: Mat4 = Mat4::new();
    let mut projection_matrix: Mat4 = Mat4::new();
    projection_matrix.project_perspective(-1.0, 1.0, -1.0, 1.0, 5.0, 0.0);
    view_matrix.lookat(0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0);


    let u_resolution: Uniform = Uniform::new(program.id(), "u_resolution").unwrap();

    let u_model_matrix: Uniform = Uniform::new(program.id(), "u_model_matrix").unwrap();
    let u_view_matrix: Uniform = Uniform::new(program.id(), "u_view_matrix").unwrap();
    let u_projection_matrix: Uniform = Uniform::new(program.id(), "u_projection_matrix").unwrap();


    unsafe { 
        gl::Uniform2f(u_resolution.id, winsdl.window.size().0 as f32, winsdl.window.size().1 as f32);
        gl::UniformMatrix4fv(u_projection_matrix.id, 1, gl::TRUE, projection_matrix.ptr());

    }

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                
                // Adjust objects positions in viewport using the viewport uv
                Event::Window {win_event, .. } => {
                    if let WindowEvent::Resized(width, height) = win_event {
                        unsafe {
                            gl::Viewport(0, 0, width, height);
                            gl::Uniform2f(u_resolution.id, width as f32, height as f32);
                        }
                    }
                }
                _   => {    }
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            model_matrix.rotate_y(0.05);

            gl::UniformMatrix4fv(u_model_matrix.id, 1, gl::TRUE, model_matrix.ptr());
            gl::UniformMatrix4fv(u_view_matrix.id, 1, gl::TRUE, view_matrix.ptr());

            gl::DrawElements(
                gl::TRIANGLES, 
                mesh.indices().len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _
            );
            
            // gl::ClearColor(0.0, 0.0, 0.1, 1.0);
        }


        winsdl.window.gl_swap_window();
    }

}


