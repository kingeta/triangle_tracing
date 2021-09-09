use std::path::Path;
use std::sync::Arc;

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
use camera::{Camera, SimpleAACamera, SimpleCamera, DOFCamera};

use minifb::{Key, Window, WindowOptions};

use rayon::prelude::*;

use std::time::Instant;




fn render<O: Object, C: Camera>(scene: &O, camera: &C, width: usize, height: usize, samples: usize, depth: usize, filename: String) {
    // The final image
    let mut result: RgbImage = ImageBuffer::new(width as u32, height as u32);


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


fn render_window_iter<O: Object + Sync + Send, C: Camera + Sync + Send>(scene: &O, camera: &mut C, width: usize, height: usize, depth: usize) {
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

    while window.is_open() && !window.is_key_down(Key::Escape){
        
        
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
        
        
        // Way with the pixels vector or whatever
        
        backbuffer = backbuffer.par_iter().zip(pixels.par_iter()).map(|(col, (x, y))| {
            *col + trace(scene, camera.generate_ray(*x, *y, width, height), depth)
        }).collect();
        

        /*
        backbuffer = backbuffer.par_iter().enumerate().map(|(i, &col)| {
            col + trace(scene, camera.generate_ray(i % width, i / width, width, height), depth)
        }).collect();
        */

        buffer = backbuffer.par_iter().map(|&col| (col/samples as Float).to_u32_rgb_filmic()).collect();

        window.update_with_buffer(&buffer, width, height).unwrap();

    }
}



fn main_() {
    let now = Instant::now();

    // Phone wallpaper thing
    //let cornell_camera_pos = Vec3::new(0., 0., -5.3);
    //let cornell_camera = SimpleAACamera::new(PI/6., cornell_camera_pos, -cornell_camera_pos.normalise(), Vec3::Y);
    // render(&cornell_phone_scene(), &cornell_camera, 1440, 3120, 512, 4, "phone_cornell.png".to_string()); //, "test.png".to_string()

    let cornell_camera_pos = Vec3::new(0., 0., -3.1);
    let mut cornell_camera = DOFCamera::new(PI/3., cornell_camera_pos, -cornell_camera_pos.normalise(), Vec3::Y, 3.1, 0.0);

    let sphere_camera_pos = Vec3::new(4., 0.6, -8.);
    let mut sphere_camera = DOFCamera::new(PI/3., sphere_camera_pos, -sphere_camera_pos.normalise(), Vec3::Y, sphere_camera_pos.norm(), 0.1);

    let setting_up = now.elapsed().as_millis();
    println!("Setting up: {} milliseconds", setting_up);

    
    render_window_iter(&cornell_box_scene(), &mut cornell_camera, 1024, 1024, 12); //, "test.png".to_string()

    println!("Rendering: {} seconds", (now.elapsed().as_millis() - setting_up) / 1000);
}



/// Test background
fn background(_: Ray) -> Colour {
    Colour::BLACK
}


const SUN_DIRECTION: Vec3 = Vec3::new(-0.577350, 0.577350, -0.577350);
const SKY_COLOUR: Colour = Colour::new(0.45, 0.68, 0.87);

/// Looks a bit like the real sky or whatever
fn _background(ray: Ray) -> Colour {
    let sun = (dot(SUN_DIRECTION, ray.direction) + 0.03).min(1.).max(0.).powf(100.) * Colour::WHITE;
    
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

fn cornell_box_scene() -> Vec<Box<dyn Object + Sync + Send>> {

    let red = Colour::new(0.71, 0., 0.);
    let green = Colour::new(0., 0.71, 0.);
    let factor: Float = 1.01; // Extend all squares a bit at the edge

    let bottom = ObjectCollection::<Triangle>::rect(
            -Vec3::Y + factor * (Vec3::Z + Vec3::X),
            -Vec3::Y + factor * (Vec3::Z - Vec3::X),
            -Vec3::Y + factor * (-Vec3::Z - Vec3::X),
            -Vec3::Y + factor * (-Vec3::Z + Vec3::X),
            Material::LambertCos(0.8), Colour::WHITE);

    let top = ObjectCollection::<Triangle>::rect(
            Vec3::Y + factor * (Vec3::Z + Vec3::X),
            Vec3::Y + factor * (-Vec3::Z + Vec3::X),
            Vec3::Y + factor * (-Vec3::Z - Vec3::X),
            Vec3::Y + factor * (Vec3::Z - Vec3::X),
            Material::LambertCos(0.8), Colour::WHITE);

    let left = ObjectCollection::<Triangle>::rect(
            Vec3::X + factor * (Vec3::Y - Vec3::Z),
            Vec3::X + factor * (Vec3::Y + Vec3::Z),
            Vec3::X + factor * (-Vec3::Y + Vec3::Z),
            Vec3::X + factor * (-Vec3::Y - Vec3::Z),
            Material::LambertCos(0.8), red);


    let right = ObjectCollection::<Triangle>::rect(
            -Vec3::X + factor * (Vec3::Y + Vec3::Z),
            -Vec3::X + factor * (Vec3::Y - Vec3::Z),
            -Vec3::X + factor * (-Vec3::Y - Vec3::Z),
            -Vec3::X + factor * (-Vec3::Y + Vec3::Z),
            Material::LambertCos(0.8), green);

    let back = ObjectCollection::<Triangle>::rect(
            Vec3::Z + 1.05*(Vec3::X + Vec3::Y),
            Vec3::Z + factor * (-Vec3::X + Vec3::Y),
            Vec3::Z + factor * (-Vec3::X - Vec3::Y),
            Vec3::Z + factor * (Vec3::X - Vec3::Y),
            Material::LambertCos(0.8), Colour::WHITE);



    let light = ObjectCollection::<Triangle>::rect(
            0.99 * Vec3::Y + 0.6 * (Vec3::Z + Vec3::X), 
            0.99 * Vec3::Y + 0.6 * (-Vec3::Z + Vec3::X),
            0.99 * Vec3::Y + 0.6 * (-Vec3::Z - Vec3::X),
            0.99 * Vec3::Y + 0.6 * (Vec3::Z - Vec3::X),
            Material::Light(6.), Colour::new(1.0, 0.776, 0.4));

    
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
    
    let glass_cube = ObjectCollection::<Triangle>::cuboid(
        Vec3::new(-0.45, -0.65, -0.2),
        0.7 * Vec3::Y,
        Vec3::new(3. *0.2828, 0., 0.2828) / 2.,
        Vec3::new(-0.2828, 0., 3. * 0.2828) / 2.,
        Material::Glass(1.54),
        Colour::WHITE,
    );

    let gas = MediumObject {
        density: 1.,
        material: Material::Scatter(0.65),
        colour: Colour::WHITE,
    };

    let glass_lens = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: -6. * Vec3::Z,
            radius: 3.2,
        },
        material: Material::Glass(0.5),
        colour: Colour::WHITE,
    };

    vec![Box::new(bottom), Box::new(top), Box::new(left), Box::new(right), Box::new(back), Box::new(light), Box::new(mirror_ball), Box::new(glass_cube)]
    //vec![Box::new(gas), Box::new(light2), Box::new(mirror_ball), Box::new(glass_ball)]
}

