use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    // Create an event loop
    let event_loop = EventLoop::new()?;

    // Create a window
    let window = WindowBuilder::new()
        .with_title("Hello, winit!")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    // Run the event loop
    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't dispatched any events.
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    // ControlFlow::Exit breaks the event loop and exits the application.
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            _ => (),
        }
    });
}
