use std::rc::Rc;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use cgmath::prelude::*;
use cgmath::{Matrix4, Vector4, Vector2};

use glium::texture::{RawImage2d, SrgbTexture2d, Texture2d};

use image::{GenericImage, ImageFormat, RgbaImage};

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
        let texture = processor.srgb_to_linear(&srgb);
        Self {
            texture: Rc::new(texture), processor
        }
    }

    pub fn random(processor: &'a Processor<'a>) -> Self {
        let w = processor.width;
        let h = processor.height;
        let len = (3 * w * h) as usize;
        let mut data = Vec::with_capacity(len);
        for _ in 0..len {
            data.push(rand::random::<f32>());
        }
        let tex_image = RawImage2d::from_raw_rgb(data, (w, h));
        let texture = Texture2d::new(processor.display, tex_image).unwrap();
        Self {
            texture: Rc::new(texture), processor
        }
    }

    pub fn grayscale(processor: &'a Processor<'a>, val: f32) -> Self {
        Self::monochrome(processor, val, val, val)
    }

    pub fn monochrome(processor: &'a Processor<'a>, r: f32, g: f32, b: f32) -> Self {
        let data = vec!(r, g, b);
        let tex_image = RawImage2d::from_raw_rgb(data, (1, 1));
        let texture = Texture2d::new(processor.display, tex_image).unwrap();
        Self {
            texture: Rc::new(texture), processor
        }
    }

    pub fn r(&self) -> Self {
        self.scale(1.0, 0.0, 0.0)
    }

    pub fn g(&self) -> Self {
        self.scale(0.0, 1.0, 0.0)
    }

    pub fn b(&self) -> Self {
        self.scale(0.0, 0.0, 1.0)
    }

    pub fn x(&self) -> Self {
        self.r()
    }

    pub fn y(&self) -> Self {
        self.g()
    }

    pub fn z(&self) -> Self {
        self.b()
    }
    pub fn uscale(&self, scale: f32) -> Self {
        self.scale(scale, scale, scale)
    }

    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        let diag = Vector4::new(x, y, z, 1.0);
        let mat = Matrix4::from_diagonal(diag);
        Self {
            texture: Rc::new(self.processor.transform(&self.texture, mat)),
            processor: self.processor,
        }
    }

    pub fn shift(&self, x: f32, y: f32) -> Self {
        Self {
            texture: Rc::new(self.processor.shift(&self.texture, Vector2::new(x, y))),
            processor: self.processor,
        }
    }

    pub fn permute(&self, x: usize, y: usize, z: usize) -> Self {
        let mut mat = Matrix4::from_value(0.0);
        mat.x[x] = 1.0;
        mat.y[y] = 1.0;
        mat.z[z] = 1.0;
        mat.w.w = 1.0;
        let mat = mat.transpose();
        Self {
            texture: Rc::new(self.processor.transform(&self.texture, mat)),
            processor: self.processor,
        }
    }

    #[allow(clippy::unreadable_literal)]
    pub fn rgb_to_xyz(&self) -> Self {
        let to_xyz = Matrix4::new(
            0.412453, 0.35758, 0.180423, 0.0,
            0.212671, 0.71516, 0.072169, 0.0,
            0.019334, 0.119193, 0.950227, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();
        Self {
            texture: Rc::new(self.processor.transform(&self.texture, to_xyz)),
            processor: self.processor,
        }
    }

    #[allow(clippy::unreadable_literal)]
    pub fn xyz_to_rgb(&self) -> Self {
        let to_rgb = Matrix4::new(
            3.240479, -1.53715, -0.498535, 0.0,
            -0.969256, 1.875991, 0.041556, 0.0,
            0.055648, -0.204043, 1.057311, 0.0,
            0.0, 0.0, 0.0, 1.0
        ).transpose();
        Self {
            texture: Rc::new(self.processor.transform(&self.texture, to_rgb)),
            processor: self.processor,
        }
    }

    pub fn visualize(&self) {
        self.processor.visualize(&self.texture);
    }

    pub fn save(&self, path: &Path) {
        let srgb = self.processor.linear_to_srgb(&self.texture);
        let pb = srgb.read_to_pixel_buffer();
        let raw_image: RawImage2d<u8> = pb.read_as_texture_2d().unwrap();
        let image = RgbaImage::from_vec(self.processor.width, self.processor.height,
                                        raw_image.data.to_vec()).unwrap();
        let image = image::imageops::flip_vertical(&image);
        image.save(path).unwrap();
    }

    pub fn diff(i1: &Self, i2: &Self, use_abs: bool) -> Self {
        let texture = Rc::new(i1.processor.diff(&i1.texture, &i2.texture, use_abs));
        Self {
            texture,
            processor: i1.processor,
        }
    }

    pub fn add(i1: &Self, i2: &Self) -> Self {
        let texture = Rc::new(i1.processor.add(&i1.texture, &i2.texture));
        Self {
            texture,
            processor: i1.processor,
        }
    }

    pub fn channels(r: &Self, g: &Self, b: &Self) -> Self {
        let texture = Rc::new(r.processor.channels(&r.texture, &g.texture, &b.texture));
        Self {
            texture,
            processor: r.processor,
        }
    }
}
