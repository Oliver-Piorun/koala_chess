pub fn initialize_open_gl_addresses(get_open_gl_address: fn(&str) -> *const std::ffi::c_void) {
    // Get and assign addresses

    // OpenGL <=1.1
    let _ = gl::BindTexture::load_with(get_open_gl_address);
    let _ = gl::Clear::load_with(get_open_gl_address);
    let _ = gl::ClearColor::load_with(get_open_gl_address);
    let _ = gl::Enable::load_with(get_open_gl_address);
    let _ = gl::GenTextures::load_with(get_open_gl_address);
    let _ = gl::GetString::load_with(get_open_gl_address);
    let _ = gl::TexImage2D::load_with(get_open_gl_address);
    let _ = gl::TexParameteri::load_with(get_open_gl_address);
    let _ = gl::Viewport::load_with(get_open_gl_address);

    // OpenGL >1.1
    let _ = gl::AttachShader::load_with(get_open_gl_address);
    let _ = gl::BindBuffer::load_with(get_open_gl_address);
    let _ = gl::BindVertexArray::load_with(get_open_gl_address);
    let _ = gl::BlendFunc::load_with(get_open_gl_address);
    let _ = gl::BufferData::load_with(get_open_gl_address);
    let _ = gl::CompileShader::load_with(get_open_gl_address);
    let _ = gl::CreateProgram::load_with(get_open_gl_address);
    let _ = gl::CreateShader::load_with(get_open_gl_address);
    let _ = gl::DeleteShader::load_with(get_open_gl_address);
    let _ = gl::DrawElements::load_with(get_open_gl_address);
    let _ = gl::EnableVertexAttribArray::load_with(get_open_gl_address);
    let _ = gl::GenBuffers::load_with(get_open_gl_address);
    let _ = gl::GenerateMipmap::load_with(get_open_gl_address);
    let _ = gl::GetProgramInfoLog::load_with(get_open_gl_address);
    let _ = gl::GetProgramiv::load_with(get_open_gl_address);
    let _ = gl::GetShaderInfoLog::load_with(get_open_gl_address);
    let _ = gl::GetShaderiv::load_with(get_open_gl_address);
    let _ = gl::GetUniformLocation::load_with(get_open_gl_address);
    let _ = gl::GenVertexArrays::load_with(get_open_gl_address);
    let _ = gl::LinkProgram::load_with(get_open_gl_address);
    let _ = gl::ShaderSource::load_with(get_open_gl_address);
    let _ = gl::Uniform1f::load_with(get_open_gl_address);
    let _ = gl::UniformMatrix4fv::load_with(get_open_gl_address);
    let _ = gl::UseProgram::load_with(get_open_gl_address);
    let _ = gl::VertexAttribPointer::load_with(get_open_gl_address);
}
