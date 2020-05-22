use image::{ImageBuffer, RgbImage, Rgb};
mod vector;
use vector::*;
mod shape;
use shape::*;

fn render(width: u32, height: u32, _samples: u32, filename: String) {
    // The final image
    let mut result: RgbImage = ImageBuffer::new(width, height);

    let origin: Vec3 = Vec3::ZERO;
    let mut direction: Vec3;
    let (hwidth, hheight) = (width as Float * 0.5, height as Float * 0.5);

    //let main: Sphere = Sphere{centre: Vec3 {x: 0., y: 0., z: 5.}, radius: 1., };
    let main: Triangle = Triangle::new(
        Vec3 {x: -1., y: 1., z: 10.},
        Vec3 {x: 1., y: -1., z: 10.},
        Vec3 {x: -1., y: -1., z: 10.},
    );

    //println!("{}, {}, {}", main.normal.x, main.normal.y, main.normal.z);


    for (x, y, pixel) in result.enumerate_pixels_mut() {
        direction = Vec3 {x: hwidth - x as Float, y: hheight - y as Float, z: hwidth}.normalise();
        let hit = main.intersect(Ray {origin: origin, direction: direction});
        *pixel = Rgb::<u8>([0, match hit {
            Some(_) => 120, //vals.dist as u8
            None => 0,
        }, 0])
    }

    result.save(filename).unwrap();
    println!("Does this work")
}

fn main() {
    //let vec: vector::Vec3f = vector::Vec3f {x: 1., y: 0., z: 10.};
    //let test = vector::ZERO;
    
    render(640, 320, 10, "test.png".to_string());
    println!("hello world");
    //println!("{}, {}, {}", vec[0], vec[1], vec[2]);
}
