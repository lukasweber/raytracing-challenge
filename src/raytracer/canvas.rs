use super::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        Self { width, height, pixels: vec![Color::default(); width * height] }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, pixel: Color) {
        self.pixels[y * self.width + x] = pixel;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x].clone()
    }

    pub fn pixels(&self) -> &Vec<Color> {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_initializes_members() {
        // When
        let c = Canvas::new(10, 20);

        // Then
        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        assert_eq!(c.pixels.len(), 10 * 20);
        assert!(c.pixels.iter().all(|p| *p == Color::new(0.0, 0.0, 0.0)));
    }

    #[test]
    fn write_pixel_sets_pixel() {
        // Given
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        // When
        c.write_pixel(2, 3, red.clone());

        // Then
        assert_eq!(c.pixel_at(2, 3), red);
    }
}
