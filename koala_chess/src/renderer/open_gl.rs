pub fn initialize_open_gl_addresses(get_open_gl_address: fn(&str) -> *const std::ffi::c_void) {
    // Get and assign addresses

    // OpenGL <=1.1
    let _ = gl::BindTexture::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::Clear::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::ClearColor::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::Enable::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GenTextures::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GetString::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::TexImage2D::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::TexParameteri::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::Viewport::load_with(|function_name| get_open_gl_address(function_name));

    // OpenGL >1.1

    let _ = gl::AttachShader::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::BindBuffer::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::BindVertexArray::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::BlendFunc::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::BufferData::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::CompileShader::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::CreateProgram::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::CreateShader::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::DeleteShader::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::DrawElements::load_with(|function_name| get_open_gl_address(function_name));
    let _ =
        gl::EnableVertexAttribArray::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GenBuffers::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GenerateMipmap::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GetProgramInfoLog::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GetProgramiv::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GetShaderInfoLog::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GetShaderiv::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GetUniformLocation::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::GenVertexArrays::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::LinkProgram::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::ShaderSource::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::Uniform1f::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::UseProgram::load_with(|function_name| get_open_gl_address(function_name));
    let _ = gl::VertexAttribPointer::load_with(|function_name| get_open_gl_address(function_name));
}
