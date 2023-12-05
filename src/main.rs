use std::{fs::File, io::BufWriter};

use raytracer::tuple::Tuple;

use crate::raytracer::{canvas::Canvas, color::Color, exporter::{PPMExporter, Exporter}};

mod raytracer;

fn main() {
    let mut p = Projectile { position: Tuple::point(0.0, 1.0, 0.0), velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25 };
    let e = Environment { gravity: Tuple::vector(0.0, -0.1, 0.0), wind: Tuple::vector(-0.01, 0.0, 0.0) };
    let mut c = Canvas::new(900, 550);

    println!("{}", p.position);
    while p.position.y() > 0.0 {
        p = tick(&e, p);
        let target_x = p.position.x().round() as usize;
        let target_y = c.height() - (p.position.y().round() as usize);
        if target_y < c.height() && target_x < c.width() {
            c.write_pixel(target_x, target_y, Color::new(255.0, 0.0, 0.0));
        }
        println!("{}", p.position);
    }
    let exporter = PPMExporter::new();
    let file= File::create("out.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    use std::time::Instant;
    let now = Instant::now();
    
    exporter.export(&c, &mut writer).unwrap();

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
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
