use gl::types::{GLchar, GLint, GLuint};
use std::ffi::CString;
use std::ptr;

#[derive(Copy, Clone)]
enum ShaderType {
    Vertex,
    Fragment,
}

pub struct ShaderProgram<'s> {
    shader_program_id: u32,
    vert_src: &'s str,
    frag_src: &'s str,
}

fn shader_type_as_gluint(shader_type: ShaderType) -> GLuint {
    match shader_type {
        ShaderType::Vertex => gl::VERTEX_SHADER,
        ShaderType::Fragment => gl::FRAGMENT_SHADER,
    }
}

impl<'s> ShaderProgram<'s> {
    pub fn new(vert_src: &'s str, frag_src: &'s str) -> Self {
        let id = unsafe { gl::CreateProgram() };
        ShaderProgram {
            shader_program_id: id,
            vert_src,
            frag_src,
        }
    }

    pub fn set_fragment_shader(&mut self, src: &'s str) {
        self.frag_src = src;
    }

    pub fn set_vertex_shader(&mut self, src: &'s str) {
        self.vert_src = src;
    }

    pub fn compile(&mut self) -> bool {
        unsafe {
            let vertex_shader_id = self.compile_shader(ShaderType::Vertex);
            if vertex_shader_id == 0 {
                return false;
            }

            let fragment_shader_id = self.compile_shader(ShaderType::Fragment);
            if fragment_shader_id == 0 {
                return false;
            }

            gl::AttachShader(self.shader_program_id, vertex_shader_id);
            gl::AttachShader(self.shader_program_id, fragment_shader_id);
            gl::LinkProgram(self.shader_program_id);
            let result = ShaderProgram::check_link_status(self.shader_program_id);
            if !result {
                return false;
            }
            gl::DeleteShader(vertex_shader_id);
            gl::DeleteShader(fragment_shader_id);
        }

        true
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_id);
        }
    }

    fn match_shader_src(&self, shader_type: ShaderType) -> &str {
        match shader_type {
            ShaderType::Vertex => self.vert_src,
            ShaderType::Fragment => self.frag_src,
        }
    }

    unsafe fn compile_shader(&self, shader_type: ShaderType) -> GLuint {
        let shader = gl::CreateShader(shader_type_as_gluint(shader_type));
        let c_str_vert = CString::new(self.match_shader_src(shader_type).as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let status = ShaderProgram::check_compile_status(shader);
        if status {
            shader
        } else {
            0
        }
    }

    unsafe fn check_compile_status(shader_id: GLuint) -> bool {
        let mut status = gl::FALSE as GLint;
        let info_length: i32 = 512;
        let mut info_log = Vec::with_capacity(info_length as usize - 1);
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
        if status == gl::TRUE as GLint {
            true
        } else {
            gl::GetShaderInfoLog(
                shader_id,
                info_length,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "Shader compilation error\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            false
        }
    }

    unsafe fn check_link_status(shader_program_id: GLuint) -> bool {
        let mut status = gl::FALSE as GLint;
        let info_length: i32 = 512;
        let mut info_log = Vec::with_capacity(info_length as usize - 1);
        gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut status);
        if status == gl::TRUE as GLint {
            true
        } else {
            gl::GetProgramInfoLog(
                shader_program_id,
                info_length,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "Shader program link error\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            false
        }
    }
}

impl Drop for ShaderProgram<'_> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_program_id);
        }
    }
}
