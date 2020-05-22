// A (hopefully simple) vector class
#![allow(dead_code, unused_imports)]

use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg, Index};
pub type Float = f32;
pub const PI: Float = 3.14159;

/*************************** VECTOR ***************************/


// Vector construct
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

/* 
Reflection: for vector or ray??
*/

/* Colour stuff, maybe another file */

/* Random vectors etc?? */

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {x: 0., y: 0., z: 0.};
    pub const X: Vec3 = Vec3 {x: 1., y: 0., z: 0.};
    pub const Y: Vec3 = Vec3 {x: 0., y: 1., z: 0.};
    pub const Z: Vec3 = Vec3 {x: 0., y: 0., z: 1.};

    pub fn new(x: Float, y: Float, z: Float) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn norm_squared(self) -> Float {
        dot(self, self)
    }
    
    pub fn norm(self) -> Float {
        dot(self, self).sqrt()
    }

    pub fn normalise(self) -> Vec3 {
        self/self.norm()
    }

}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {x: -self.x,
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
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

/******************************** RAY *******************************/

// Ray construct
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    // Time, other clever factors etc
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray {origin: origin, direction: direction}
    }
}