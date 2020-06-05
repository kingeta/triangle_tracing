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



/// A camera with no DOF etc but AA
/// The FOV is horizontal
pub struct SimpleAACamera {
    pub tan_half_fov: Float,
    pub position: Vec3,
    pub looking: Vec3,
    pub side: Vec3,
    pub up: Vec3,
}

impl SimpleAACamera {
    pub fn new(fov: Float, position: Vec3, looking: Vec3, global_up: Vec3) -> SimpleAACamera {
        let (side, up) = directions(looking, global_up);
        SimpleAACamera {
            tan_half_fov: (fov / 2.).tan(),
            position: position,
            looking: looking,
            side: side,
            up: up,
        }
    }
}

impl Camera for SimpleAACamera {
    fn generate_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        //let x_bar = 1. - (2*x) as Float/width as Float;
        //let y_bar = (height as Float - 2. * y as Float)/width as Float;

        //Ray {origin: self.position,
        //    direction: (self.looking + self.tan_half_fov * (x_bar * self.side + y_bar * self.up)).normalise()}
            
        //let x_bar = width as Float - (2 * x) as Float;
        let x_bar = width as Float - 2. * (x as Float + random_float());

        //let y_bar = height as Float - (2 * y) as Float;
        let y_bar = height as Float - 2. * (y as Float + random_float());

        Ray {origin: self.position,
            direction: (width as Float*self.looking + self.tan_half_fov*(x_bar*self.side + y_bar*self.up)).normalise()}
    
    }


    fn translate(&mut self, direction: Vec3) {
        self.position += direction.x * self.looking
            + direction.y * self.side
            + direction.z * self.up;
    }
 }



/// A camera with DOF and AA
/// The FOV is horizontal
/// Aperture is also perfectly -circular- square (???)
/// Probably requires more thought
/// Focal plane is flat for some reason
pub struct DOFCamera {
    pub tan_half_fov: Float,
    pub position: Vec3,
    pub looking: Vec3,
    pub side: Vec3,
    pub up: Vec3,
    pub focal_distance: Float,
    pub aperture: Float, // Some generic measure of focal distance
}

impl DOFCamera {
    pub fn new(fov: Float, position: Vec3, looking: Vec3, global_up: Vec3, focal_distance: Float, aperture: Float) -> DOFCamera {
        let (side, up) = directions(looking, global_up);
        DOFCamera {
            tan_half_fov: (fov / 2.).tan(),
            position: position,
            looking: looking,
            side: side,
            up: up,
            focal_distance: focal_distance,
            aperture: aperture,
        }
    }
}

impl Camera for DOFCamera {
    fn generate_ray(&self, x: usize, y: usize, width: usize, height: usize) -> Ray {
        //let x_bar = 1. - (2*x) as Float/width as Float;
        //let y_bar = (height as Float - 2. * y as Float)/width as Float;

        //Ray {origin: self.position,
        //    direction: (self.looking + self.tan_half_fov * (x_bar * self.side + y_bar * self.up)).normalise()}
            
        let x_bar = width as Float - 2. * (x as Float + random_float());

        let y_bar = height as Float - 2. * (y as Float + random_float());

        let scaled_direction = self.focal_distance / width as Float * (width as Float*self.looking + self.tan_half_fov*(x_bar*self.side + y_bar*self.up));

        //let random_position = self.aperture * ((2. * random_float() - 1.) * self.side + (2. * random_float() - 1.) * self.up);

        let angle = random_float() * 2. * PI;
        let random_position = self.aperture * random_float().sqrt() * (angle.sin() * self.side + angle.cos() * self.up);

        Ray {origin: self.position + random_position,
            direction: (scaled_direction - random_position).normalise()}
    
    }


    fn translate(&mut self, direction: Vec3) {
        self.position += direction.x * self.looking
            + direction.y * self.side
            + direction.z * self.up;
    }
 }