extern crate sdl2;
extern crate gl;

pub mod render_gl;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use std::ffi::{CString, CStr};

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
    .opengl()
    .resizable()
    .position_centered()
    .build()
    .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl_context.event_pump().unwrap();

    unsafe{
        gl::Viewport(0,0,800,600);
        gl::ClearColor(0.3,0.3,0.5,1.0);
    }

    use std::ffi::CString;

    let vert_shader = render_gl::Shader::from_vertex_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();
    
    let frag_shader = render_gl::Shader::from_fragment_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();
    
    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

    'main: loop{
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }

            unsafe{
                gl::Clear(gl::COLOR_BUFFER_BIT);
                
            }

            window.gl_swap_window();

        }
    }
}


