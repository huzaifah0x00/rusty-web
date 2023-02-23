use glium::glutin;
use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;

use glium::Frame;
use glium::Surface;

mod helpers;
use helpers::Vertex;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let upper_triangle = [
        Vertex { position: [-0.95, 0.95] },
        Vertex { position: [-0.95, -0.95] },
        Vertex { position: [0.95, 0.95] },
    ];
    let bottom_triangle = [
        Vertex { position: [0.95, -0.95] },
        Vertex { position: [-0.95, -0.95] },
        Vertex { position: [0.95, 0.95] },
    ];

    event_loop.run(move |ev, _, control_flow| {
        // let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        // control_flow.set_wait_until(next_frame_time);
        control_flow.set_wait();
        handle_event(ev, control_flow);

        let mut target = display.draw();

        init_background(&mut target);
        helpers::draw_triangle(upper_triangle.to_vec(), &display, &mut target);
        helpers::draw_triangle(bottom_triangle.to_vec(), &display, &mut target);

        target.finish().unwrap();
    });
}

fn init_background(frame: &mut Frame) {
    frame.clear_color(0.0, 0.0, 0.0, 0.5);
}

fn handle_event<T>(event: Event<T>, control_flow: &mut ControlFlow) {
    match event {
        Event::WindowEvent { event, .. } => handle_window_event(event, control_flow),
        _ => return,
    }
}

fn handle_window_event(event: WindowEvent, control_flow: &mut ControlFlow) {
    match event {
        WindowEvent::CloseRequested => control_flow.set_exit(),
        _ => return,
    }
}
