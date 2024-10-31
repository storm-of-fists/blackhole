// /// The mass of the Earth, in kilograms.
// const MASS_OF_EARTH: f64 = 5.97219E24; // kg
// /// The fundamental gravitational constant.
// const GRAVITATIONAL_CONSTANT: f64 = 6.67430E-11; // Nm^2kg^-2

use std::{fmt::Debug, time::Duration};
// /// TODO(use std::simd)

#[derive(Default, Clone, Copy, Debug)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// #[derive(Default, Clone, Copy, Debug)]
// pub struct Basis3 {
//     x: Vector3,
//     y: Vector3,
//     z: Vector3,
// }



/// A particle that has vectors for different locations. Not sure this object
/// oriented is the right approach for lots of particles.
/// TODO(do we need some kind of "fixed" parameter in here?)
#[derive(Default, Debug)]
pub struct Particle {
    pub id: u64,
    pub position: Vector3,
    pub velocity: Vector3,
    pub acceleration: Vector3,
    /// We store an inverse mass since we use that most and it avoids divide by zero
    /// issues.
    pub inverse_mass: f64,
}

// pub struct Gravity {
//     gravity: Vector3,
//     rules: Vec<Box<dyn Fn(&Particle) -> bool>>,
// }

// // pub struct DampingForceGenerator {
// //     rules: Vec<Box<dyn Fn(&Particle) -> bool>>,
// // }

// /// TODO(make a bungee type where it allows any compression past some point)
// pub struct Spring {
//     particle_a_id: u64,
//     particle_b_id: u64,
//     spring_constant: f64,
//     damping: f64,
//     minimum_length: f64,
//     maximum_length: f64,
//     equilibrium_length: f64,
//     rules: Vec<Box<dyn Fn(&Particle) -> bool>>,
// }

// /// Special spring case where there is no force applied at some minimum length.
// /// This can be used for bouyancy as well.
// pub struct Bungee {
//     spring: Spring,
// }

// pub struct Rope {
//     spring: Spring,
// }

// pub struct Rod {
//     spring: Spring,
// }

// pub trait Constraint: {
//     fn apply_constraint(&self, particle: &mut Particle);
// }

// pub struct ConstraintsRegistry {
//     constraints: Vec<Box<dyn Constraint>>,
// }

/// TODO(need a orthornormal basis creator method?)
impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub const fn x(&self) -> &f64 {
        &self.x
    }

    #[inline]
    pub fn y(&self) -> &f64 {
        &self.y
    }

    #[inline]
    pub fn z(&self) -> &f64 {
        &self.z
    }

    #[inline]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    #[inline]
    pub fn set_z(&mut self, z: f64) {
        self.z = z;
    }

    pub fn magnitude(&self) -> f64 {
        self.square_magnitude().sqrt()
    }

    pub fn square_magnitude(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normal(&self) -> Self {
        let magnitude = self.magnitude();

        // Only divide once.
        self.scalar_multiply(&magnitude.recip())
    }

    #[inline]
    pub fn scalar_multiply(mut self, to_multiply: &f64) -> Self {
        self.x *= to_multiply;
        self.y *= to_multiply;
        self.z *= to_multiply;

        self
    }

    #[allow(clippy::should_implement_trait)]
    #[inline]
    pub fn add(mut self, add: &Vector3) -> Self {
        self.x += *add.x();
        self.y += *add.y();
        self.z += *add.z();

        self
    }

    // #[inline]
    // pub fn sub(mut self, sub: &Vector3) -> Self {
    //     self.x -= *sub.x();
    //     self.y -= *sub.y();
    //     self.z -= *sub.z();

    //     self
    // }

    // #[inline]
    // pub fn neg(mut self) -> Self {
    //     self.x = -self.x();
    //     self.y = -self.y();
    //     self.z = -self.z();

    //     self
    // }

    #[inline]
    pub fn component_multiply(mut self, multiply: &Vector3) -> Self {
        self.x *= multiply.x();
        self.y *= multiply.y();
        self.z *= multiply.z();

        self
    }

    #[inline]
    pub fn scalar_product(&self, multiply: &Vector3) -> f64 {
        self.x * multiply.x() + self.y * multiply.y() + self.z * multiply.z()
    }

    #[inline]
    /// https://www.geogebra.org/m/psMTGDgc
    pub fn vector_product_rh(mut self, multiply: &Vector3) -> Vector3 {
        self.x = self.y * multiply.z() - self.z * multiply.y();
        self.y = self.z * multiply.x() - self.x * multiply.z();
        self.z = self.x * multiply.y() - self.y * multiply.x();

        self
    }
}

impl Particle {
    pub fn set_position(&mut self, position: &Vector3) {
        self.position = *position;
    }

    pub fn set_velocity(&mut self, velocity: &Vector3) {
        self.velocity = *velocity;
    }

    pub fn set_acceleration(&mut self, acceleration: &Vector3) {
        self.acceleration = *acceleration;
    }

    pub fn euler_integrate(&mut self, duration: Duration) {
        if duration <= Duration::ZERO {
            return;
        }

        let duration_as_seconds_f64 = duration.as_secs_f64();

        self.position = self
            .position
            .add(&self.velocity.scalar_multiply(&duration_as_seconds_f64));

        self.velocity = self
            .velocity
            .add(&self.acceleration.scalar_multiply(&duration_as_seconds_f64));

        self.acceleration = Vector3::new(0.0, 0.0, 0.0);
    }

    pub fn accumulate_force(&mut self, force: &Vector3) {
        self.acceleration.add(&force.scalar_multiply(&self.inverse_mass));
    }
}