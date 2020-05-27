/* Quite complex actually; define coordinate systems etc and ray generation */
#![allow(dead_code)]

use super::vector::*;

/// From L the direction a camera points and G the global up,
/// generate the orthonormal basis (L, S, U) where S points to
/// the side (???) and U locally points up; this returns S and U
fn directions(looking: Vec3, global_up: Vec3) -> (Vec3, Vec3) {
    let side = cross(global_up, looking).normalise();
    return (side, cross(looking, side))
}
/* THINK ABOUT THIS MORE IF NEEDED */

/// A Camera encapsulates ray generation (and usually holds basis vectors etc)
pub trait Camera {
    fn generate_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray;
    fn translate(&mut self, direction: Vec3);
}

/// A camera with no DOF etc
/// The FOV is horizontal
pub struct SimpleCamera {
    pub tan_half_fov: Float,
    pub position: Vec3,
    pub looking: Vec3,
    pub side: Vec3,
    pub up: Vec3,
}

impl SimpleCamera {
    pub fn new(fov: Float, position: Vec3, looking: Vec3, global_up: Vec3) -> SimpleCamera {
        let (side, up) = directions(looking, global_up);
        SimpleCamera {
            tan_half_fov: (fov / 2.).tan(),
            position: position,
            looking: looking,
            side: side,
            up: up,
        }
    }
}

impl Camera for SimpleCamera {
    fn generate_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        //let x_bar = 1. - (2*x) as Float/width as Float;
        //let y_bar = (height as Float - 2. * y as Float)/width as Float;

        //Ray {origin: self.position,
        //    direction: (self.looking + self.tan_half_fov * (x_bar * self.side + y_bar * self.up)).normalise()}
            
        let x_bar = width as Float - (2 * x) as Float;
        let y_bar = height as Float - (2 * y) as Float;

        Ray {origin: self.position,
            direction: (width as Float*self.looking + self.tan_half_fov*(x_bar*self.side + y_bar*self.up)).normalise()}
    
    }


    fn translate(&mut self, direction: Vec3) {
        self.position += direction.x * self.looking
            + direction.y * self.side
            + direction.z * self.up;
    }
 }