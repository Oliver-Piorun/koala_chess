pub fn initialize_open_gl_addresses(get_open_gl_address: fn(&str) -> *const std::ffi::c_void) {
    // Get and assign addresses

    // OpenGL <=1.1
    gl::BindTexture::load_with(get_open_gl_address);
    gl::Clear::load_with(get_open_gl_address);
    gl::ClearColor::load_with(get_open_gl_address);
    gl::Enable::load_with(get_open_gl_address);
    gl::GenTextures::load_with(get_open_gl_address);
    gl::GetString::load_with(get_open_gl_address);
    gl::TexImage2D::load_with(get_open_gl_address);
    gl::TexParameteri::load_with(get_open_gl_address);
    gl::Viewport::load_with(get_open_gl_address);

    // OpenGL >1.1
    gl::AttachShader::load_with(get_open_gl_address);
    gl::BindBuffer::load_with(get_open_gl_address);
    gl::BindVertexArray::load_with(get_open_gl_address);
    gl::BlendFunc::load_with(get_open_gl_address);
    gl::BufferData::load_with(get_open_gl_address);
    gl::CompileShader::load_with(get_open_gl_address);
    gl::CreateProgram::load_with(get_open_gl_address);
    gl::CreateShader::load_with(get_open_gl_address);
    gl::DeleteShader::load_with(get_open_gl_address);
    gl::DrawElements::load_with(get_open_gl_address);
    gl::EnableVertexAttribArray::load_with(get_open_gl_address);
    gl::GenBuffers::load_with(get_open_gl_address);
    gl::GenerateMipmap::load_with(get_open_gl_address);
    gl::GetProgramInfoLog::load_with(get_open_gl_address);
    gl::GetProgramiv::load_with(get_open_gl_address);
    gl::GetShaderInfoLog::load_with(get_open_gl_address);
    gl::GetShaderiv::load_with(get_open_gl_address);
    gl::GetUniformLocation::load_with(get_open_gl_address);
    gl::GenVertexArrays::load_with(get_open_gl_address);
    gl::LinkProgram::load_with(get_open_gl_address);
    gl::ShaderSource::load_with(get_open_gl_address);
    gl::Uniform1f::load_with(get_open_gl_address);
    gl::UniformMatrix4fv::load_with(get_open_gl_address);
    gl::UseProgram::load_with(get_open_gl_address);
    gl::VertexAttribPointer::load_with(get_open_gl_address);
}
