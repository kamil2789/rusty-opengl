use crate::polygons::RGBA;

#[derive(Copy, Clone)]
pub enum VertexLocation {
    Position,
    Color,
    Texture,
}

#[derive(Clone)]
pub struct Vertices {
    vert_pos: Vec<f32>,
    colors_pos: Vec<f32>,
    textures_pos: Vec<f32>,
}

impl Vertices {
    #[must_use]
    pub fn new(vert_pos: Vec<f32>, colors_pos: Vec<f32>, textures_pos: Vec<f32>) -> Self {
        Vertices {
            vert_pos,
            colors_pos,
            textures_pos,
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Vertices {
            vert_pos: vec![],
            colors_pos: vec![],
            textures_pos: vec![],
        }
    }

    pub fn set_position(&mut self, position: &[f32], location: VertexLocation) {
        match location {
            VertexLocation::Position => {
                self.vert_pos = Vertices::build_data(self.vert_pos.clone(), position);
            }
            VertexLocation::Color => {
                self.colors_pos = Vertices::build_data(self.colors_pos.clone(), position);
            }
            VertexLocation::Texture => {
                self.textures_pos = Vertices::build_data(self.textures_pos.clone(), position);
            }
        }
    }

    pub fn append_colors(&mut self, colors: &[RGBA]) {
        for color in colors.iter() {
            self.colors_pos
                .extend_from_slice(&color.get_as_normalized_f32());
        }
    }

    pub fn set_one_color_for_all_vert(&mut self, color: &RGBA) {
        self.colors_pos.clear();
        let size = self.vert_pos.len() / 3;
        for _ in 0..size {
            self.colors_pos
                .extend_from_slice(&color.get_as_normalized_f32());
        }
    }

    #[must_use]
    pub fn is_triangle(&self) -> bool {
        self.vert_pos.len() == 9
    }

    #[must_use]
    pub fn is_reactangle(&self) -> bool {
        self.vert_pos.len() == 12
    }

    #[must_use]
    pub fn create_single_vertices_array(&self) -> Option<Vec<f32>> {
        if !self.is_valid_structure() {
            return None;
        }

        let mut result = Vec::with_capacity(self.sum_capacity());
        let is_texture = !self.textures_pos.is_empty();
        let is_color = !self.colors_pos.is_empty();
        let vertices = self.vert_pos.len() / 3;

        let mut vert_pos_index = 0;
        let mut color_pos_index = 0;
        let mut texture_pos_index = 0;
        for _ in 0..vertices {
            for _ in 0..3 {
                result.push(self.vert_pos[vert_pos_index]);
                vert_pos_index += 1;
            }

            if is_color {
                for _ in 0..4 {
                    result.push(self.colors_pos[color_pos_index]);
                    color_pos_index += 1;
                }
            }

            if is_texture {
                for _ in 0..2 {
                    result.push(self.textures_pos[texture_pos_index]);
                    texture_pos_index += 1;
                }
            }
        }

        Some(result)
    }

    #[must_use]
    pub fn get_stride(&self) -> i32 {
        let mut result = 3;
        if !self.colors_pos.is_empty() {
            result = 7;
        }

        if !self.textures_pos.is_empty() {
            result = 9;
        }

        (result * std::mem::size_of::<f32>()) as i32
    }

    #[must_use]
    pub fn sum_capacity(&self) -> usize {
        self.vert_pos.len() + self.colors_pos.len() + self.textures_pos.len()
    }

    #[must_use]
    pub fn is_texture(&self) -> bool {
        !self.textures_pos.is_empty()
    }

    fn build_data(mut data: Vec<f32>, position: &[f32]) -> Vec<f32> {
        if data.is_empty() || data.len() < position.len() {
            data.clear();
            for item in position {
                data.push(*item);
            }
        }

        if data.len() >= position.len() {
            for (i, item) in position.iter().enumerate() {
                data[i] = *item;
            }
            data.shrink_to_fit();
        }

        data
    }

    fn is_valid_structure(&self) -> bool {
        let mut valid_colors = true;
        let mut valid_texture = true;
        if !self.colors_pos.is_empty() {
            let output: f32 = self.vert_pos.len() as f32 / self.colors_pos.len() as f32;
            valid_colors = output.to_bits() == (0.75_f32).to_bits();
        }

        if !self.textures_pos.is_empty() {
            let output = self.vert_pos.len() as f32 / self.textures_pos.len() as f32;
            valid_texture = output.to_bits() == (1.5_f32).to_bits();
        }

        valid_texture && valid_colors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_vertices() {
        let vertices = Vertices::empty();
        let size =
            vertices.vert_pos.len() + vertices.colors_pos.len() + vertices.textures_pos.len();
        assert_eq!(0, size);
    }

    #[test]
    fn test_create_vertices() {
        let vertices = Vertices::new(
            vec![0.5, 0.5, 0.0],
            vec![1.0, 0.5, 0.0, 1.0],
            vec![1.0, 1.0],
        );
        let size = vertices.sum_capacity();
        assert_eq!(9, size);
    }

    #[test]
    fn test_create_vertices_only_positions() {
        let mut vertices = Vertices::empty();
        let pos = vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0];
        vertices.set_position(&pos, VertexLocation::Position);
        let result = vertices.create_single_vertices_array();
        assert!(result.is_some());
        assert_eq!(pos, result.unwrap());
    }

    #[test]
    fn test_create_vertices_with_color() {
        let mut vertices = Vertices::empty();
        let pos = vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0];
        let color = vec![1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5];
        let expected = vec![
            -0.9, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, -0.5, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.5,
        ];
        vertices.set_position(&pos, VertexLocation::Position);
        vertices.set_position(&color, VertexLocation::Color);
        let result = vertices.create_single_vertices_array();
        assert!(result.is_some());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_create_vertices_with_texture() {
        let mut vertices = Vertices::empty();
        let pos = vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0];
        let color = vec![1.0, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.5];
        let texture = vec![1.0, 0.0, 1.0, 0.0, 0.0, 0.0];
        let expected = vec![
            -0.9, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0,
            0.0, -0.5, 0.0, 0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 0.0,
        ];
        vertices.set_position(&pos, VertexLocation::Position);
        vertices.set_position(&color, VertexLocation::Color);
        vertices.set_position(&texture, VertexLocation::Texture);
        let result = vertices.create_single_vertices_array();
        assert!(result.is_some());
        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_create_vertices_with_texture_invalid_format() {
        let mut vertices = Vertices::empty();
        let pos = vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0];
        let color = vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];
        let texture = vec![1.0, 0.0, 1.0, 0.0];
        vertices.set_position(&pos, VertexLocation::Position);
        vertices.set_position(&color, VertexLocation::Color);
        vertices.set_position(&texture, VertexLocation::Texture);
        let result = vertices.create_single_vertices_array();
        assert!(result.is_none());
    }

    #[test]
    fn test_get_stride_only_pos() {
        let mut vertices = Vertices::empty();
        let pos = vec![-0.9, 0.0, 0.0, -0.5, 0.5, 0.0, -0.5, 0.0, 0.0];
        vertices.set_position(&pos, VertexLocation::Position);
        let result = vertices.get_stride();
        assert_eq!(12, result);
    }

    #[test]
    fn test_append_colors() {
        let mut vertices = Vertices::empty();
        let colors = [RGBA::new(255, 0, 255, 1.0), RGBA::new(0, 255, 0, 1.0)];
        vertices.append_colors(&colors);
        assert_eq!(8, vertices.sum_capacity());
        assert_eq!(1_f32.to_bits(), vertices.colors_pos[0].to_bits());
        assert_eq!(1_f32.to_bits(), vertices.colors_pos[5].to_bits());

        let second_color = [RGBA::new(255, 0, 255, 1.0)];
        vertices.append_colors(&second_color);
        assert_eq!(12, vertices.sum_capacity());
        assert_eq!(1_f32.to_bits(), vertices.colors_pos[10].to_bits());
    }
}
