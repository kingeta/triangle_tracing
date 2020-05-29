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

use std::time::Instant;

fn render<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize, samples: usize, depth: usize, filename: String) {
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
            col += trace(scene, camera.generate_ray(x as usize, y as usize, width, height), depth);
        }

        *pixel = (col / samples as Float).to_image_rgb();

        //*pixel = trace(&sphere, main_camera.generate_ray(x, y, width, height), 5).to_rgb();
    }

    result.save(filename).unwrap();
}


fn render_iter<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize, samples: usize, depth: usize) {
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
            colour += trace(scene, camera.generate_ray(x, y, width, height), depth)
        }

        buffer[i] = (colour / samples as Float).to_u32_rgb();
    });

    let mut window = Window::new("Pathtracing", width, height, WindowOptions::default())
    .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}


fn render_window<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize, depth: usize) {

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
                backbuffer[x + y * width] += trace(scene, camera.generate_ray(x, y, width, height), depth);
                buffer[x + y * width] = (backbuffer[x + y * width]/samples as Float).to_u32_rgb()
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();

    }
}

fn render_window_iter<O: Object, C: Camera>(scene: &O, camera: &mut C, width: usize, height: usize, depth: usize) {
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
            *col + trace(scene, camera.generate_ray(*x, *y, width, height), depth)
        }).collect();

        buffer = backbuffer.par_iter().map(|&col| (col/samples as Float).to_u32_rgb()).collect();

        window.update_with_buffer(&buffer, width, height).unwrap();

    }
}



fn main() {
    let now = Instant::now();

    let cornell_camera_pos = Vec3::new(0., 0., -3.1);
    let mut cornell_camera = camera::SimpleAACamera::new(PI/3., cornell_camera_pos, -cornell_camera_pos.normalise(), Vec3::Y);

    let setting_up = now.elapsed().as_millis();
    println!("Setting up: {} milliseconds", setting_up);
    //let sphere_camera_pos = Vec3::new(2., 0.3, -4.);
    //let mut sphere_camera = camera::SimpleCamera::new(PI/2., sphere_camera_pos, -sphere_camera_pos.normalise(), Vec3::Y);

    
    render_window_iter(&cornell_box_scene(), &mut cornell_camera, 512, 512, 4); //, "test.png".to_string()

    println!("Rendering: {} milliseconds", now.elapsed().as_millis() - setting_up);
}

/// Test background
fn background(_: Ray) -> Colour {
    Colour::BLACK
}


const SUN_DIRECTION: Vec3 = Vec3::new(-0.577350, 0.577350, -0.577350);
const SKY_COLOUR: Colour = Colour::new(0.45, 0.68, 0.87);

/// Looks a bit like the real sky or whatever
fn _background(ray: Ray) -> Colour {
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
            Material::Lambert(albedo) => {
                let new_direction = random_hemisphere(normal);
                if dot(new_direction, normal) <= 0. {
                    println!("Now this is good")
                }
                2. * dot(new_direction, normal).max(0.) * albedo * trace(scene,
                    Ray::new(point + EPS * normal, new_direction), depth - 1)
            },

            Material::LambertCos(albedo) => {

                if dot(ray.direction, normal) < 0. {
                    let new_direction = random_hemisphere_cosine(normal);
                    albedo * trace(scene, Ray::new(point + EPS * normal, new_direction), depth - 1)
                } else {
                    Colour::BLACK
                }
            }

            Material::Mirror(albedo) => {
                let new_direction = reflect(ray.direction, normal); // dot(new_direction, normal).max(0.) * 
                albedo * trace(scene,
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

            Material::Scatter(g) => {
                let new_direction = random_float().signum() * random_vector(); // Not great way of sampling from whole sphere
                let cos = dot(new_direction, ray.direction);

                henyey_greenstein(cos, g) * colour * trace(scene,
                    Ray::new(point, new_direction), depth - 1)/ (2.*PI)
            }

            Material::Light(intensity) => intensity * Colour::WHITE, // Note lights are omnidirectional; they light in front and behind of themselves
            
            Material::LightUni(intensity) => intensity * dot(normal, ray.direction).is_sign_negative() as usize as Float * Colour::WHITE, // Unidirectional light
            
            Material::LightCos(intensity) => dot(ray.direction, -normal).max(0.).powf(100.) * intensity * Colour::WHITE, // This is unidirectional

            Material::Test => Colour::WHITE,
        },

        None => background(ray),
    }
}

