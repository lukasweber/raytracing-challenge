use std::io::Write;

use super::canvas;

pub trait Exporter {
    fn export(&self, canvas: &canvas::Canvas, writer: &mut dyn Write) -> std::io::Result<()>;
}

pub struct PPMExporter {}

const MAX_PPM_LINE_LENGTH: usize = 70;

impl PPMExporter {
    pub fn new() -> Self {
        Self {}
    }
}

impl Exporter for PPMExporter {
    fn export(&self, canvas: &canvas::Canvas, writer: &mut dyn Write) -> std::io::Result<()> {
        let mut buf = itoa::Buffer::new();
        
        writer.write_all(format!("P3\n{} {}\n255\n", canvas.width(), canvas.height()).as_bytes())?;

        let mut current_line_length = 0;
        let pixels = canvas.pixels();
        for el in pixels.iter() {
            let colors: [u8; 3] = [get_out_val(el.red()), get_out_val(el.green()), get_out_val(el.blue())];
            for color in colors.iter() {
                writer.write_all(buf.format(*color).as_bytes())?;
                if (current_line_length + 1) % (canvas.width() * 3) == 0 || current_line_length + 1 >= MAX_PPM_LINE_LENGTH {
                    writer.write_all(b"\n")?;
                    current_line_length = 0;
                } else {
                    writer.write_all(b" ")?;
                    current_line_length += 1;
                }
            }
        }
        Ok(())
    }
}

fn get_out_val(px: f64) -> u8 {
    match px {
        px if px <= 0.0 => 0,
        px if px >= 1.0 => 255,
        px => (px * 255.0).round() as u8,
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::color::Color;

    use super::*;
    use std::io::{BufWriter, BufReader, BufRead};

    #[test]
    fn ppm_export_creates_header() {
        // Given
        let exporter = PPMExporter::new();
        let mut buffer = BufWriter::new(Vec::new());
        let canvas = canvas::Canvas::new(5, 3);
        
        // When
        exporter.export(&canvas, &mut buffer).unwrap();

        // Then
        buffer.flush().unwrap();
        let reader = BufReader::new(buffer.get_ref().as_slice());
        let mut lines = reader.lines();
        assert_eq!(lines.next().unwrap().unwrap(), "P3");
        assert_eq!(lines.next().unwrap().unwrap(), "5 3");
        assert_eq!(lines.next().unwrap().unwrap(), "255");
    }

    #[test]
    fn ppm_export_creates_data() {
        // Given
        let exporter = PPMExporter::new();
        let mut buffer = BufWriter::new(Vec::new());
        let mut canvas = canvas::Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        
        // When
        canvas.write_pixel(0, 0, c1);
        canvas.write_pixel(2, 1, c2);
        canvas.write_pixel(4, 2, c3);
        exporter.export(&canvas, &mut buffer).unwrap();

        // Then
        buffer.flush().unwrap();
        let reader = BufReader::new(buffer.get_ref().as_slice());
        let mut lines = reader.lines();
        // skip header
        for _i in 0..3 {
            lines.next().unwrap().unwrap();
        }
        assert_eq!(lines.next().unwrap().unwrap(), "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines.next().unwrap().unwrap(), "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines.next().unwrap().unwrap(), "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn ppm_export_splits_after_max_length() {
        // Given
        let exporter = PPMExporter::new();
        let mut buffer = BufWriter::new(Vec::new());
        let canvas = canvas::Canvas::new(MAX_PPM_LINE_LENGTH / 3 + 1, 1);

        // When
        exporter.export(&canvas, &mut buffer).unwrap();

        // Then
        buffer.flush().unwrap();
        let reader = BufReader::new(buffer.get_ref().as_slice());
        let mut lines = reader.lines();
        // skip header
        for _i in 0..3 {
            lines.next().unwrap().unwrap();
        }

        // there should be just zeroes with a space like "0 " except the last character
        let mut line = lines.next().unwrap().unwrap();
        assert_eq!(line.len(), MAX_PPM_LINE_LENGTH * 2 - 1);

        // the last line should contain the overflow
        line = lines.next().unwrap().unwrap();
        assert_eq!(line.len(), 4);
    }

    #[test]
    fn ppm_export_terminates_with_newline() {
        // Given
        let exporter = PPMExporter::new();
        let mut buffer = BufWriter::new(Vec::new());
        let canvas = canvas::Canvas::new(3, 2);

        // When
        exporter.export(&canvas, &mut buffer).unwrap();

        // Then
        buffer.flush().unwrap();
        let chars = buffer.get_ref().as_slice();
        assert!(chars.iter().last().unwrap().eq(&b'\n'))
    }
}
