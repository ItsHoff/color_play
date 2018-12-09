use std::path::{PathBuf, Path};
use std::thread;
use std::time::Duration;

use glium::glutin::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent, dpi::LogicalSize};

mod image;
mod presentation;
mod process;
mod scene;

use self::image::Image;
use self::presentation::Presentation;
use self::process::Processor;

/// Convert u8 color to float color in range [0, 1]
pub fn srgb_to_float(c: u8) -> f32 {
    (f32::from(c) / 255.0).powf(2.2)
}

#[allow(clippy::single_match, unused_variables)]
fn main() {
    let width = 1536;
    let height = 864;
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(f64::from(width), f64::from(height)));
    let context = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display =
        glium::Display::new(window, context, &events_loop).expect("Failed to create display");
    let mut fullscreen = false;

    let processor = Processor::new(&display, width, height);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = root_dir.join("results");
    std::fs::create_dir_all(output_dir.clone()).unwrap();
    let image_dir = root_dir.join("images");
    let mut presentation = Presentation::new(&processor, &image_dir);

    loop {
        presentation.image().visualize();
        let mut quit = false;
        events_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => quit = true,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => match input {
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Space),
                    ..
                } => presentation.toggle(),
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Up),
                    ..
                } => presentation.previous_view(),
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Down),
                    ..
                } => presentation.next_view(),
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => presentation.next_scene(),
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => presentation.previous_scene(),
                KeyboardInput {
                    state: ElementState::Released,
                    virtual_keycode: Some(VirtualKeyCode::F),
                    ..
                } => {
                    let window = display.gl_window();
                    fullscreen = !fullscreen;
                    if fullscreen {
                        let monitor = window.get_current_monitor();
                        window.set_fullscreen(Some(monitor));
                    } else {
                        window.set_fullscreen(None);
                    }
                }
                _ => (),
            }
            _ => (),
        });
        if quit {
            return;
        }
        thread::sleep(Duration::from_millis(100));
    }
}

#[allow(dead_code)]
fn mix_chroma_luma<'a>(tex1: &'a Image, tex2: &'a Image) -> Image<'a> {
    let chroma = tex1.rgb_to_xyz();
    let luma = tex2.rgb_to_xyz();
    Image::channels(&chroma, &luma, &chroma).xyz_to_rgb()
}

#[allow(dead_code)]
fn luma_random_mixes<'a>(tex: &'a Image, dir: &Path) {
    let random = Image::random(tex.processor).rgb_to_xyz();
    let luma = tex.rgb_to_xyz();
    Image::channels(&random, &random, &luma).xyz_to_rgb().save(&dir.join("random_xy.png"));
    Image::channels(&luma, &random, &random).xyz_to_rgb().save(&dir.join("random_yz.png"));
    Image::channels(&random, &luma, &random).xyz_to_rgb().save(&dir.join("random_xz.png"));
    Image::channels(&luma, &random, &luma).xyz_to_rgb().save(&dir.join("random_y.png"));
}

#[allow(dead_code)]
fn pink_scale<'a>(tex: &'a Image, dir: &Path) {
    let pink = Image::monochrome(tex.processor, srgb_to_float(255), srgb_to_float(145), srgb_to_float(175));
    let scale = tex.rgb_to_xyz().single_channel(2);
    Image::mul(&pink, &scale).save(&dir.join("pink_scale.png"));
}
