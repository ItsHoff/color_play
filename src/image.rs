use std::rc::Rc;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector4};

use glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

use image::{GenericImage, ImageFormat};

use crate::process::Processor;

#[derive(Clone)]
pub struct Image<'a> {
    texture: Rc<Texture2d>,
    processor: &'a Processor<'a>,
}

#[allow(dead_code)]
impl<'a> Image<'a> {
    pub fn new(processor: &'a Processor<'a>, path: &Path) -> Self {
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
        let srgb = SrgbTexture2d::new(processor.display, tex_image).unwrap();
        let texture = processor.make_linear(&srgb);
        Self {
            texture: Rc::new(texture), processor
        }
    }

    pub fn r(self) -> Self {
        self.scale(1.0, 0.0, 0.0)
    }

    pub fn g(self) -> Self {
        self.scale(0.0, 1.0, 0.0)
    }

    pub fn b(self) -> Self {
        self.scale(0.0, 0.0, 1.0)
    }

    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        let diag = Vector4::new(x, y, z, 1.0);
        let mat = Matrix4::from_diagonal(diag);
        self.texture = Rc::new(self.processor.transform(&self.texture, mat));
        self
    }

    pub fn permute(mut self, x: usize, y: usize, z: usize) -> Self {
        let mut mat = Matrix4::from_value(0.0);
        mat.x[x] = 1.0;
        mat.y[y] = 1.0;
        mat.z[z] = 1.0;
        mat.w.w = 1.0;
        self.texture = Rc::new(self.processor.transform(&self.texture, mat));
        self
    }

    pub fn visualize(&self) {
        self.processor.visualize(&self.texture);
    }

    pub fn diff(i1: &Self, i2: &Self, use_abs: bool) -> Self {
        let texture = Rc::new(i1.processor.diff(&i1.texture, &i2.texture, use_abs));
        Self {
            texture,
            processor: i1.processor,
        }
    }
}
