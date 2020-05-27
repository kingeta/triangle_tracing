// A (hopefully simple) vector class
#![allow(dead_code, unused_imports)]

use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg, Index};
pub type Float = f32;
pub const PI: Float = 3.14159;
pub const EPS: Float = 1e-5;

/// Wrapper for random float function
pub fn random_float() -> Float {
    rand::random::<Float>()
}

/// Random vector on unit hemisphere
pub fn random_vector() -> Vec3 {
    let u1 = random_float();
    let u2 = random_float();

    let r = (1.-u1*u1).sqrt();
    let phi = 2.*PI*u2;

    Vec3 {x: phi.cos()*r, y: phi.sin()*r, z: u1}
}

/// Random vector in the hemisphere defined by u
pub fn random_hemisphere(u: Vec3) -> Vec3 {
    let v = random_vector();
    dot(u, v).signum() * v
}

/// Cosine weighted sampling on the unit hemisphere upwards
pub fn random_cosine() -> Vec3 {
    let u1 = random_float();
    let theta = 2. * PI * random_float();

    let r = u1.sqrt();

    Vec3 {
        x: r * theta.cos(),
        y: r * theta.sin(),
        z: (1.-u1).max(0.).sqrt(),
    }
}

/// Generate, from one vector, an orthonormal basis
/// without too many divisions by zero etc.
/// Taken from Duff etc paper
pub fn onb(u: Vec3) -> (Vec3, Vec3) {
    //let sign = (1_f32).copysign(u.z);
    let sign = u.z.signum();
    let a = -1./ (sign + u.z);
    let b = u.x * u.y * a;

    (Vec3::new(1. + sign * u.x * u.x * a, sign * b, -sign * u.x), Vec3::new(b, sign + u.y * u.y * a, -u.y))
}

/// Cosine weighted sampling on the hemisphere defined by u.
pub fn random_hemisphere_cosine(u: Vec3) -> Vec3 {
    let (v, w) = onb(u);
    let r = random_cosine();
    
    // The cosine weighted random vector has positive z component;
    // we want the transformed ray to be in the u direction, so
    // r.z multiplies u. I believe this result should be of norm 1
    // (so long as ONB is)
    r.z * u + r.x * v + r.y * w
}


/*************************** VECTOR ***************************/


/// Vector construct
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,    
}

pub fn dot(u: Vec3, v: Vec3) -> Float {
    u.x*v.x + u.y*v.y + u.z*v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: u.y*v.z - u.z*v.y,
        y: u.z*v.x - u.x*v.z,
        z: u.x*v.y - u.y*v.x,
    }
}

pub fn reflect(u: Vec3, n: Vec3) -> Vec3 {
    u - 2. * dot(u, n) * n
}


impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {x: 0., y: 0., z: 0.};
    pub const X: Vec3 = Vec3 {x: 1., y: 0., z: 0.};
    pub const Y: Vec3 = Vec3 {x: 0., y: 1., z: 0.};
    pub const Z: Vec3 = Vec3 {x: 0., y: 0., z: 1.};

    pub const fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn norm_squared(self) -> Float {
        dot(self, self)
    }
    
    pub fn norm(self) -> Float {
        dot(self, self).sqrt()
    }

    pub fn normalise(self) -> Vec3 {
        if self.norm() < EPS {
            println!("There's been a problem")
        }
        self/self.norm()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z}
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z}
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z}
    }
}

impl Mul<Vec3> for Float {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,            
        }
    }
}

// No multiplication the other way to be proper


// To not be proper: division of vectors

impl Div<Float> for Vec3 {
    type Output = Vec3;
    fn div(self, other: Float) -> Vec3 {
        if other < EPS {
            println!("Uh oh")
        }
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

/******************************** RAY *******************************/

/// Ray construct
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    // Time, other clever factors etc
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin: origin, direction: direction}
    }

    pub fn eval(self, dist: Float) -> Vec3 {
        self.origin + dist * self.direction
    }
}