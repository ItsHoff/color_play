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

    let processor = Processor::new(&display, width, height);
    let root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let output_dir = root_dir.join("results");
    std::fs::create_dir_all(output_dir.clone()).unwrap();
    let image_dir = root_dir.join("images");

    // let tex = Image::new(&processor, &image_dir.join("2.jpg"));
    // let backgrounds = vec!(
    //     Image::random(&processor),
    //     Image::grayscale(&processor, 1.0),
    //     Image::grayscale(&processor, 0.0),
    //     Image::new(&processor, &image_dir.join("2.jpg")),
    // );

    // let mut shift = true;
    // let mut scale = 0.0;
    // let mut background = &backgrounds[0];
    // let mut mode = 7;
    let mut presentation = Presentation::new(&processor, &image_dir);
    loop {
        // let res = if mode == 0 {
        // } else if mode == 1 {
        //     let shifted_bg = background.shift(dx, dy);
        //     let r = Image::add(&shifted_bg.r().uscale(1.0 - scale), &tex.r().uscale(scale));
        //     Image::channels(&r, &shifted_bg, &shifted_bg)
        // } else if mode == 2 {
        //     let shifted_bg = background.shift(dx, dy);
        //     let g = Image::add(&shifted_bg.g().uscale(1.0 - scale), &tex.g().uscale(scale));
        //     Image::channels(&shifted_bg, &g, &shifted_bg)
        // } else if mode == 3 {
        //     let shifted_bg = background.shift(dx, dy);
        //     let b = Image::add(&shifted_bg.b().uscale(1.0 - scale), &tex.b().uscale(scale));
        //     Image::channels(&shifted_bg, &shifted_bg, &b)
        // } else if mode == 4 {
        //     let shifted_bg = background.shift(dx, dy).rgb_to_xyz();
        //     let xyz = tex.rgb_to_xyz();
        //     let x = Image::add(&shifted_bg.x().uscale(1.0 - scale), &xyz.x().uscale(scale));
        //     Image::channels(&x, &shifted_bg, &shifted_bg).xyz_to_rgb()
        // } else if mode == 5 {
        //     let shifted_bg = background.shift(dx, dy).rgb_to_xyz();
        //     let xyz = tex.rgb_to_xyz();
        //     let y = Image::add(&shifted_bg.y().uscale(1.0 - scale), &xyz.y().uscale(scale));
        //     Image::diff(&Image::channels(&shifted_bg, &y, &shifted_bg).xyz_to_rgb(),
        //                 &Image::grayscale(&processor, 0.0), true)
        // } else if mode == 6 {
        //     let shifted_bg = background.shift(dx, dy).rgb_to_xyz();
        //     let xyz = tex.rgb_to_xyz();
        //     let z = Image::add(&shifted_bg.z().uscale(1.0 - scale), &xyz.z().uscale(scale));
        //     Image::channels(&shifted_bg, &shifted_bg, &z).xyz_to_rgb()
        // } else if mode == 7 {
        //     tex.permute(1, 0, 2)
        // } else if mode == 8 {
        //     tex.permute(0, 2, 1)
        // } else if mode == 9 {
        //     tex.permute(2, 1, 0)
        // } else if mode == 10 {
        //     tex.permute(1, 2, 0)
        // } else {
        //     tex.rgb_to_xyz().xyz_to_rgb()
        // };
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
