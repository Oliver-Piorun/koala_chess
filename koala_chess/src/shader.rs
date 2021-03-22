use logger::*;
use std::{error::Error, fs::read_to_string};

#[derive(Copy, Clone)]
pub struct Shader {
    program: gl::types::GLuint,
}

impl Shader {
    pub fn new(
        vertex_shader_path: &str,
        fragment_shader_path: &str,
    ) -> Result<Shader, Box<dyn Error>> {
        let mut vertex_shader_code = read_to_string(vertex_shader_path)?;
        vertex_shader_code.push('\0');

        let mut fragment_shader_code = read_to_string(fragment_shader_path)?;
        fragment_shader_code.push('\0');

        unsafe {
            // Vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);

            gl::ShaderSource(
                vertex_shader,
                1,
                &(vertex_shader_code.as_ptr() as *const gl::types::GLchar),
                std::ptr::null::<gl::types::GLint>(),
            );

            gl::CompileShader(vertex_shader);

            check_for_shader_errors(vertex_shader, vertex_shader_path)?;

            // Fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_shader_code.as_ptr() as *const gl::types::GLchar),
                std::ptr::null::<gl::types::GLint>(),
            );

            gl::CompileShader(fragment_shader);

            check_for_shader_errors(fragment_shader, fragment_shader_path)?;

            // Program
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            check_for_program_errors(program)?;

            // Delete shaders. They are already linked into the program
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Ok(Shader { program })
        }
    }

    pub fn r#use(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_float(&self, name: &str, value: gl::types::GLfloat) -> Result<(), Box<dyn Error>> {
        let uniform_location = unsafe {
            gl::GetUniformLocation(self.program, name.as_ptr() as *const gl::types::GLchar)
        };

        if uniform_location == -1 {
            return Err(format!(
                "Could not get uniform location! (name: {}, value: {})",
                name, value
            )
            .into());
        }

        unsafe {
            gl::Uniform1f(uniform_location, value);
        }

        Ok(())
    }
}

fn check_for_shader_errors(
    shader: gl::types::GLuint,
    shader_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut success: gl::types::GLint = 0;

    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    if success == gl::TRUE as gl::types::GLint {
        logger::info!("Shader compile status: success ({})", shader_path);
    } else {
        let mut log: [gl::types::GLchar; 1024] = [0; 1024];

        unsafe {
            gl::GetShaderInfoLog(
                shader,
                1024,
                std::ptr::null_mut::<gl::types::GLsizei>(),
                log.as_mut_ptr(),
            )
        };

        let log_cstr = unsafe { std::ffi::CStr::from_ptr(log.as_ptr()) };

        match log_cstr.to_str() {
            Ok(log_str) => {
                return Err(format!("Shader compile error: {}! ({})", log_str, shader_path).into())
            }
            Err(e) => return Err(format!("Could not create a CStr from a pointer! ({})", e).into()),
        }
    }

    Ok(())
}

fn check_for_program_errors(program: gl::types::GLuint) -> Result<(), Box<dyn Error>> {
    let mut success: gl::types::GLint = 0;

    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    }

    if success == gl::TRUE as gl::types::GLint {
        logger::info!("Shader program link status: success");
    } else {
        let mut log: [gl::types::GLchar; 1024] = [0; 1024];

        unsafe {
            gl::GetProgramInfoLog(
                program,
                1024,
                std::ptr::null_mut::<gl::types::GLsizei>(),
                log.as_mut_ptr(),
            )
        };

        let log_cstr = unsafe { std::ffi::CStr::from_ptr(log.as_ptr()) };

        match log_cstr.to_str() {
            Ok(log_str) => return Err(format!("Shader program link error: {}!", log_str).into()),
            Err(e) => return Err(format!("Could not create a CStr from a pointer! ({})", e).into()),
        }
    }

    Ok(())
}
