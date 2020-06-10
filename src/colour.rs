/* Note colours are 'raw' and un-gamma-encoded; they are only exported a single way */
use std::ops::Mul;
use image::Pixel;
use super::vector::*;

const GAMMA: Float = 2.2;

pub type Colour = Vec3;

/*pub fn clamp(val: Float) -> Float {
    val.max(0.).min(1.)
}*/

fn exp(x: Float) -> Float {
    1.- (-x * 2.).exp() //Brightnes??
}

// ACES Filmic Tone Mapping Curve from the internet
fn filmic(x: Float) -> Float {
    let (a, b, c, d, e) = (2.51, 0.03, 2.43, 0.59, 0.14);

    ((x*(a*x+b))/(x*(c*x+d)+e)).max(0.).min(1.)
}

fn gamma_encode(linear: Float) -> Float {
    linear.powf(1./GAMMA)
}

impl Mul<Colour> for Colour {
    type Output = Colour;
    fn mul(self, other: Colour) -> Colour {
        Colour {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z}
    }
}

impl Colour {
    pub const BLACK: Colour = Colour {x: 0., y: 0., z: 0.};
    pub const WHITE: Colour = Colour {x: 1., y: 1., z: 1.};
    pub const WARN: Colour = Colour {x: 1., y: 0., z: 0.};

    /*pub fn clamp(self) -> Colour {
        Colour {x: clamp(x), y: clamp(y), z: clamp(z)}
    }*/

    pub fn to_image_rgb(self) -> image::Rgb<u8> {
        image::Rgb::from_channels(
            (gamma_encode(exp(self.x)).max(0.).min(1.) * 255.) as u8,
            (gamma_encode(exp(self.y)).max(0.).min(1.) * 255.) as u8,
            (gamma_encode(exp(self.z)).max(0.).min(1.) * 255.) as u8,
            0,
        )
    }

    pub fn to_u32_rgb_filmic(self) -> u32 {
        let r = (gamma_encode(filmic(self.x)) * 255.) as u32;
        let g = (gamma_encode(filmic(self.y)) * 255.) as u32;
        let b = (gamma_encode(filmic(self.z)) * 255.) as u32;
        
        (r << 16) | (g << 8) | b
    }

    pub fn to_u32_rgb(self) -> u32 {
        let r = (gamma_encode(exp(self.x)) * 255.) as u32;
        let g = (gamma_encode(exp(self.y)) * 255.) as u32;
        let b = (gamma_encode(exp(self.z)) * 255.) as u32;
        
        (r << 16) | (g << 8) | b
    }

}