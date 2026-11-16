use wry::WebViewBuilder;
use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;

fn main() {

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("GENTWO IDE").build(&event_loop).unwrap();
    let webview = WebViewBuilder::new().with_html(include_str!("./index.html")).build(&window).unwrap();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}
