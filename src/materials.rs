pub mod diffuse;
pub mod metal;
pub mod specular;

use glam::Vec3A;
use rand::rngs::SmallRng;
use std::f32::consts::PI;

use crate::{
    colour::Colour,
    materials::{diffuse::Diffuse, metal::Metal, specular::Specular},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialType {
    Diffuse(Diffuse),
    Reflective,
    Emissive,
    Specular(Specular),
    Metal(Metal),
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub kind: MaterialType,
    pub colour: Colour,
}

impl Material {
    pub fn sample_bsdf(&self, outgoing: Vec3A, rng: &mut SmallRng) -> Vec3A {
        match self.kind {
            MaterialType::Diffuse(mat) => mat.sample_bsdf(rng),
            MaterialType::Reflective => outgoing.reflect(Vec3A::Z),
            MaterialType::Emissive => Vec3A::ZERO,
            MaterialType::Specular(mat) => mat.sample_bsdf(outgoing, rng),
            MaterialType::Metal(mat) => mat.sample_bsdf(outgoing, rng),
        }
    }

    pub fn bsdf(&self, incoming: Vec3A, outgoing: Vec3A) -> Colour {
        match self.kind {
            MaterialType::Diffuse(_) => self.colour / PI,
            MaterialType::Reflective => self.colour / incoming.angle_between(Vec3A::Z).cos(),
            MaterialType::Emissive => Colour::black(),
            MaterialType::Specular(mat) => mat.bsdf(incoming, outgoing, self.colour),
            MaterialType::Metal(mat) => mat.bsdf(incoming, outgoing, self.colour),
        }
    }

    pub fn pdf(&self, incoming: Vec3A, outgoing: Vec3A) -> f32 {
        match self.kind {
            MaterialType::Diffuse(_) => incoming.angle_between(Vec3A::Z).cos() / PI,
            MaterialType::Reflective => 1.0,
            MaterialType::Emissive => 0.0,
            MaterialType::Specular(mat) => mat.pdf(incoming, outgoing),
            MaterialType::Metal(mat) => mat.pdf(incoming, outgoing),
        }
    }
}