fn cornell_phone_scene() -> Vec<Box<dyn Object>> {

    let red = Colour::new(0.71, 0., 0.);
    let green = Colour::new(0., 0.71, 0.);
    let factor: Float = 1.02; // Extend all squares a bit at the edge

    let ratio = 19.5/9.; // Screen ratio of OnePlus 7T Pro

    let bottom = ObjectCollection::<Triangle>::rect(
            -ratio * Vec3::Y + factor * (Vec3::Z + Vec3::X),
            -ratio * Vec3::Y + factor * (Vec3::Z - Vec3::X),
            -ratio * Vec3::Y + factor * (-Vec3::Z - Vec3::X),
            -ratio * Vec3::Y + factor * (-Vec3::Z + Vec3::X),
            Material::LambertCos(0.8), Colour::WHITE);

    let top = ObjectCollection::<Triangle>::rect(
            ratio * Vec3::Y + factor * (Vec3::Z + Vec3::X),
            ratio * Vec3::Y + factor * (-Vec3::Z + Vec3::X),
            ratio * Vec3::Y + factor * (-Vec3::Z - Vec3::X),
            ratio * Vec3::Y + factor * (Vec3::Z - Vec3::X),
            Material::LambertCos(0.8), Colour::WHITE);

    let left = ObjectCollection::<Triangle>::rect(
            Vec3::X + factor * (ratio * Vec3::Y - Vec3::Z),
            Vec3::X + factor * (ratio * Vec3::Y + Vec3::Z),
            Vec3::X + factor * (-ratio * Vec3::Y + Vec3::Z),
            Vec3::X + factor * (-ratio * Vec3::Y - Vec3::Z),
            Material::LambertCos(0.8), red);

    let right = ObjectCollection::<Triangle>::rect(
            -Vec3::X + factor * (ratio * Vec3::Y + Vec3::Z),
            -Vec3::X + factor * (ratio * Vec3::Y - Vec3::Z),
            -Vec3::X + factor * (-ratio * Vec3::Y - Vec3::Z),
            -Vec3::X + factor * (-ratio * Vec3::Y + Vec3::Z),
            Material::LambertCos(0.8), green);


    let back = ObjectCollection::<Triangle>::rect(
            Vec3::Z + factor * (Vec3::X + ratio * Vec3::Y),
            Vec3::Z + factor * (-Vec3::X + ratio * Vec3::Y),
            Vec3::Z + factor * (-Vec3::X - ratio * Vec3::Y),
            Vec3::Z + factor * (Vec3::X - ratio * Vec3::Y),
            Material::LambertCos(0.8), Colour::WHITE);


    let light = ObjectCollection::<Triangle>::rect(
            (ratio - EPS) * Vec3::Y + 0.6 * (Vec3::Z + Vec3::X), 
            (ratio - EPS) * Vec3::Y + 0.6 * (-Vec3::Z + Vec3::X),
            (ratio - EPS) * Vec3::Y + 0.6 * (-Vec3::Z - Vec3::X),
            (ratio - EPS) * Vec3::Y + 0.6 * (Vec3::Z - Vec3::X),
            Material::Light(3.), Colour::new(1.0, 0.776, 0.4));

    
    let mirror_ball = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: Vec3::new(0.45, 0.3-ratio, 0.),
            radius: 0.3,
        },
        material: Material::Mirror(1.),
        colour: Colour::WHITE
    };
    
    let glass_ball = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: Vec3::new(-0.45, 0.4-ratio, -0.2),
            radius: 0.4,
        },
        material: Material::Glass(1.54),
        colour: Colour::WHITE
    };
    
    vec![Box::new(bottom), Box::new(top), Box::new(left), Box::new(right), Box::new(back), Box::new(light), Box::new(mirror_ball), Box::new(glass_ball)]
}


