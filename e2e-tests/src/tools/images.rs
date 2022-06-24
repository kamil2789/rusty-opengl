use rusty_opengl::config::Window;

pub fn save_screen_as_img_png(window: &Window, image_name: &str) {
    let (width, height) = window.get_framebuffer_size();
    unsafe {
        gl::PixelStorei(gl::PACK_ALIGNMENT, 4);
        gl::ReadBuffer(gl::FRONT);

        let nr_channels = 3;
        let mut stride = nr_channels * width;
        if stride % 4 == 0 {
            stride += 4 - (stride % 4);
        }

        let buffer_size: usize = (stride * height).try_into().unwrap();
        let mut buffer: Vec<u8> = vec![0; buffer_size - 2400];

        gl::ReadPixels(
            0,
            0,
            width,
            height,
            gl::RGB,
            gl::UNSIGNED_BYTE,
            buffer.as_mut_ptr().cast::<std::ffi::c_void>(),
        );
        image::save_buffer(
            image_name,
            &buffer,
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
}
