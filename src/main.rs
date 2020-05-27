#![allow(dead_code, unused_imports)]
use std::path::Path;

use image::{ImageBuffer, RgbImage};
mod vector;
use vector::*;

mod colour;
use colour::Colour;

mod material;
use material::*;

mod shape;
use shape::{Triangle, Sphere, Plane};

mod object;
use object::*;

mod camera;
use camera::Camera;

use minifb::{Key, Window, WindowOptions};

use rayon::prelude::*;

fn render<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize, samples: usize, filename: String) {
    // The final image
    let mut result: RgbImage = ImageBuffer::new(width as u32, height as u32);


    /* Square made of triangles */

    /*let main_object = ObjectCollection::<Triangle> {
        shapes: vec![
            Triangle::new(
                Vec3 {x: -1., y: 1., z: 10.},
                Vec3 {x: 1., y: -1., z: 10.},
                Vec3 {x: -1., y: -1., z: 10.},        
            ),
            Triangle::new(
                Vec3 {x: -1., y: 1., z: 10.},
                Vec3 {x: 1., y: 1., z: 10.},
                Vec3 {x: 1., y: -1., z: 10.},        
            )
        ],
        material: Material::Test,
        colour: Colour::new(1., 0., 0.),
    };*/


    // Set up camera
    //let main_camera = camera::SimpleCamera::new(PI/2., camera_pos, -camera_pos.normalise(), Vec3::Y);
    //let cornell_camera_pos = Vec3::new(0., 0., -1.);
    //let main_camera = camera::SimpleCamera::new(PI/2., cornell_camera_pos, -cornell_camera_pos.normalise(), Vec3::Y);

    let mut col; // = Colour::BLACK;

    for (x, y, pixel) in result.enumerate_pixels_mut() {
        col = Colour::BLACK;
        
        for _ in 0..samples {
            col += trace(scene, camera.generate_ray(x as usize, y as usize, width, height), 4);
        }

        *pixel = (col / samples as Float).to_image_rgb();

        //*pixel = trace(&sphere, main_camera.generate_ray(x, y, width, height), 5).to_rgb();
    }

    result.save(filename).unwrap();
}


fn render_iter<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize, samples: usize) {
    let mut buffer: Vec<u32> = vec![0; width*height];

    /*
    for y in 0..height {
        for x in 0..width {
            pixels.push((x, y))
        }
    }
    */
    
    // The final image
    (0..width*height).for_each(|i| {
        let mut colour = Colour::BLACK;
        let x = i % width;
        let y = i / height;

        for _ in 0..samples {
            colour += trace(scene, camera.generate_ray(x, y, width, height), 5)
        }

        buffer[i] = (colour / samples as Float).to_u32_rgb();
    });

    let mut window = Window::new("Pathtracing", width, height, WindowOptions::default())
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}


fn render_window<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize) {

    let mut backbuffer = vec![Colour::ZERO; width * height];
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new("Pathtracing", width, height, WindowOptions::default())
                .unwrap();

    let mut samples = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // Rendering and pushing to the window
        samples += 1;
        for x in 0..width {
            for y in 0..height {
                backbuffer[x + y * width] += trace(scene, camera.generate_ray(x, y, width, height), 5);
                buffer[x + y * width] = (backbuffer[x + y * width]/samples as Float).to_u32_rgb()
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();

    }
}

fn render_window_iter<O: Object, C: Camera>(scene: &O, camera: &mut C, width: usize, height: usize) {
    let mut pixels: Vec<(usize, usize)> = vec![];

    for y in 0..height {
        for x in 0..width {
            pixels.push((x, y))
        }
    }

    let mut backbuffer = vec![Colour::ZERO; width * height];
    let mut buffer: Vec<u32>; // = vec![0; width * height];

    let mut window = Window::new("Pathtracing", width, height, WindowOptions::default())
                .unwrap();

    let mut samples = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        
        if window.is_key_down(Key::W) {
            camera.translate(0.1 * Vec3::X);
            samples = 0;
            backbuffer = vec![Colour::ZERO; width * height];
        }

        if window.is_key_down(Key::S) {
            camera.translate(-0.1 * Vec3::X);
            samples = 0;
            backbuffer = vec![Colour::ZERO; width * height];
        }

        if window.is_key_down(Key::A) {
            camera.translate(0.1 * Vec3::Y);
            samples = 0;
            backbuffer = vec![Colour::ZERO; width * height];
        }

        if window.is_key_down(Key::D) {
            camera.translate(-0.1 * Vec3::Y);
            samples = 0;
            backbuffer = vec![Colour::ZERO; width * height];
        }

        if window.is_key_down(Key::Space) {
            camera.translate(0.1 * Vec3::Z);
            samples = 0;
            backbuffer = vec![Colour::ZERO; width * height];
        }

        if window.is_key_down(Key::LeftCtrl) {
            camera.translate(-0.1 * Vec3::Z);
            samples = 0;
            backbuffer = vec![Colour::ZERO; width * height];
        }
        
        // Rendering and pushing to the window
        samples += 1;
        
        /*
        for x in 0..width {
            for y in 0..height {
                backbuffer[x + y * width] += trace(scene, camera.generate_ray(x as u32, y as u32, width as u32, height as u32), 5);
                buffer[x + y * width] = (backbuffer[x + y * width]/samples as Float).to_u32_rgb()
            }
        }
        */

        backbuffer = backbuffer.iter().zip(pixels.iter()).map(|(col, (x, y))| {
            *col + trace(scene, camera.generate_ray(*x, *y, width, height), 5)
        }).collect();

        buffer = backbuffer.par_iter().map(|&col| (col/samples as Float).to_u32_rgb()).collect();

        window.update_with_buffer(&buffer, width, height).unwrap();

    }
}



