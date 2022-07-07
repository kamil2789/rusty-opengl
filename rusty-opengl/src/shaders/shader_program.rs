use gl::types::GLuint;
use std::ffi::CString;
use std::ptr;

#[derive(Copy, Clone)]
enum ShaderType {
    Vertex,
    Fragment,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

pub struct ShaderProgram {
    shader_program_id: u32,
    vert_src: String,
    frag_src: String,
    is_compiled: bool,
}

fn shader_type_as_gluint(shader_type: ShaderType) -> GLuint {
    match shader_type {
        ShaderType::Vertex => gl::VERTEX_SHADER,
        ShaderType::Fragment => gl::FRAGMENT_SHADER,
    }
}

impl ShaderProgram {
    #[must_use]
    pub fn new(vert_src: &str, frag_src: &str) -> Self {
        let id = unsafe { gl::CreateProgram() };
        ShaderProgram {
            shader_program_id: id,
            vert_src: String::from(vert_src),
            frag_src: String::from(frag_src),
            is_compiled: false,
        }
    }

    pub fn set_fragment_shader(&mut self, src: &str) {
        self.frag_src = String::from(src);
        self.is_compiled = false;
    }

    pub fn set_vertex_shader(&mut self, src: &str) {
        self.vert_src = String::from(src);
        self.is_compiled = false;
    }

    pub fn compile(&mut self) -> bool {
        if self.is_compiled {
            return true;
        }

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

        self.is_compiled = true;
        true
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.shader_program_id);
        }
    }

    pub fn deactivae(&self) {
        unsafe {
            gl::UseProgram(0);
        } 
    }

    #[must_use]
    pub fn is_compiled(&self) -> bool {
        self.is_compiled
    }

    pub fn set_uniform4f_variable(&self, variable: &str, value: Color) -> bool {
        self.activate();
        unsafe {
            let c_variable = CString::new(variable).unwrap();
            let uniform_location = gl::GetUniformLocation(self.shader_program_id, c_variable.as_ptr());
            if uniform_location == -1 {
                self.deactivae();
                return false;
            }

            gl::Uniform4f(uniform_location, value.r, value.g, value.b, value.a);
            self.deactivae();
            true
        }
    }

    fn match_shader_src(&self, shader_type: ShaderType) -> &String {
        match shader_type {
            ShaderType::Vertex => &self.vert_src,
            ShaderType::Fragment => &self.frag_src,
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
        let mut status = i32::from(gl::TRUE);
        let info_length: usize = 512;
        let mut info_log: Vec<u8> = Vec::with_capacity(info_length - 1);
        gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut status);
        if status == i32::from(gl::TRUE) {
            true
        } else {
            gl::GetShaderInfoLog(
                shader_id,
                info_length.try_into().unwrap(),
                ptr::null_mut(),
                info_log.as_mut_ptr().cast::<i8>(),
            );
            println!(
                "Shader compilation error\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            false
        }
    }

    unsafe fn check_link_status(shader_program_id: GLuint) -> bool {
        let mut status = i32::from(gl::FALSE);
        let info_length: usize = 512;
        let mut info_log: Vec<u8> = Vec::with_capacity(info_length - 1);
        gl::GetProgramiv(shader_program_id, gl::LINK_STATUS, &mut status);
        if status == i32::from(gl::TRUE) {
            true
        } else {
            gl::GetProgramInfoLog(
                shader_program_id,
                info_length.try_into().unwrap(),
                ptr::null_mut(),
                info_log.as_mut_ptr().cast::<i8>(),
            );
            println!(
                "Shader program link error\n{}",
                std::str::from_utf8(&info_log).unwrap()
            );
            false
        }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_program_id);
        }
    }
}
