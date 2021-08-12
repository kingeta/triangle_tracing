fn main_() {
    let (window_w, window_h) = (800, 600);
    let sld_context = sdl2::init().unwrap();
    let video_subsystem = sld_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    
    let window = video_subsystem.window("Test", window_w, window_h)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);


    let vert_shader = render_gl::Shader::from_vert_source(
        &CString::new(include_str!("shaders/test.vert")).unwrap()
    ).unwrap();

    let frag_shader = render_gl::Shader::from_frag_source(
        &CString::new(include_str!("shaders/test.frag")).unwrap()
    ).unwrap();

    let shader_program = render_gl::Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();


    let vertices: Vec<f32> = vec![
        -0.5, -0.5, 0.0,
        0.5, -0.5, 0.0,
        0.0, 0.5, 0.0
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, //Target
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, //size (bytes) of data
            vertices.as_ptr() as *const gl::types::GLvoid, // data pointer
            gl::STATIC_DRAW, //use
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
            0, //Index of generic vertex attribute layour location = 0
            3, // # components per generic vertex attribute
            gl::FLOAT, //dtype
            gl::FALSE, //normalised
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, //byte offset between attributes i.e. stride
            std::ptr::null() //offset of first component
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }


    unsafe {
        gl::Viewport(0, 0, window_w as i32, window_h as i32);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }


    let mut event_pump = sld_context.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            shader_program.set_used();
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawArrays(
                    gl::TRIANGLES, // mode
                    0, // start index
                    3 // # indices to be rendered
                )
            }

            window.gl_swap_window();
        }
    }
}