fn main() {
    //let cornell_camera_pos = Vec3::new(0., 0., -1.);
    //let cornell_camera = camera::SimpleCamera::new(PI/2., cornell_camera_pos, -cornell_camera_pos.normalise(), Vec3::Y);

    let sphere_camera_pos = Vec3::new(2., 0.3, -4.);
    let mut sphere_camera = camera::SimpleCamera::new(PI/2., sphere_camera_pos, -sphere_camera_pos.normalise(), Vec3::Y);

    
    render_window_iter(&sphere_test_scene(), &mut sphere_camera, 1280, 720);

    println!("All done");

}

/// Test background
fn _background(_: Ray) -> Colour {
    Colour::BLACK
}


const SUN_DIRECTION: Vec3 = Vec3::new(-0.577350, 0.577350, -0.577350);
const SKY_COLOUR: Colour = Colour::new(0.45, 0.68, 0.87);

/// Looks a bit like the real sky or whatever
fn background(ray: Ray) -> Colour {
    let sun = (dot(SUN_DIRECTION, ray.direction) + 0.03).min(1.).max(0.).powf(300.) * Colour::WHITE;
    
    let lerp = (0.5 + ray.direction.y/2.).powf(1.5);
    let sky = (1. - lerp) * SKY_COLOUR + lerp * Colour::WHITE;

    sun + 0.4 * sky
}

fn trace<T: Object>(scene: &T, ray: Ray, depth: usize) -> Colour {
    if depth <= 0 { return Colour::BLACK };

    //let hit = scene.intersect(ray);

    match scene.intersect(ray) {
        // This notation is actually a bit cumbersome; it unwraps the hit
        // whilst ignoring distance which isn't used. I'm not sure whether
        // this is fast or not
        Some(ObjectHit{point, normal, dist: _, material, colour}) => colour * match material {
            Material::Lambert => {
                let new_direction = random_hemisphere(normal);
                if dot(new_direction, normal) <= 0. {
                    println!("Now this is good")
                }
                2. * dot(new_direction, normal).max(0.) * trace(scene,
                    Ray::new(point + EPS * normal, new_direction), depth - 1)
            },

            Material::LambertCos => {
                let new_direction = random_hemisphere_cosine(normal);

                trace(scene, Ray::new(point + EPS * normal, new_direction), depth - 1)
            }

            Material::Mirror => {
                let new_direction = reflect(ray.direction, normal); // dot(new_direction, normal).max(0.) * 
                trace(scene,
                    Ray::new(point + EPS * normal, new_direction), depth - 1)
            },

            // Glass with refractive index refr
            Material::Glass(n_dielectric) => {
                let cos = dot(ray.direction, normal);
                let ratio: Float;
                //let sl: Float;
                //let refract_ray: Ray;

                if cos < 0. {
                    // Ray coming from outside
                    ratio = 1./n_dielectric;
                } else {
                    // Ray coming from inside
                    ratio = n_dielectric;
                }

                match refract(ray.direction, -cos.signum() * normal, ratio) {
                    Some(refract_direction) => {
                        let schlick_factor = schlick(cos.abs(), n_dielectric);

                        let reflect_direction = reflect(ray.direction, -cos.signum() * normal);

                        if random_float() < schlick_factor { //dot(reflect_direction, normal).abs() * 
                            trace(scene,
                                Ray::new(point + EPS * -cos.signum() * normal, reflect_direction), depth - 1)    
                        } else {
                            trace(scene,
                                Ray::new(point + EPS * cos.signum() * normal, refract_direction), depth - 1)    
                        }

                    },
                    None => {
                        let new_direction = reflect(ray.direction, normal);
                        
                        trace(scene,
                            Ray::new(point + EPS * -cos.signum() * normal, new_direction), depth - 1)
                    },
                }

            }

            Material::Light(intensity) => intensity * Colour::WHITE,
            
            Material::Test => Colour::WHITE,
        },

        None => background(ray),
    }
}

