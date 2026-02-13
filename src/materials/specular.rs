use glam::Vec3A;
use rand::{Rng, rngs::SmallRng};

use crate::colour::Colour;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Specular {
    pub ior: f32,
}

impl Specular {
    pub fn sample_bsdf(&self, outgoing: Vec3A, rng: &mut SmallRng) -> Vec3A {
        let dot = outgoing.dot(Vec3A::Z);
        let ior = if dot > 0.0 { self.ior } else { 1.0 / self.ior };

        let reflect_prob = self.fresnel(outgoing);
        let p: f32 = rng.random();
        if p < reflect_prob {
            outgoing.reflect(Vec3A::Z)
        } else {
            outgoing.refract(Vec3A::Z, ior)
        }
    }

    pub fn bsdf(&self, incoming: Vec3A, outgoing: Vec3A, colour: Colour) -> Colour {
        let dot = outgoing.dot(Vec3A::Z);
        let ior = if dot > 0.0 { self.ior } else { 1.0 / self.ior };

        let reflect_prob = self.fresnel(outgoing);
        if incoming == outgoing.reflect(Vec3A::Z) {
            reflect_prob * colour / incoming.angle_between(Vec3A::Z).cos()
        } else if incoming == outgoing.refract(Vec3A::Z, ior) {
            (1.0 - reflect_prob) * colour / incoming.angle_between(Vec3A::Z).cos()
        } else {
            Colour::black()
        }
    }

    pub fn pdf(&self, incoming: Vec3A, outgoing: Vec3A) -> f32 {
        let dot = outgoing.dot(Vec3A::Z);
        let ior = if dot > 0.0 { self.ior } else { 1.0 / self.ior };

        let reflect_prob = self.fresnel(outgoing);
        if incoming == outgoing.reflect(Vec3A::Z) {
            reflect_prob
        } else if incoming == outgoing.refract(Vec3A::Z, ior) {
            1.0 - reflect_prob
        } else {
            0.0
        }
    }

    fn fresnel(&self, outgoing: Vec3A) -> f32 {
        let dot = outgoing.dot(Vec3A::Z);
        let ior = if dot > 0.0 { self.ior } else { 1.0 / self.ior };

        let transmitted = outgoing.refract(Vec3A::Z, ior);
        if transmitted == Vec3A::ZERO {
            return 1.0;
        }
        let cosangle_o = outgoing.angle_between(-Vec3A::Z).cos();
        let cosangle_t = transmitted.angle_between(-Vec3A::Z).cos();

        let parallel = (ior * cosangle_o - cosangle_t) / (ior * cosangle_o + cosangle_t);
        let perp = (cosangle_o - ior * cosangle_t) / (cosangle_o + ior * cosangle_t);

        let reflect_probability = 0.5 * (parallel * parallel + perp * perp);
        reflect_probability
    }
}
