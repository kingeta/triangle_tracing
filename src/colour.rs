/* Note colours are 'raw' and un-gamma-encoded; they are only exported a single way */
use std::ops::Mul
use image::{Rgb, Pixel};
use super::vector::*;

const GAMMA: Float = 2.2;

pub type Colour = Vec3;

/*pub fn clamp(val: Float) -> Float {
    val.max(0.).min(1.)
}*/

fn exp(x: Float) -> Float {
    1.- (-x).exp() //Brightnes??
}

fn gamma_encode(linear: Float) -> Float {
    linear.powf(1/GAMMA)
}

impl Mul<Colour> for Colour {
    type Output = Colour;
    fn mul(self, other: Colour) -> Colour {
        Colour {
            x: self.x*other.x,
            y: self.y*other.y,
            z: self.z*other.z}
    }
}

impl Colour {
    pub const BLACK = Colour::new(0., 0., 0.);
    pub const WHITE = Colour::new(1., 1., 1.);

    /*pub fn clamp(self) -> Colour {
        Colour {x: clamp(x), y: clamp(y), z: clamp(z)}
    }*/

    pub fn to_rgb(self) -> Rgb<u8> {
        Rgb::from_channels(
            gamma_encode(exp(self.x)) * 255. as u8,
            gamma_encode(exp(self.y)) * 255. as u8,
            gamma_encode(exp(self.z)) * 255. as u8,
            0,
        )
    }

}