fn cornell_box_scene() -> Vec<Box<dyn Object>> {

    let red = Colour::new(0.71, 0., 0.);
    let green = Colour::new(0., 0.71, 0.);

    let bottom = GeneralObject {
        shape: Plane {
            normal: Vec3::Y,
            size: -1.,
        },
        material: material::Material::LambertCos,
        colour: Colour::WHITE,
    };

    let top = GeneralObject {
        shape: Plane {
            normal: -Vec3::Y,
            size: -1.,
        },
        material: material::Material::LambertCos,
        colour: Colour::WHITE,
    };

    let left = GeneralObject {
        shape: Plane {
            normal: -Vec3::X,
            size: -1.,
        },
        material: material::Material::LambertCos,
        colour: red,
    };

    let right = GeneralObject {
        shape: Plane {
            normal: Vec3::X,
            size: -1.,
        },
        material: material::Material::LambertCos,
        colour: green,
    };

    let back = GeneralObject {
        shape: Plane {
            normal: -Vec3::Z,
            size: -1.,
        },
        material: material::Material::LambertCos,
        colour: Colour::WHITE,
    };

    let light = GeneralObject {
        shape: Sphere {
            centre: Vec3::Y,
            radius: 0.6,
        },
        material: material::Material::Light(3.),
        colour: Colour::new(0.859, 0.776, 0.569),
    };

    vec![Box::new(bottom), Box::new(top), Box::new(left), Box::new(right), Box::new(back), Box::new(light)]
}


fn sphere_test_scene() -> Vec<Box<dyn Object>> {
    let left = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: Vec3::X,
            radius: 1.,
        },
        material: Material::Glass(1.54),
        colour: Colour::WHITE,
    };

    let middle = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: -Vec3::X,
            radius: 1.,
        },
        material: Material::LambertCos,
        colour: Colour::new(1., 1., 0.),
    };

    let right = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: -3. * Vec3::X - 0.2 * Vec3::Y,
            radius: 0.8,
        },
        material: Material::Mirror,
        colour: Colour::new(0.8, 0.4, 0.4),
    };

    let floor = GeneralObject::<Plane> {
        shape: Plane::new(Vec3::Y, -Vec3::Y),
        material: Material::LambertCos,
        colour: Colour::new(0.3, 0.25, 0.25),
    };

    vec![Box::new(left), Box::new(middle), Box::new(right), Box::new(floor)]
}


fn obj_scene() -> Vec<Box<dyn Object>> {
    /* OBJ Mesh */

    
    let obj_path = Path::new("octahedron.obj");
    let obj_mesh: obj::Obj<obj::SimplePolygon> = obj::Obj::load(obj_path).expect("Failed to load obj file");
    
    let obj = ObjectCollection::<Triangle> {
        shapes: convert_objects_to_polygons(&obj_mesh),
        material: Material::Glass(1.54),
        colour: Colour::new(1., 1., 1.),
    };
    
    let floor = GeneralObject::<Plane> {
        shape: Plane::new(Vec3::Y, -Vec3::Y),
        material: Material::LambertCos,
        colour: Colour::new(0.3, 0.25, 0.25),
    };

    vec![Box::new(obj), Box::new(floor)]
}