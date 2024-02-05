use std::{fs::File, io::BufWriter};

use raytracer::{objects::{ray::Ray, sphere::Sphere}, transformation, tuple::Tuple};

use crate::raytracer::{canvas::Canvas, color::Color, exporter::{PPMExporter, Exporter}};

mod raytracer;

fn main() {
    let mut canvas = Canvas::new(200, 200);
    let half_canvas = canvas.width() as f64 / 2.0;

    let mut s = Sphere::default();
    let m = transformation::scaling(30.0, 30.0, 30.0)
        .translate(half_canvas, half_canvas, 0.0);

    s.set_transform(m);

    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            let ray = Ray::new(Tuple::point(x as f64, y as f64, 0.0), Tuple::vector(0.0, 0.0, 20.0));
            let intersects = s.intersects(&ray);
            if intersects.len() == 0 {
                canvas.write_pixel(x, y, Color::new(1.0, 1.0, 1.0));
            } else {
                canvas.write_pixel(x, y, Color::new(1.0, 0.0, 0.0));
            }
        }
    }

    export_ppm(&canvas);
}

fn export_ppm(c: &Canvas) {
    let exporter = PPMExporter::new();
    let file= File::create("out.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    use std::time::Instant;
    let now = Instant::now();
    
    exporter.export(&c, &mut writer).unwrap();

    let elapsed = now.elapsed();
    println!("Exporting took: {:.2?}", elapsed);
}