fn sphere_test_scene() -> Vec<Box<dyn Object + Sync + Send>> {
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

    let middle2 = ObjectCollection::<Triangle>::cuboid(
        Vec3::new(-1., -0.25, 0.),
        1.5 * Vec3::Y,
        1.5 * Vec3::X,
        1.5 * Vec3::Z,
        Material::LambertCos(0.9),
        Colour::new(1., 1., 0.),
    );

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


    let light = GeneralObject::<Sphere> {
        shape: Sphere {
            centre: 40. * Vec3::Z + 6. * Vec3::X - 0.5 * Vec3::Y,
            radius: 0.5,
        },
        material: Material::LightUni(4.),
        colour: Colour::new(0., 173., 223.) / 255.,
    };

    vec![Box::new(left), Box::new(middle2), Box::new(right), Box::new(floor), Box::new(light)]
}


fn obj_scene() -> Vec<Box<dyn Object + Sync + Send>> {
    /* OBJ Mesh */

    
    let obj_path = Path::new("shuttle.obj");
    let obj_mesh: obj::Obj<obj::SimplePolygon> = obj::Obj::load(obj_path).expect("Failed to load obj file");
    
    let obj = ObjectCollection::<Triangle> {
        shapes: convert_objects_to_polygons(&obj_mesh),
        //material: Material::Glass(1.54),
        material: Material::LambertCos(0.60),
        colour: Colour::new(1., 1., 1.),
    };
    
    let floor = GeneralObject::<Plane> {
        shape: Plane::new(Vec3::Y, -Vec3::Y),
        material: Material::LambertCos(0.9),
        colour: Colour::new(0.3, 0.25, 0.25),
    };

    vec![Box::new(obj), Box::new(floor)]
}
