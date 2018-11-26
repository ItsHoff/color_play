use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use glium::glutin::{Event, WindowEvent, dpi::LogicalSize};

mod image;
mod process;

use self::image::Image;
use self::process::Processor;

#[allow(clippy::single_match)]
fn main() {
    let width = 1000;
    let height = 800;
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(f64::from(width), f64::from(height)));
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display =
        glium::Display::new(window, context, &events_loop).expect("Failed to create display");

    let processor = Processor::new(&display, width, height);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tex1 = Image::new(&processor, &root_dir.join("images/1.jpg"));
    let tex2 = Image::new(&processor, &root_dir.join("images/2.jpg"));
    let random = Image::random(&processor);
    let black = Image::grayscale(&processor, 0.0);
    let white = Image::grayscale(&processor, 1.0);
    let res = mix_chroma_luma(&white, &tex2);

    res.save(&root_dir.join("images/saved.png"));
    loop {
        res.visualize();
        let mut quit = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => quit = true,
            _ => (),
        });
        if quit {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn mix_chroma_luma<'a>(tex1: &'a Image, tex2: &'a Image) -> Image<'a> {
    let chroma = tex1.rgb_to_xyz();
    let luma = tex2.rgb_to_xyz();
    Image::channels(&chroma, &luma, &chroma).xyz_to_rgb()
}
