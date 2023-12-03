use raytracer::tuple::Tuple;

mod raytracer;

fn main() {
    let mut p = Projectile { position: Tuple::point(0.0, 1.0, 0.0), velocity: Tuple::vector(1.0, 1.0, 0.0).normalize() };
    let e = Environment { gravity: Tuple::vector(0.0, -0.1, 0.0), wind: Tuple::vector(-0.01, 0.0, 0.0) };
    println!("{}", p.position);
    while p.position.y() > 0.0 {
        p = tick(&e, p);
        println!("{}", p.position);
    }
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
