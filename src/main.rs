use std::{fs::File, io::BufWriter};

use raytracer::{objects::{intersection::Intersection, lights::PointLight, materials::Material, object::Object, sphere::Sphere}, ray::Ray, transformation, tuple::Tuple};

use crate::raytracer::{canvas::Canvas, color::Color, exporter::{PPMExporter, Exporter}};

mod raytracer;

fn main() {

    use std::time::Instant;
    let now = Instant::now();

    let ray_origin = Tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_pixel = 200;
    let pixel_size = wall_size / canvas_pixel as f64;
    let half = wall_size / 2.0;

    let mut canvas = Canvas::new(canvas_pixel, canvas_pixel);

    let mut material = Material::default();
    material.set_color(Color::new(1.0, 0.2, 1.0));

    let mut s = Sphere::default();
    s.set_material(material);

    let light_position = Tuple::point(-10.0, 10.0, -10.0);
    let light_color = Color::new(1.0, 1.0, 1.0);
    let light = PointLight::new(light_position, light_color);

    for y in 0..canvas.height() {
        let world_y = half - pixel_size * y as f64;

        for x in 0..canvas.width() {
            let world_x = -half + pixel_size * x as f64;

            let position = Tuple::point(world_x, world_y, wall_z);

            let r = Ray::new(ray_origin.clone(), (position - &ray_origin).normalize());
            let xs = s.intersects(&r);
            
            let hit = Intersection::from_hit(&xs);

            if hit.is_some() {
                let point = r.position(hit.unwrap().t());
                let normal = hit.unwrap().object().normal_at(&point);
                let eye = -r.direction();
                let color = hit.unwrap().object().material().lighting(&light, &point, &eye, &normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }

    let elapsed = now.elapsed();
    println!("Rendering took: {:.2?}", elapsed);

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