fn cornell_box_scene() -> Vec<Box<dyn Object>> {

    let red = Colour::new(0.71, 0., 0.);
    let green = Colour::new(0., 0.71, 0.);
    let factor: Float = 1.02; // Extend all squares a bit at the edge

    let _bottom = GeneralObject {
        shape: Plane {
            normal: Vec3::Y,
            size: -1.,
        },
        material: material::Material::LambertCos(0.9),
        colour: Colour::WHITE,
    };

    let bottom2 = ObjectCollection::<Triangle>::square(
            -Vec3::Y + factor * (Vec3::Z + Vec3::X),
            -Vec3::Y + factor * (Vec3::Z - Vec3::X),
            -Vec3::Y + factor * (-Vec3::Z - Vec3::X),
            -Vec3::Y + factor * (-Vec3::Z + Vec3::X),
            Material::LambertCos(0.8), Colour::WHITE);

    let _top = GeneralObject {
        shape: Plane {
            normal: Vec3::Y,
            size: 1.,
        },
        material: material::Material::LambertCos(0.9),
        colour: Colour::WHITE,
    };

    let top2 = ObjectCollection::<Triangle>::square(
            Vec3::Y + factor * (Vec3::Z + Vec3::X),
            Vec3::Y + factor * (-Vec3::Z + Vec3::X),
            Vec3::Y + factor * (-Vec3::Z - Vec3::X),
            Vec3::Y + factor * (Vec3::Z - Vec3::X),
            Material::LambertCos(0.8), Colour::WHITE);

    let _left = GeneralObject {
        shape: Plane {
            normal: -Vec3::X,
            size: -1.,
        },
        material: material::Material::LambertCos(0.9),
        colour: red,
    };

    let left2 = ObjectCollection::<Triangle>::square(
            Vec3::X + factor * (Vec3::Y - Vec3::Z),
            Vec3::X + factor * (Vec3::Y + Vec3::Z),
            Vec3::X + factor * (-Vec3::Y + Vec3::Z),
            Vec3::X + factor * (-Vec3::Y - Vec3::Z),
            Material::LambertCos(0.8), red);

    let _right = GeneralObject {
        shape: Plane {
            normal: Vec3::X,
            size: -1.,
        },
        material: material::Material::LambertCos(0.9),
        colour: green,
    };

    let right2 = ObjectCollection::<Triangle>::square(
            -Vec3::X + factor * (Vec3::Y + Vec3::Z),
            -Vec3::X + factor * (Vec3::Y - Vec3::Z),
            -Vec3::X + factor * (-Vec3::Y - Vec3::Z),
            -Vec3::X + factor * (-Vec3::Y + Vec3::Z),
            Material::LambertCos(0.8), green);

    let _back = GeneralObject {
        shape: Plane {
            normal: -Vec3::Z,
            size: -1.,
        },
        material: material::Material::LambertCos(0.9),
        colour: Colour::WHITE,
    };

    let back2 = ObjectCollection::<Triangle>::square(
            Vec3::Z + 1.05*(Vec3::X + Vec3::Y),
            Vec3::Z + factor * (-Vec3::X + Vec3::Y),
            Vec3::Z + factor * (-Vec3::X - Vec3::Y),
            Vec3::Z + factor * (Vec3::X - Vec3::Y),
            Material::LambertCos(0.8), Colour::WHITE);


    let _light = GeneralObject {
        shape: Sphere {
            centre: Vec3::Y,
            radius: 0.6,
        },
        material: material::Material::Light(3.),
        colour: Colour::new(0.859, 0.776, 0.569),
    };

    let light2 = ObjectCollection::<Triangle>::square(
            0.98 * Vec3::Y + 0.6 * (Vec3::Z + Vec3::X), 
            0.98 * Vec3::Y + 0.6 * (-Vec3::Z + Vec3::X),
            0.98 * Vec3::Y + 0.6 * (-Vec3::Z - Vec3::X),
            0.98 * Vec3::Y + 0.6 * (Vec3::Z - Vec3::X),
            Material::LightUni(3.), Colour::new(1.0, 0.776, 0.4));

    
    let mirror_ball = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: Vec3::new(0.45, -0.7, 0.),
            radius: 0.3,
        },
        material: Material::Mirror(1.),
        colour: Colour::WHITE
    };
    
    let glass_ball = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: Vec3::new(-0.45, -0.6, -0.2),
            radius: 0.4,
        },
        material: Material::Glass(1.54),
        colour: Colour::WHITE
    };
    
    let gas = MediumObject {
        density: 0.3,
        material: Material::Scatter(1.),
        colour: Colour::WHITE,
    };

    vec![Box::new(gas), Box::new(bottom2), Box::new(top2), Box::new(left2), Box::new(right2), Box::new(back2), Box::new(light2), Box::new(mirror_ball), Box::new(glass_ball)]
    //vec![Box::new(gas), Box::new(light2), Box::new(mirror_ball), Box::new(glass_ball)]
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
        material: Material::LambertCos(0.9),
        colour: Colour::new(1., 1., 0.),
    };

    let right = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: -3. * Vec3::X - 0.2 * Vec3::Y,
            radius: 0.8,
        },
        material: Material::Mirror(0.95),
        colour: Colour::new(0.8, 0.4, 0.4),
    };

    let floor = GeneralObject::<Plane> {
        shape: Plane::new(Vec3::Y, -Vec3::Y),
        material: Material::LambertCos(0.9),
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
        material: Material::LambertCos(0.9),
        colour: Colour::new(0.3, 0.25, 0.25),
    };

    vec![Box::new(obj), Box::new(floor)]
}