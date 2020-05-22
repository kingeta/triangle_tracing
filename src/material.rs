/* Materials */
use super::vector::*;
use super::colour::*;

/*pub trait BxDF {
    pub fn f(in: Vec3, out: Vec3) -> Float;
    pub fn sample_f(in: Vec3, out: Vec3);
}*/

// Do I need an enum which considers whether e.g. this is a mirror and only needs one sample / this is lambert and needs many?

pub fn random_float(seed: &mut u32) -> Float {
    // Random float in [0, 1)
    let mut x = *seed;
    x ^= x >> 13;
    x ^= x << 17;
    x ^= x >> 5;
    *seed = x;
    let float_bits = (x & 0x007FFFFF) | 0x3F800000;
    let float: Float = unsafe { ::core::mem::transmute(float_bits) };
    return float - 1.0;
}

// Random vector on unit hemisphere
pub fn random_vector(seed: &mut u32) -> Vec3 {
    let u1 = random_float(seed);
    let u2 = random_float(seed);

    let r = (1.-u1*u1).sqrt();
    let phi = 2*PI*u2;

    Vec3 {x: phi.cos()*r, y: phi.sin()*r, z: u1}
}

pub trait Material {
    pub fn bsdf(self, in: Vec3, normal: Vec3, seed: &mut u32) -> Option<Vec3>; // Something about samples returned, etc etc; samples in?
    pub fn colour(self, position: Vec3) -> Colour; // -> diffuse colour if not light else emittance
}

// Lambertian diffuse material
pub struct Lambert {

    pub col: Colour;

}

impl Material for Lambert {
    pub fn bsdf(self, in: Vec3, normal: Vec3, seed: &mut u32) -> Option<Vec3> {
        let random: Vec3 = random_vector(seed);

        dot(random, normal).sign() * random
    }

    pub fn colour(self, _: Vec3) -> Colour {
        self.col
    }
}