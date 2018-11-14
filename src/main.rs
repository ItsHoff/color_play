use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use cgmath::{Vector3, Matrix4};

use glium::backend::Facade;
use glium::glutin::{Event, WindowEvent, dpi::LogicalSize};
use glium::texture::{RawImage2d, SrgbTexture2d};

use image::{GenericImage, ImageFormat};

mod process;

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

    let mut processor = Processor::new(&display, width, height);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let tex1 = load_image(&display, &root_dir.join("images/1.jpg"));
    let tex1 = processor.make_linear(&tex1);
    let tex2 = load_image(&display, &root_dir.join("images/2.jpg"));
    let tex2 = processor.make_linear(&tex2);
    let tex = processor.permute(&tex1, Vector3::new(2, 0, 1));

    loop {
        processor.visualize(&tex);
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

/// Load an image from path
fn load_image<F: Facade>(facade: &F, path: &Path) -> SrgbTexture2d {
    let image_format = match path.extension().unwrap().to_str().unwrap() {
        "png" => ImageFormat::PNG,
        "jpg" | "jpeg" => ImageFormat::JPEG,
        "gif" => ImageFormat::GIF,
        "webp" => ImageFormat::WEBP,
        "pnm" => ImageFormat::PNM,
        "tiff" => ImageFormat::TIFF,
        "tga" => ImageFormat::TGA,
        "bmp" => ImageFormat::BMP,
        "ico" => ImageFormat::ICO,
        "hdr" => ImageFormat::HDR,
        ext => {
            panic!("Unknown image extension {}", ext);
        }
    };
    let reader = BufReader::new(File::open(path).unwrap());
    let image = image::load(reader, image_format).unwrap();
    let image_dim = image.dimensions();
    let tex_image = RawImage2d::from_raw_rgb_reversed(&image.to_rgb().into_raw(), image_dim);
    SrgbTexture2d::new(facade, tex_image).unwrap()
}
