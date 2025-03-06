extern crate gl;
extern crate sdl2;

use std::fmt::Error;

macro_rules! check_bindings {
    [ $( gl::$i:ident ),+ ] => {
        $(
            if !gl::$i::is_loaded() {
                panic!("Could not load {}", stringify!($i))
            };
        )+
    };
}

/// Loads gl bindings from the sdl2 video subsystem and gl-rs.
/// Checks that each listed gl function has been loaded 
pub fn load_gl_bindings(video_subsystem: &sdl2::VideoSubsystem) -> Result<(), Error> {
    gl::load_with(|c| video_subsystem.gl_get_proc_address(c) as *const _ );
    check_bindings![
        // Buffers
        gl::GenBuffers,
        gl::BindBuffer,
        gl::BufferData,
        gl::GenVertexArrays,

        // Shader Related Bindings
        gl::CreateProgram,
        gl::LinkProgram,
        gl::UseProgram,

        gl::CreateShaderProgramv,
        gl::CreateShader,
        gl::ShaderBinary,
        gl::ShaderSource,
        gl::CompileShader,
        gl::GetShaderiv,
        gl::GetShaderInfoLog,
        gl::AttachShader,
        gl::DetachShader,
        gl::DeleteShader
    ];
    Ok(())
}