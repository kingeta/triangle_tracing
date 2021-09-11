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

fn next_power_of_2(x: i32) -> i32 {
    let mut y = x;
    y -= 1;
    y |= y >> 1;
    y |= y >> 2;
    y |= y >> 4;
    y |= y >> 8;
    y |= y >> 16;
    y += 1;
    y

}

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
        //.resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Bind and create shaders
    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("shaders/quad2.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("shaders/tex.frag")).unwrap()
    ).unwrap();

    let comp_shader = render_gl::Shader::from_comp_source(
        &CString::new(include_str!("shaders/pt.comp")).unwrap()
    ).unwrap();


    let quad_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    let comp_program = render_gl::Program::from_shaders(
        &[comp_shader]
    ).unwrap();


    quad_program.set_used();

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

    let tex_handle;
    unsafe {
        tex_handle = gl::GetUniformLocation(
            quad_program.id(),
            "tex".as_ptr() as *const gl::types::GLchar
        );
        gl::Uniform1i(tex_handle, 0);
    }

    comp_program.set_used();
    let mut work_group_size: [i32; 3] = [0; 3];
    unsafe {
        //let work_group_size: Vec::<isize>;
        gl::GetProgramiv(
            comp_program.id(),
            gl::COMPUTE_WORK_GROUP_SIZE,
            &mut work_group_size as *mut gl::types::GLint
        );
        //println!("{:?}", work_group_size);
    }


    let mut tex_id: gl::types::GLuint = 0;
    unsafe {
        gl::GenTextures(1, &mut tex_id);
        gl::BindTexture(gl::TEXTURE_2D, tex_id);
        gl::TexStorage2D(gl::TEXTURE_2D, 1, gl::RGBA32F, window_w, window_h);

        //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F);
        gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, window_w, window_h, gl::RGBA32F, gl::FLOAT, vec![0.; (window_w*window_h) as usize].as_ptr() as *const std::ffi::c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);
        gl::SamplerParameteri(tex_id, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::SamplerParameteri(tex_id, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }


    // Setup some random opengl stuff
    unsafe {
        gl::Viewport(0, 0, window_w, window_h);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    /* !!!!

    // Uniforms
    let viewport_handle: gl::types::GLint;
    unsafe {
        let viewport_name = &CString::new("viewport").unwrap();
        viewport_handle = gl::GetUniformLocation(
            quad_program.id(),
            viewport_name.as_ptr() as *const gl::types::GLchar
        );

        gl::ProgramUniform4i(
            quad_program.id(),
            viewport_handle,
            0,
            0,
            window_w,
            window_h
        );
    }

    !!!! */

    let mut position: cgmath::Vector3<f32> = 3. * cgmath::Vector3::<f32>::unit_z();
    let position_uniform = render_gl::Uniform::new("origin", comp_program.id()).unwrap();

    let mut direction: cgmath::Vector3<f32>; // = -cgmath::Vector3::<f32>::unit_z();
    let direction_uniform = render_gl::Uniform::new("forward", comp_program.id()).unwrap();
    let mut horizontal_angle: f32 = 0.;
    let mut vertical_angle: f32 = 0.;
    
    let up: cgmath::Vector3<f32> = cgmath::Vector3::<f32>::unit_y();
    let up_uniform = render_gl::Uniform::new("up", comp_program.id()).unwrap();


    let focus_dist_uniform = render_gl::Uniform::new("focus_dist", comp_program.id()).unwrap();
    let mut focus_dist: f32 = 2.;

    let focus_radius_uniform = render_gl::Uniform::new("focus_radius", comp_program.id()).unwrap();
    let mut focus_radius: f32 = 0.;

    let mut canvas_side;
    let mut canvas_up;

    let speed = 0.005; // in units per millisecond
    let mut time: usize = 0;
    let mut new_time: usize;
    let mut current_time: usize = 0;
    let mut frame_time: usize;
    let frame_uniform = render_gl::Uniform::new("frame", comp_program.id()).unwrap();
    

    let keys_list = vec![Keycode::W, Keycode::A, Keycode::S, Keycode::D, Keycode::Space, Keycode::C, Keycode::Up, Keycode::Down, Keycode::Left, Keycode::Right];
    let mut keys_down: HashSet<Keycode> = HashSet::new();
    let mut focus = false;


    let timer = sld_context.timer().unwrap();
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
                	}
                }

                Event::MouseWheel { direction: dir, .. } => {
                    focus_dist += dir.to_ll() as f32 / 10.;
                }

                /* !!! Event::Window { win_event, .. } => {
                    if let sdl2::event::WindowEvent::Resized(new_w, new_h) = win_event {
                        window_w = new_w;
                        window_h = new_h;
                        unsafe {
                            gl::Viewport(0, 0, window_w, window_h);
                            
                            gl::ProgramUniform4i(
                                quad_program.id(),
                                viewport_handle,
                                0,
                                0,
                                window_w,
                                window_h
                            );
                        }
                    }
                } !!! */

                _ => {break},
            }}
            

            direction = cgmath::vec3(horizontal_angle.sin(), -vertical_angle.sin(), -horizontal_angle.cos() * vertical_angle.cos());
            canvas_side = up.cross(direction).normalize(); //norm cross up direction
            canvas_up = direction.cross(canvas_side);

            new_time = timer.ticks() as usize;
            frame_time = new_time - current_time;
            current_time = new_time;

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

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            comp_program.set_used();

            position_uniform.push_3f(position);
            direction_uniform.push_3f(direction);
            up_uniform.push_3f(up);
            frame_uniform.push_1ui(time as u32);
            focus_dist_uniform.push_1f(focus_dist);
            focus_radius_uniform.push_1f(focus_radius);

            unsafe {
                gl::BindImageTexture(0, tex_id, 0, gl::FALSE, 0, gl::WRITE_ONLY, gl::RGBA32F);
            }

            let work_x = next_power_of_2(window_w);
            let work_y = next_power_of_2(window_h);

            unsafe {
                gl::DispatchCompute((work_x / work_group_size[0]) as u32, (work_y / work_group_size[1]) as u32, 1);

                gl::BindImageTexture(0, 0, 0, gl::FALSE, 0, gl::READ_WRITE, gl::RGBA32F);
                gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
                gl::UseProgram(0);
            }

            quad_program.set_used();
            
            unsafe {
                gl::BindVertexArray(vao);
                gl::BindTexture(gl::TEXTURE_2D, tex_id);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // start index
                    6 // # indices to be rendered
                );

                gl::BindTexture(gl::TEXTURE_2D, 0);
                gl::UseProgram(0);
            }
            
            window.gl_swap_window();
        }
    }
}
