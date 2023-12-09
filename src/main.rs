use std::{fs::File, io::BufWriter};

use raytracer::{tuple::Tuple, matrix::Matrix};

use crate::raytracer::{canvas::Canvas, color::Color, exporter::{PPMExporter, Exporter}};

mod raytracer;

fn main() {
    let mut points: Vec<Tuple> = vec![];
    let ref_point = Tuple::point(0.0, 0.0, 1.0);

    for i in 0..12 {
        let p = &Matrix::identity(4, 4)
            .rotate_y(f64::from(i) * std::f64::consts::PI/6.0) * &ref_point;

        points.push(p);
    }

    let mut canvas = Canvas::new(200, 200);
    let half_canvas = canvas.width() as f64 / 2.0;
    let margin = 20.0;
    for p in points {
        let x = (half_canvas + p.x() * (half_canvas - margin)).round() as usize;
        let y = (half_canvas + p.z() * (half_canvas - margin)).round() as usize;
        canvas.write_pixel(x, y, Color::new(255.0, 255.0, 255.0));
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

struct Projectile {
    position: Tuple,
    velocity: Tuple
}

struct Environment {
    gravity: Tuple,
    wind: Tuple
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + &proj.velocity;
    let velocity = proj.velocity + &env.gravity + &env.wind;
    Projectile { position, velocity }
}
