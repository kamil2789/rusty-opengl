use std::path::Path;

#[derive(Copy, Clone)]
pub enum Filtering {
    Linear = 0x2601,
    Nearest = 0x2600,
}

#[derive(Copy, Clone)]
pub enum Wrapping {
    Repeat = 0x2901,
    Mirrored = 0x8370,
    ClampToEdge = 0x812F,
    ClampToBorder = 0x812D,
}

#[derive(Clone)]
pub struct Texture {
    id: u32,
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Texture {
    /// # Panics
    ///
    /// Will panic if provided path to the image is invalid
    #[must_use]
    pub fn new(image_path: &Path) -> Self {
        if let Ok(img) = image::open(&image_path) {
            let mut id = 0;
            unsafe {
                gl::GenTextures(1, &mut id);
            }
            Texture {
                id,
                width: img.width(),
                height: img.height(),
                data: img.into_bytes(),
            }
        } else {
            panic!(
                "Failed to load texture at path {}",
                image_path.to_str().unwrap()
            );
        }
    }

    pub fn set_filtering(&self, filtering: Filtering) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as i32);
        }
    }

    pub fn set_wrapping(&self, wrapping: Wrapping) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping as i32);
        }
    }

    /// # Panics
    pub fn set_default(&mut self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_S,
                gl::REPEAT.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_WRAP_T,
                gl::REPEAT.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR.try_into().unwrap(),
            );
        }
    }

    /// # Panics
    ///
    /// Will panic if provided image is invalid
    pub fn generate_mipmap(&mut self) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB.try_into().unwrap(),
                self.width.try_into().unwrap(),
                self.height.try_into().unwrap(),
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                self.data.as_ptr().cast::<std::ffi::c_void>(),
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
