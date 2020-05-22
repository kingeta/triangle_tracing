// The actual shapes/primitives, not specific info e.g. colour or material
#![allow(dead_code, unused_imports)]


use super::vector::*;

pub struct Hit {
    pub dist: Float,
    pub point: Vec3, // Where the hit was
    pub norm: Vec3, //Normal at that point
}

pub trait Shape {
    fn intersect(&self, ray: Ray) -> Option<Hit>;
}

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
            a: a, b: b, c: c,
            normal: cross(b-a, c-a).normalise(),
        }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: Ray) -> Option<Hit> {
        let norm_dot_d = dot(ray.direction, self.normal);
        let normdot_o = dot(ray.origin, self.normal);



        if norm_dot_d < 0. {
            let t = (dot(self.a, self.normal)-normdot_o)/norm_dot_d;

            let point = ray.origin + t*ray.direction;

            if dot(self.normal, cross(point-self.a, self.b-self.a)) <= 0. && dot(self.normal, cross(point-self.b, self.c-self.b)) <= 0. && dot(self.normal, cross(point-self.c, self.a-self.c)) <= 0. {
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

        if d2 < 0. {
            return None;
        }
        let mut res = (-b - d2.sqrt())/(2.*a);
        if res < 0. {
            let res2 = (-b + d2.sqrt())/(2.*a);
            if res2 < 0. {
                return None;
            }
            res = res2
        }

        let hitpos = ray.origin + res*ray.direction;

        Some(Hit{dist: res, point: hitpos, norm: self.normal(hitpos)})
    }
}