// now this is a good vector class thing
use std::ops::{Add, Sub, Mul, Div, AddAssign, Neg, Index};

pub type Float = f32;

trait Dot<T: Copy + Add<Output = T> + Mul<Output = T>> {
    fn dot_vector(self, other: Vector3::<T>) -> T;
    fn dot_normal(self, other: Normal3::<T>) -> T;
}

// A vector class for directions
// Clone and Copy?
#[derive(Debug, Clone, Copy)]
pub struct Vector3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vector3<Float>;
pub const ZERO: Vec3f = Vec3f {x: 0., y: 0., z: 0.};
pub const X: Vec3f = Vec3f {x: 1., y: 0., z: 0.};
pub const Y: Vec3f = Vec3f {x: 0., y: 1., z: 0.};
pub const Z: Vec3f = Vec3f {x: 0., y: 0., z: 1.};

impl Vec3f {
    pub fn scalar(x: Float) -> Self {
        debug_assert!(!x.is_nan());
        Vec3f {x: x, y: x, z: x}
    }

    pub fn new(x: Float, y: Float, z: Float) -> Self {
        debug_assert!(!x.is_nan() & !y.is_nan() & !z.is_nan());
        Vec3f {x: x, y: y, z: z}
    }

    pub fn magnitude_sqr(self) -> Float {
        dot(self, self)
    }

    pub fn magnitude(self) -> Float {
        self.magnitude_sqr().sqrt()
    }

    pub fn normalise(self) -> Vec3f {
        self/self.magnitude()
    }
}

impl<T> Neg for Vector3<T> where T: Copy + Neg<Output = T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Add for Vector3<T> where T: Copy + Add<Output = T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector3<T> where T: Copy + Sub<Output = T>{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl<T> Mul<T> for Vector3<T> where T: Copy + Mul<Output = T> {
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl<T> Div<T> for Vector3<T> where T: Copy + Div<Output = T> {
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl<T> Index<usize> for Vector3<T> where T: Copy {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        if idx > 1 {
            &self.z
        } else if idx == 1 {
            &self.y
        } else {
            &self.x
        }
    }
}

impl Dot<Vector3::<T>> for Vector3::<T> where T: Copy + Add<Output = T> + Mul<Output = T> {
    fn dot_vector<T>(u: Vector3<T>, v: Vector3<T>) -> T {
        u.x*v.x + u.y*v.y + u.z*v.z
    }    
}

fn dot<T>(u: Vector3<T>, v: Vector3<T>) -> T where T: Copy + Add<Output = T> + Mul<Output = T> {
    u.x*v.x + u.y*v.y + u.z*v.z
}

// A bit too difficult to figure out how to do this with generics
fn dot_abs(u: Vec3f, v: Vec3f) -> Float {
    dot(u, v).abs()
}

fn cross<T>(u: Vector3<T>, v: Vector3<T>) -> Vector3<T> where T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> {
    Vector3::<T> {
        x: u.y*v.z - v.y*u.z,
        y: v.x*u.z - u.x*v.z,
        z: u.x*v.y - v.x*u.y,
    }
}


// Point class for positions
#[derive(Debug, Clone, Copy)]
pub struct Point3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    // maybe w as well?
}

pub type Point3f = Point3<Float>;
pub const ZERO_POINT: Point3f = Point3f {x: 0., y: 0., z: 0.};


impl<T: Copy + Add> Point3<T> {
    pub fn scalar(a: T) -> Point3<T> {
        Point3::<T> {x: a, y: a, z: a}
    }

    pub fn new(x: T, y: T, z: T) -> Point3<T> {
        Point3::<T> {x: x, y: y, z: z}
    }
}


impl<T> Add<Vector3<T>> for Point3<T> where T: Copy + Add<Output = T> {
    type Output = Self;

    fn add(self, other: Vector3::<T>) -> Point3::<T> {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub<Vector3<T>> for Point3<T> where T: Copy + Sub<Output = T> {
    type Output = Self;

    fn sub(self, other: Vector3::<T>) -> Point3::<T> {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl<T> Sub for Point3<T> where T: Copy + Sub<Output = T> {
    type Output = Vector3::<T>;

    fn sub(self, other: Self) -> Vector3::<T> {
        Vector3::<T> {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


// Point class for positions
#[derive(Debug, Clone, Copy)]
pub struct Normal3<T: Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Normal3f = Normal3<Float>;

impl<T: Copy + Add> Normal3<T> {
    /*pub fn scalar(a: T) -> Normal3<T> {
        Point3::<T> {x: a, y: a, z: a}
    }*/

    pub fn new(x: T, y: T, z: T) -> Self {
        Normal3::<T> {x: x, y: y, z: z}
    }
}



pub struct Ray {
    pub o: Point3f,
    pub r: Vec3f,
    pub t_max: Float,
    pub time: Float,
}