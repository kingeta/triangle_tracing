//#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use std::ffi::CString;
use std::collections::HashSet;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use cgmath::prelude::*;
//extern crate gl;
//use gl;

pub mod render_gl;

fn main() {
    // Starting SDL
    let (mut window_w, mut window_h) = (1200i32, 650i32);
    let sld_context = sdl2::init().unwrap();
    let video_subsystem = sld_context.video().unwrap();

    // OpenGL stuff
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 3); //4, 5
    
    let window = video_subsystem.window("Test", window_w as u32, window_h as u32)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Bind and create shaders
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("shaders/quad.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("shaders/pt.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    // Setup the background quad
    let quad: Vec<f32> = vec![
        -1.0,  1.0,  0.0,
        -1.0, -1.0,  0.0,
         1.0, -1.0,  0.0,
         1.0, -1.0,  0.0,
         1.0,  1.0,  0.0,
        -1.0,  1.0,  0.0    
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // Target
            (quad.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size (bytes) of data
            quad.as_ptr() as *const gl::types::GLvoid, // data pointer
            gl::STATIC_DRAW, // use
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); //unbind buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); //layout location = 0
        gl::VertexAttribPointer(
            0, //Index of generic vertex attribute layout location = 0
            3, // # components per generic vertex attribute
            gl::FLOAT, //dtype
            gl::FALSE, //normalised
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // byte offset between attributes i.e. stride
            std::ptr::null() //offset of first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    /*let mut tex_rand: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut tex_rand);
        gl::BindTexture(gl::TEXTURE_1D, tex_rand);
        gl::TexStorage1D(gl::TEXTURE_1D, 1, gl::RGB8, 100);
        gl::TexSubImage1D()
    }*/

    let mut tex_id: gl::types::GLuint = 0;
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::GenTextures(1, &mut tex_id);
        gl::BindTexture(gl::TEXTURE_1D, tex_id);
        gl::TexStorage1D(gl::TEXTURE_1D, 1, gl::RGB8, 4);
        gl::TexSubImage1D(gl::TEXTURE_1D, 0, 0, 4, gl::R32I, gl::UNSIGNED_BYTE, vec![4 as i32, 12 as i32, 200 as i32, 150 as i32].as_ptr() as *const std::ffi::c_void);

        gl::TexParameteri(gl::TEXTURE_1D, gl::TEXTURE_MIN_FILTER, 0);
        gl::BindTexture(gl::TEXTURE_1D, tex_id);
    }

    // Setup some random opengl stuff
    unsafe {
        gl::Viewport(0, 0, window_w, window_h);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    // Uniforms
    let viewport_handle: gl::types::GLint;
    unsafe {
        let viewport_name = &CString::new("viewport").unwrap();
        viewport_handle = gl::GetUniformLocation(
            shader_program.id(),
            viewport_name.as_ptr() as *const gl::types::GLchar
        );

        gl::ProgramUniform4i(
            shader_program.id(),
            viewport_handle,
            0,
            0,
            window_w,
            window_h
        );
    }


    //let mut position: Vec<f32> = vec![0., 0., 0.];
    let mut position: cgmath::Vector3<f32> = 3. * cgmath::Vector3::<f32>::unit_z();
    let position_uniform = render_gl::Uniform::new("origin", shader_program.id()).unwrap();

    //let (mut mouse_x, mut mouse_y): (i32, i32) = (0, 0);
    //let (mut mouse_x_old, mut mouse_y_old): (i32, i32);
    //let (mut theta, mut phi): (f32, f32) = (0., 0.);
    
    let mut direction: cgmath::Vector3<f32>; // = -cgmath::Vector3::<f32>::unit_z();
    let direction_uniform = render_gl::Uniform::new("forward", shader_program.id()).unwrap();
    let mut horizontal_angle: f32 = 0.;
    let mut vertical_angle: f32 = 0.;
    
    let up: cgmath::Vector3<f32> = cgmath::Vector3::<f32>::unit_y();
    let up_uniform = render_gl::Uniform::new("up", shader_program.id()).unwrap();


    let focus_dist_uniform = render_gl::Uniform::new("focus_dist", shader_program.id()).unwrap();
    let mut focus_dist: f32 = 2.;

    let focus_radius_uniform = render_gl::Uniform::new("DOF_RADIUS", shader_program.id()).unwrap();
    let mut focus_radius: f32 = 0.;

    let mut canvas_side;
    let mut canvas_up;

    let speed = 0.005; // in units per millisecond
    //let mouse_speed = 0.005 / (2. * 3.141); // in radians per pixel
    let mut time: usize = 0;
    let mut new_time: usize;
    let mut current_time: usize = 0;
    let mut frame_time: usize;


    let frame_uniform = render_gl::Uniform::new("frame", shader_program.id()).unwrap();



    let keys_list = vec![Keycode::W, Keycode::A, Keycode::S, Keycode::D, Keycode::Space, Keycode::C, Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right];
    let mut keys_down: HashSet<Keycode> = HashSet::new();
    let mut focus = false;

    let mut event_pump = sld_context.event_pump().unwrap();
    'main: loop {

        loop {for event in event_pump.poll_iter() {
            match event {

                Event::KeyDown { keycode: Some(Keycode::Escape), .. } | Event::KeyDown { keycode: Some(Keycode::Q), .. } | Event::Quit { .. } => {
                    break 'main;
                }

                Event::KeyDown { keycode: Some(x), repeat: false, .. } => {
                    if keys_list.contains(&x) { keys_down.insert(x); };
                }

                Event::KeyUp { keycode: Some(y), repeat: false, .. } => {
                    if keys_list.contains(&y) { keys_down.remove(&y); }; 
                }

                Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, .. } => {
                    focus = !focus;
                    sld_context.mouse().set_relative_mouse_mode(focus);
                }
                
                Event::MouseMotion { xrel: x, yrel: y, .. } => {
                	if focus {
                        horizontal_angle += x as f32 / window_w as f32 * 6.;
                        vertical_angle += y as f32 / window_w as f32 * 6.;
                        vertical_angle = vertical_angle.max(-3.141/2.).min(3.141/2.);
                        //println!("{}", state.y());	
                	}
                	//println!("{}, {}", x, y);
                }

                Event::MouseWheel { direction: dir, .. } => {
                    focus_dist += dir.to_ll() as f32 / 10.;
                }

                Event::Window { win_event, .. } => {
                    if let sdl2::event::WindowEvent::Resized(new_w, new_h) = win_event {
                        window_w = new_w;
                        window_h = new_h;
                        unsafe {
                            gl::Viewport(0, 0, window_w, window_h);
                            
                            gl::ProgramUniform4i(
                                shader_program.id(),
                                viewport_handle,
                                0,
                                0,
                                window_w,
                                window_h
                            );
                        }
                    }
                }

                _ => {break},
            }}
            
            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader_program.set_used();
            
            //println!("{}, {}", canvas_side.magnitude(), canvas_up.magnitude());

            direction = cgmath::vec3(horizontal_angle.sin(), -vertical_angle.sin(), -horizontal_angle.cos() * vertical_angle.cos());
            canvas_side = up.cross(direction).normalize(); //norm cross up direction
            canvas_up = direction.cross(canvas_side);
            //println!("{:?}, {:?}, {:?}, {:?}, {:?}", horizontal_angle, vertical_angle, direction, canvas_up, canvas_side);
           
            //println!("Side: {:?}, up: {:?}, direction: {:?}", canvas_side.magnitude(), canvas_up.magnitude(), direction.magnitude());

            new_time = sld_context.timer().unwrap().ticks() as usize;
            frame_time = new_time - current_time;
            current_time = new_time;
            //println!("{}", frame_time);
            // Time stuff


            if keys_down.contains(&Keycode::W) {
                position += speed * direction * frame_time as f32;
            } else if keys_down.contains(&Keycode::S) {
                position -= speed * direction * frame_time as f32;
            }
            
            if keys_down.contains(&Keycode::Space) {
                position += speed * canvas_up * frame_time as f32;
            } else if keys_down.contains(&Keycode::C) {
                position -= speed * canvas_up * frame_time as f32;
            }
            
            if keys_down.contains(&Keycode::A) {
                position += speed * canvas_side * frame_time as f32;
            } else if keys_down.contains(&Keycode::D) {
                position -= speed * canvas_side * frame_time as f32;
            }

            if keys_down.contains(&Keycode::Up) {
                focus_dist += 0.01 * frame_time as f32;
            } else if keys_down.contains(&Keycode::Down) {
                focus_dist -= 0.01 * frame_time as f32;
            }
            
            if keys_down.contains(&Keycode::Right) {
                focus_radius += 0.001 * frame_time as f32;
            } else if keys_down.contains(&Keycode::Left) {
                focus_radius -= 0.001 * frame_time as f32;
            }

            focus_dist = focus_dist.max(0.);
            focus_radius = focus_radius.max(0.);

            time += frame_time;


            position_uniform.push_3f(position);
            direction_uniform.push_3f(direction);
            up_uniform.push_3f(up);
            focus_dist_uniform.push_1f(focus_dist);
            focus_radius_uniform.push_1f(focus_radius);
            frame_uniform.push_1ui(time as u32);
	
            /*unsafe {
                let test_name = CString::new("textureSampler").unwrap();
                let test = gl::GetUniformLocation(
                    shader_program.id(),
                    test_name.as_ptr() as *const gl::types::GLchar
                );
            */


            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // start index
                    6 // # indices to be rendered
                )
            }
            
            window.gl_swap_window();
        }
    }
}
