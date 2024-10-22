use cyclone_rs::{Vector3, Particle};
use std::time::Duration;

fn main() {
    let mut particle = Particle::default();
    particle.set_position(&Vector3::new(0.0, 0.0, 0.0));
    particle.set_velocity(&Vector3::new(1.0, 1.0, 1.0));
    
    let delta_t = Duration::from_millis(100);

    for _ in 0..10000 {
        particle.euler_integrate(delta_t);
        // println!("particle: {:?}", particle);
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
