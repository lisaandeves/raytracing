use std::f32::consts::PI;

use glam::{Vec3, Vec3A};
use rand::{Rng, rngs::SmallRng};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Diffuse {}

impl Diffuse {
    pub fn sample_bsdf(&self, rng: &mut SmallRng) -> Vec3A {
        let (i, j): (f32, f32) = rng.random();
        let angle = 2.0 * PI * i;
        let radius = j.sqrt();
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let z = (1.0 - (x * x + y * y)).sqrt();
        let incoming: Vec3A = Vec3 { x, y, z }.into();
        incoming.normalize()
    }
}
