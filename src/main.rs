extern crate sdl2;
extern crate gl;

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

fn shader_from_source(source: &CStr, kind: gl::types::GLenum) -> Result<gl::types::GLuint, String>{
    let id = unsafe{gl::CreateShader(kind)};

    unsafe{
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;

    unsafe{
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    if success == 0{
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

        buffer.extend([b' '].iter().cycle().take(len as usize));

        let error : CString = unsafe {CString::from_vec_unchecked(buffer)};

        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );

            return Err(error.to_string_lossy().into_owned());
        }
    }

    return Ok(id);
}
