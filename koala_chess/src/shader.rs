use logger::*;
use std::fs::read_to_string;

#[derive(Copy, Clone)]
pub struct Shader {
    program: gl::types::GLuint,
}

impl Shader {
    pub fn new(vertex_shader_path: &str, fragment_shader_path: &str) -> Shader {
        let mut vertex_shader_code = read_to_string(vertex_shader_path).unwrap();
        vertex_shader_code.push('\0');

        let mut fragment_shader_code = read_to_string(fragment_shader_path).unwrap();
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

            check_for_shader_errors(vertex_shader, vertex_shader_path);

            // Fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);

            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_shader_code.as_ptr() as *const gl::types::GLchar),
                std::ptr::null::<gl::types::GLint>(),
            );

            gl::CompileShader(fragment_shader);

            check_for_shader_errors(fragment_shader, fragment_shader_path);

            // Program
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);

            check_for_program_errors(program);

            // Delete shaders. They are already linked into the program
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            Shader { program }
        }
    }

    pub fn r#use(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn set_float(&self, name: &str, value: gl::types::GLfloat) {
        let uniform_location = unsafe {
            gl::GetUniformLocation(self.program, name.as_ptr() as *const gl::types::GLchar)
        };

        if uniform_location == -1 {
            // TODO: Error handling
            logger::error!(
                "Could not get uniform location! (name: {}, value: {})",
                name,
                value
            );
            return;
        }

        unsafe {
            gl::Uniform1f(uniform_location, value);
        }
    }
}

fn check_for_shader_errors(shader: gl::types::GLuint, shader_path: &str) {
    let mut success: gl::types::GLint = 0;

    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }

    logger::info!("Shader compile status: {} ({})", success, shader_path);

    if success == 0 {
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
        let log_str = log_cstr.to_str().unwrap();

        logger::error!("Shader compile error: {} ({})", log_str, shader_path);
    }
}

fn check_for_program_errors(program: gl::types::GLuint) {
    let mut success: gl::types::GLint = 0;

    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
    }

    logger::info!("Shader program link status: {}", success);

    if success == 0 {
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
        let log_str = log_cstr.to_str().unwrap();

        logger::error!("Shader program link error: {}", log_str);
    }
}
