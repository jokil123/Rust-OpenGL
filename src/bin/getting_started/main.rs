use crate::mesh::triangle::{self, TRIANGLE};

#[macro_use]
extern crate glium;

mod mesh;

fn main() {
    use glium::{glutin, Surface};

    // event loop handles window and device events
    let mut event_loop = glutin::event_loop::EventLoop::new();
    // window specific attributes
    let wb = glutin::window::WindowBuilder::new();
    // OpenGL context attributes (Multisampling, vsync, etc)
    let cb = glutin::ContextBuilder::new();
    // OpenGL window
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &TRIANGLE).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        uniform float x;
        uniform float y;

        void main() {
            vec2 pos = position;
            pos.x += x;
            pos.y += y;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.5;
    let mut v_x: f32 = 0.1;
    let mut v_y: f32 = 0.1;

    // this closure is run every frame
    event_loop.run(move |ev, _, control_flow| {
        y += v_y;
        x += v_x;
        v_y += -0.0001;

        if -0.5 > x || x > 0.5 {
            v_x *= -0.9;
            x = x.min(0.5).max(-0.5);
        }

        if -0.5 > y || y > 0.5 {
            v_y *= -0.9;
            y = y.min(0.5).max(-0.5);
        }

        // The actual drawing code goes here:
        // =====================

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        // draw call for the triangle
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform! { x: x, y: y },
                &Default::default(),
            )
            .unwrap();

        // finishes the buffer
        target.finish().unwrap();

        // =====================

        // calculate the time for the next frame
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        // pauses the event loop until the next frame should be rendered
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // if the event is a close event, stop the event loop
                glutin::event::WindowEvent::CloseRequested => {
                    // event loop is stopped
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}
