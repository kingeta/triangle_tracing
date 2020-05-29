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
    Lambert(Float), // albedo included
    LambertCos(Float), // Uses cosine weighted sampling
    Mirror(Float), // Albedo here for some reason
    Glass(Float), // Glass with refractive index
    Light(Float), // Lights like this also absorb all incoming light
    LightUni(Float), // A light which only emits on the normal facing side
    LightCos(Float), // Cosine weighted light, which is also unidirectional
    Scatter(Float), // Henyey-Greenstein phase function thing
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

/// Henyey-Greenstein phase function, taking cos = cos(theta) (instead of theta)
/// -1 <= g <= 1
pub fn henyey_greenstein(cos: Float, g: Float) -> Float {
    0.5 * (1. - g*g)/(1. + g*g - 2.*g*cos).powf(1.5)
}