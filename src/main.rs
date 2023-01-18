// use std::{
//     io::{Read, Write},
//     net::TcpStream,
// };

use winit::window::{UserAttentionType, Window};
use winit::{event::Event, event_loop::EventLoop};
use raw_window_handle::HasRawDisplayHandle;

fn main() {
    println!("Hello, world!");

    // let mut stream = TcpStream::connect("127.0.0.1:8000").expect("Something went wrong");

    // stream.write(b"GET /\n\n\n").ok();

    // let mut response = String::from("");
    // for byte in stream.bytes() {
    //     let char = byte.unwrap();
    //     response.push(char::from(char));
    // }

    // println!("{}", response);

    let event_loop: EventLoop<()> = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    let handle = window.raw_display_handle();
    println!("Handle: {:?}", handle);

    event_loop.run(move |event, _, control_flow| {
        println!("event: {:?}", event);
        control_flow.set_wait();

        match event {
            Event::MainEventsCleared => {
                // Application update code.

                window.request_user_attention(Some(UserAttentionType::Critical));

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw, in
                // applications which do not always need to. Applications that redraw continuously
                // can just render here instead.
                window.request_redraw();
            }
            Event::NewEvents(w) => println!("We got some neweevents, {:?}", w),
            Event::WindowEvent { window_id, event } => println!("Something happened"),
            Event::DeviceEvent { device_id, event } => println!("Some device thing"),
            Event::UserEvent(_) => todo!(),
            Event::Suspended => todo!(),
            Event::Resumed => println!("We're resumed"),
            Event::RedrawRequested(s) => println!("Redraw is reuequested"),
            Event::RedrawEventsCleared => println!("Redraw is cleared"),
            Event::LoopDestroyed => todo!(),
        }
    });

    println!("Bye workdl");
}
