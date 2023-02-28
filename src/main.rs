use glium::glutin;
use glium::glutin::event::Event;
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::ControlFlow;

use glium::Surface;

mod helpers;
use helpers::Vertex;

#[derive(Copy, Clone, PartialEq, Debug)]
struct ApplicationState {
    mouse_state: glutin::event::ElementState,
    force_redraw: bool,
}

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut application_state = ApplicationState {
        mouse_state: glutin::event::ElementState::Released,
        force_redraw: false,
    };
    render(&display, application_state);

    event_loop.run(move |ev, _, control_flow| {
        // let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        // control_flow.set_wait_until(next_frame_time);
        control_flow.set_wait();

        let old_state = application_state;
        application_state = handle_event(ev, control_flow, application_state);

        if old_state != application_state || application_state.force_redraw {
            println!("re-rendering. State changed: {:?}", application_state);
            render(&display, application_state);

            application_state.force_redraw = false;
        }
    });
}

fn render(display: &glium::Display, application_state: ApplicationState) {
    let mut frame = display.draw();

    render_background(application_state, &mut frame);
    render_content(display, &mut frame);

    frame.finish().unwrap();
}

fn render_background(application_state: ApplicationState, frame: &mut glium::Frame) {
    match application_state.mouse_state {
        glutin::event::ElementState::Pressed => frame.clear_color(1.0, 1.0, 1.0, 0.5),
        glutin::event::ElementState::Released => frame.clear_color(0.0, 0.0, 0.0, 0.5),
    }
}

fn render_content(display: &glium::Display, frame: &mut glium::Frame) {
    // TODO: instead of drawing triangles. start rendering html and css thingys
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
    helpers::draw_triangle(upper_triangle.to_vec(), &display, frame);
    helpers::draw_triangle(bottom_triangle.to_vec(), &display, frame);
}

fn handle_event<T>(event: Event<T>, control_flow: &mut ControlFlow, state: ApplicationState) -> ApplicationState {
    match event {
        Event::WindowEvent { event, .. } => return handle_window_event(event, control_flow, state),
        _ => return state,
    }
}

fn handle_window_event(
    event: WindowEvent,
    control_flow: &mut ControlFlow,
    state: ApplicationState,
) -> ApplicationState {
    match event {
        WindowEvent::CloseRequested => {
            control_flow.set_exit();
            return state;
        }
        WindowEvent::MouseInput { state, .. } => return ApplicationState { mouse_state: state, force_redraw: false },

        WindowEvent::Moved { .. }
        | WindowEvent::Focused { .. }
        | WindowEvent::Resized { .. }
        | WindowEvent::ScaleFactorChanged { .. } => {
            return ApplicationState { mouse_state: state.mouse_state, force_redraw: true }
        }
        _ => {
            // println!("Unhandled Window event: {:?}", event);
            return state;
        }
    }
}
