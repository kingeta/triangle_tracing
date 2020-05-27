/* Materials */
#![allow(dead_code, unused_imports)]
use super::vector::*;
//use super::colour::*;



/// Holds the material types I guess; this is
/// perhaps a hack. I'd prefer much more general
/// material types by either storing a function pointer
/// or having materials as traits, but it seems
/// it won't be so.
#[derive(Copy, Clone)]
pub enum Material {
    Lambert, // No albedo included
    LambertCos, // Uses cosine weighted sampling
    Mirror,
    Glass(Float), // Glass with refractive index
    Light(Float), // Lights like this also absorb all light
    Test,
}

/// Schlick approximation or something
pub fn schlick(cos: Float, n_dielectric: Float) -> Float {
    let r0 = ((1.-n_dielectric)/(1.+n_dielectric)).powf(2.);
    r0 + (1. - r0) * (1. - cos).powf(5.)
}

/// Refract a ray into a material with a given refractive index;
/// TIR if the ray is too sharp
pub fn refract(v: Vec3, n: Vec3, refr: Float) -> Option<Vec3> {
    let dt = dot(v, n);
    let discriminant = 1. - refr*refr * (1.- dt*dt);

    if discriminant > 0. {
        // Refract
        Some(refr * (v + dt.abs() * n) - discriminant.sqrt() * n)
    } else {
        // TIR => no refracted ray
        None
    }
}

/*********** Old Material definition ***********/


/*pub trait Material {
    fn bsdf(self, direction: Vec3, normal: Vec3) -> Option<Vec3>; // Something about samples returned, etc etc; samples in?
    //fn colour(self, position: Vec3) -> Colour; // -> diffuse colour if not light else emittance
}

/// Lambertian diffuse material
pub struct Lambert {
    pub col: Colour,
}

impl Material for Lambert {
    fn bsdf(self, _: Vec3, normal: Vec3) -> Option<Vec3> {
        let random: Vec3 = random_vector();

        Some(dot(random, normal).signum() * random)
    }
}*/