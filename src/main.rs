use sdl2::event::Event;
use gl_objects::{create_program, Ibo, Vao, Vbo};

use crate::winsdl::Winsdl;

mod winsdl;

mod gl_objects;

fn main() {
    println!("Hello, world!");

    let mut winsdl = Winsdl::new(800, 600).unwrap();

    let program = create_program().unwrap();
    program.set();  

    let vertices = vec![
        -0.5, -0.5,
        0.5, -0.5,
        0.0, 0.5,
    ];

    let indices = vec![
        0, 1, 2
    ];

    let vbo = Vbo::gen();
    vbo.set(&vertices);

    let vao = Vao::gen();
    vao.set();

    let ibo = Ibo::gen();
    ibo.set(&indices);

    let current_color: (f32, f32, f32) = (0.5, 0.5, 0.5);


    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                _   => {    }
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            
            
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as *const _
            );
            gl::ClearColor(current_color.0, current_color.1, current_color.2, 1.0);
        }


        winsdl.window.gl_swap_window();
    }

}
