use std::path::Path;

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
            Texture {
                id: 0,
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

    /// # Panics
    pub fn set_options(&mut self) {
        unsafe {
            //TMP
            gl::GenTextures(1, &mut self.id);
            //TMP
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
    pub fn init(&mut self) {
        unsafe {
            if self.id == 0 {
                gl::GenTextures(1, &mut self.id);
            }
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
