#[derive(Clone)]
pub struct ColorRGBA {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
}

impl ColorRGBA {
    pub fn new(red: u8, green: u8, blue: u8, alpha: f32) -> Self {
        ColorRGBA{r: red, g: green, b: blue, a: ColorRGBA::normalized_alpha(alpha)}
    }

    pub fn empty() -> Self {
        ColorRGBA{r: 0, g: 0, b: 0, a: 1_f32}
    }

    pub fn from_hex(color: u32) -> Self {
        let bytes = color.to_be_bytes();
        ColorRGBA{r: bytes[0], g: bytes[1], b: bytes[2], a: ColorRGBA::convert_from_u8_to_normalized_f32(bytes[3])}
    }

    pub fn get_as_normalized_f32(&self) -> [f32; 4] {
        let red = ColorRGBA::convert_from_u8_to_normalized_f32(self.r);
        let green = ColorRGBA::convert_from_u8_to_normalized_f32(self.g);
        let blue = ColorRGBA::convert_from_u8_to_normalized_f32(self.b);
        let alpha = self.a;
        [red, green, blue, alpha]
    }

    pub fn get_rgba(&self) -> (u8, u8, u8, f32) {
        (self.r, self.g, self.b, self.a)
    }

    fn convert_from_u8_to_normalized_f32(number: u8) -> f32 {
        (1_f32 / u8::MAX as f32) * number as f32
    }

    fn normalized_alpha(number: f32) -> f32 {
        if number > 1_f32 {
            return 1_f32;
        }

        if number < 0_f32 {
            return 0_f32;
        }

        number
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_color_rgba() {
        let red = ColorRGBA::new(255, 0, 0, 1_f32);
        assert_eq!((255, 0, 0, 1_f32), red.get_rgba());
    }

    #[test]
    fn test_new_color_empty() {
        let color = ColorRGBA::empty();
        assert_eq!((0, 0, 0, 1_f32), color.get_rgba());
    }

    #[test]
    fn test_new_rgba_alpha_not_in_scope() {
        let oversized = ColorRGBA::new(255, 0, 0, 23_f32);
        assert_eq!((255, 0, 0, 1_f32), oversized.get_rgba());

        let minus_value = ColorRGBA::new(255, 0, 0, -0.4_f32);
        assert_eq!((255, 0, 0, 0_f32), minus_value.get_rgba());
    }

    #[test]
    fn test_new_color_rgba_from_hex() {
        let color = ColorRGBA::from_hex(0xff00ffff);
        assert_eq!((255, 0, 255, 1_f32), color.get_rgba());
    }

    #[test]
    fn test_get_as_normalized_f32() {
        let color = ColorRGBA::new(255, 0, 0, 1_f32);
        let rgba = color.get_as_normalized_f32();
        assert_eq!([1_f32, 0.0, 0.0, 1_f32], rgba);
    }
}