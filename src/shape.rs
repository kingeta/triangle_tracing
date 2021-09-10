//! The actual shapes/primitives, not specific info e.g. colour or material
#![allow(dead_code, unused_imports)]
use super::vector::*;

#[derive(Copy, Clone)]
pub struct Hit {
    pub dist: Float,
    pub point: Vec3, // Where the hit was
    pub norm: Vec3, //Normal at that point
}

/// A primitive, all it requires is an intersection function
pub trait Shape {
    //fn intersect(&self, ray: Ray) -> Option<Hit>;
    fn intersect(&self, ray: Ray) -> Option<Hit>; // The point and normal
}

/// A triangle; the normal is precomputed (although perhaps the sides should be as well).
/// Possibly it should contain a switch to change the normal orientation?
/// The question of whether shapes should intersect on both sides is an interesting one
pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    pub normal: Vec3,
    // In/out/invert normal?
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        Triangle {
            a, b, c,
            normal: cross(b-a, c-a).normalise(),
        }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let norm_dot_d = dot(ray.direction, self.normal);
        let norm_dot_o = dot(ray.origin, self.normal);

        if norm_dot_d.abs() > EPS {
            let t = (dot(self.a, self.normal)-norm_dot_o)/norm_dot_d;

            //let point = ray.origin + t*ray.direction;
            let point = ray.eval(t);

            if t > EPS
                && dot(self.normal, cross(point-self.a, self.b-self.a)) <= 10. * EPS
                && dot(self.normal, cross(point-self.b, self.c-self.b)) <= 10. * EPS
                && dot(self.normal, cross(point-self.c, self.a-self.c)) <= 10. * EPS {
                Some(Hit {
                    dist: t,
                    point: point,
                    norm: self.normal})
            } else {
                None
            }

        } else {
            // Ray from wrong direction
            None
        }
    }
}

/// A simple sphere
pub struct Sphere {
    pub centre: Vec3,
    pub radius: Float,
}

impl Sphere {
    fn normal(&self, point: Vec3) -> Vec3 {
        (point - self.centre)/self.radius
    }
}

impl Shape for Sphere {
    // Copied off the internet a long time ago
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let a = dot(ray.direction, ray.direction);
        let to = ray.origin - self.centre;
        let b = 2. * dot(ray.direction, to);
        let c = dot(to, to) - self.radius*self.radius;

        let d2 = b*b-4.*a*c;

        if d2 < EPS { return None; }

        let mut res = (-b - d2.sqrt())/(2.*a);
        if res < EPS {
            let res2 = (-b + d2.sqrt())/(2.*a);
            if res2 < EPS { // EPS
                return None;
            }
            res = res2
        }

        let hitpos = ray.eval(res);

        Some(Hit{dist: res, point: hitpos, norm: self.normal(hitpos)})
    }
}


/// An infinite plane, defined by a normal and "size" number,
/// although it should be created probably with a point.
/// Intersects on both sides
pub struct Plane {
    pub normal: Vec3,
    pub size: Float
}

impl Plane {
    /// Generate a plane from the normal and a point on it
    pub fn new(normal: Vec3, point: Vec3) -> Plane {
        Plane {normal: normal, size: dot(normal, point)}
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let t = dot(ray.direction, self.normal);

        if t.abs() > 0. {
            let dist = (self.size - dot(self.normal, ray.origin))/t;
            if dist > 0. {
                Some(Hit{dist: dist, point: ray.eval(dist), norm: self.normal})
            } else {
                // Plane behind camera
                None
            }
        } else {
            // Ray parallel
            None
        }
    }
}