#![allow(dead_code, unused_imports, unused_variables, unused_mut)]
use std::ffi::CString;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
//extern crate gl;
//use gl;

pub mod render_gl;

fn main() {
    let (window_w, window_h) = (800i32, 600i32);
    let sld_context = sdl2::init().unwrap();
    let video_subsystem = sld_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    
    let window = video_subsystem.window("Test", window_w as u32, window_h as u32)
        .opengl()
        //.resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("shaders/test.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("shaders/rt.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();


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

    unsafe {
        gl::Viewport(0, 0, window_w, window_h);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    let viewport_name = &CString::new("viewport").unwrap();
    unsafe {
        let viewport_handle: gl::types::GLint = gl::GetUniformLocation(
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


    let mut eye: Vec<f32> = vec![0., 0., -8.];
    let eye_name = &CString::new("eye").unwrap();
    let eye_handle: gl::types::GLint;
    unsafe {
        eye_handle = gl::GetUniformLocation(
            shader_program.id(),
            eye_name.as_ptr() as *const gl::types::GLchar
        );
    }


    let (mut mouse_x, mut mouse_y): (i32, i32) = (0, 0);
    let (mut mouse_x_old, mut mouse_y_old): (i32, i32);
    let (mut theta, mut phi): (f32, f32) = (0., 0.);
    let direction_name = &CString::new("direction").unwrap();
    let direction_handle: gl::types::GLint;
    unsafe {
        direction_handle = gl::GetUniformLocation(
            shader_program.id(),
            direction_name.as_ptr() as *const gl::types::GLchar
        );
    }

    let angle_name = &CString::new("angle").unwrap();
    let angle_handle: gl::types::GLint;
    unsafe {
        angle_handle = gl::GetUniformLocation(
            shader_program.id(),
            angle_name.as_ptr() as *const gl::types::GLchar
        );
    }

    let mut eye_vel: Vec<f32> = vec![0., 0., 0.];
    let mut down: Vec<bool> = vec![false, false, false, false, false, false, false, false, false, false, false, false];
    let mut angle = 0.;

    let speed = 0.01;

    let mut event_pump = sld_context.event_pump().unwrap();
    'main: loop {

        loop { for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(Keycode::W), repeat: false, .. } => {down[2] = true;break},
                Event::KeyDown { keycode: Some(Keycode::S), repeat: false, .. } => {down[3] = true;break},
                Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {down[1] = true;break},
                Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {down[0] = true;break},
                Event::KeyDown { keycode: Some(Keycode::Q), repeat: false, .. } => {down[6] = true;break},
                Event::KeyDown { keycode: Some(Keycode::E), repeat: false, .. } => {down[7] = true;break},
                Event::KeyDown { keycode: Some(Keycode::Left), repeat: false, .. } => {down[8] = true;break},
                Event::KeyDown { keycode: Some(Keycode::Right), repeat: false, .. } => {down[9] = true;break},
                Event::KeyDown { keycode: Some(Keycode::Up), repeat: false, .. } => {down[10] = true;break},
                Event::KeyDown { keycode: Some(Keycode::Down), repeat: false, .. } => {down[11] = true;break},
                /*Event::KeyDown { keycode: Some(Keycode::Space), repeat: false, .. } => {down[4] = true},
                Event::KeyDown { keycode: Some(Keycode::X), repeat: false, .. } => {down[5] = true},
                */

                Event::KeyUp { keycode: Some(Keycode::W), repeat: false, .. } => {down[2] = false;break},
                Event::KeyUp { keycode: Some(Keycode::S), repeat: false, .. } => {down[3] = false;break},
                Event::KeyUp { keycode: Some(Keycode::A), repeat: false, .. } => {down[1] = false;break},
                Event::KeyUp { keycode: Some(Keycode::D), repeat: false, .. } => {down[0] = false;break},
                Event::KeyUp { keycode: Some(Keycode::Q), repeat: false, .. } => {down[6] = false;break},
                Event::KeyUp { keycode: Some(Keycode::E), repeat: false, .. } => {down[7] = false;break},
                Event::KeyUp { keycode: Some(Keycode::Left), repeat: false, .. } => {down[8] = false;break},
                Event::KeyUp { keycode: Some(Keycode::Right), repeat: false, .. } => {down[9] = false;break},
                Event::KeyUp { keycode: Some(Keycode::Up), repeat: false, .. } => {down[10] = false;break},
                Event::KeyUp { keycode: Some(Keycode::Down), repeat: false, .. } => {down[11] = false;break},
                /*Event::KeyUp { keycode: Some(Keycode::Space), repeat: false, .. } => {down[4] = false},
                Event::KeyUp { keycode: Some(Keycode::X), repeat: false, .. } => {down[5] = false},
                */

                Event::MouseMotion {
                    x, y, .. //mouse_btn: sdl2::mouse::MouseButton::Left, 
                } => {
                    //mouse_x = 2. * (x as f32 / window_w as f32 - 0.5);
                    //mouse_y = 2. * (y as f32 / window_h as f32 - 0.5);
                    //mouse_x = (2*x - window_w) as f32 / window_h as f32;
                    //mouse_y = (2*y - window_h) as f32 / window_h as f32;
                    /*mouse_x_old = mouse_x;
                    mouse_y_old = mouse_y;
                    mouse_x = x;
                    mouse_y = y;*/
                }

                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    down[4] = true;
                    break;
                }

                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Left,
                    ..
                } => {
                    down[4] = false;
                    break;
                }

                Event::MouseButtonDown {
                    mouse_btn: sdl2::mouse::MouseButton::Right,
                    ..
                } => {
                    down[5] = true;
                    break;
                }

                Event::MouseButtonUp {
                    mouse_btn: sdl2::mouse::MouseButton::Right,
                    ..
                } => {
                    down[5] = false;
                    break;
                }

                Event::KeyDown {keycode: Some(Keycode::Escape), .. } | Event::Quit { .. } => {
                    break 'main;
                }

                /*Event::Quit {..} => break 'main,*/
                _ => {},
            }
            break;
            }

            //mouse_x_old = mouse_x;
            //mouse_y_old = mouse_y;
            //mouse_x = event_pump.mouse_state().x();
            //mouse_y = event_pump.mouse_state().y();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader_program.set_used();
            
            for i in 0..3 {
                if down[2*i] && !down[2*i + 1] {
                    eye[i] += speed;
                } else if !down[2*i] && down[2*i + 1] {
                    eye[i] -= speed;
                }
            }

            if down[6] {
                angle -= speed / (2. * 3.14);
            } else if down[7] {
                angle += speed / (2. * 3.14);
            }

            //theta += (mouse_x_old - mouse_x) as f32 / window_h as f32;
            //phi += (mouse_y_old - mouse_y) as f32 / window_h as f32;
            //theta += ((mouse_x) as f32 / window_w as f32 - 0.5) * 2.;
            //phi += ((mouse_y) as f32 / window_h as f32 - 0.5) * 2.;

            if down[8] {
                theta -= speed / (2. * 3.14);
            } else if down[9] {
                theta += speed / (2. * 3.14);
            }
            
            if down[10] {
                phi += speed / (2. * 3.14);
            } else if down[11] {
                phi -= speed / (2. * 3.14);
            }
            
            //println!("{}, {}", theta, phi);

            unsafe {
                gl::ProgramUniform3f(
                    shader_program.id(),
                    eye_handle,
                    eye[0],
                    eye[1],
                    eye[2]
                );        
            }

            unsafe {
                gl::ProgramUniform3f(
                    shader_program.id(),
                    direction_handle,
                    theta.sin(),
                    phi.sin(),
                    theta.cos() * phi.cos()
                )
            }

            unsafe {
                gl::ProgramUniform1f(
                    shader_program.id(),
                    angle_handle,
                    angle
                )
            }

            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // start index
                    6 // # indices to be rendered
                )
            }
            
            window.gl_swap_window();

            //::std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}