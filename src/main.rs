use std::thread::current;

use sdl2::event::Event;

use crate::winsdl::Winsdl;

mod winsdl;


fn main() {
    println!("Hello, world!");

    let mut winsdl = Winsdl::new(800, 600).unwrap();

    let current_color: (f32, f32, f32) = (0.0, 0.0, 0.0);

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'running,
                _   => {    }
            }
        }

        unsafe {
            gl::ClearColor(current_color.0, current_color.1, current_color.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }


        winsdl.window.gl_swap_window();
    }

}
