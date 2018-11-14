use std::collections::HashMap;
use std::cell::RefCell;

use cgmath::conv::*;
use cgmath::Matrix4;

use glium::texture::{SrgbTexture2d, MipmapsOption, Texture2d, UncompressedFloatFormat};
use glium::{implement_vertex, uniform, DrawParameters, IndexBuffer, Surface, VertexBuffer};
use glium::backend::glutin::Display;

#[derive(Clone, Copy)]
struct Vertex {
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, tex_coords);

pub struct Processor<'a> {
    width: u32,
    height: u32,
    pub display: &'a Display,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    shaders: RefCell<HashMap<String, glium::Program>>,
}

macro_rules! draw_with_shader {
    ($shader_name:ident, $self:ident, $target:ident, $uniforms: expr, $draw_parameters: expr) => {
        {
            let key = stringify!($shader_name).to_string();
            let mut shaders = $self.shaders.borrow_mut();
            let shader = if let Some(shader) = shaders.get(&key) {
                shader
            } else {
                let vertex_shader_src = include_str!("shaders/passthrough.vert");
                let fragment_shader_src = include_str!(
                    concat!("shaders/", stringify!($shader_name), ".frag")
                );
                let shader = glium::Program::from_source(
                    $self.display,
                    vertex_shader_src,
                    fragment_shader_src,
                    None,
                ).unwrap();
                shaders.insert(key.clone(), shader);
                &shaders[&key]
            };
            $target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
            $target
                .draw(
                    &$self.vertex_buffer,
                    &$self.index_buffer,
                    shader,
                    $uniforms,
                    $draw_parameters,
                )
                .unwrap();
        }
    };
}

#[allow(dead_code)]
impl<'a> Processor<'a> {
    pub fn new(display: &'a Display, width: u32, height: u32) -> Self {
        let vertices = vec![
            Vertex {
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                tex_coords: [0.0, 1.0],
            },
        ];
        let vertex_buffer =
            VertexBuffer::new(display, &vertices).expect("Failed to create vertex buffer!");
        let indices = vec![0, 1, 2, 0, 2, 3];
        let index_buffer =
            IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices)
                .expect("Failed to create index buffer!");

        Self {
            width,
            height,
            display,
            vertex_buffer,
            index_buffer,
            shaders: RefCell::new(HashMap::new()),
        }
    }

    pub fn transform(&self, texture: &Texture2d, transform: Matrix4<f32>) -> Texture2d {
        // Simple passthrough works with proper texture types
        let uniforms = uniform! {
            image: texture,
            transform: array4x4(transform),
        };
        let draw_parameters = DrawParameters {
            ..Default::default()
        };
        let output = Texture2d::empty_with_format(
            self.display,
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
            self.width,
            self.height,
        ).unwrap();
        let mut target = output.as_surface();
        draw_with_shader!(transform, self, target, &uniforms, &draw_parameters);
        output
    }

    pub fn channels(&self, r: &Texture2d, g: &Texture2d, b: &Texture2d) -> Texture2d {
        let uniforms = uniform! {
            r: r,
            g: g,
            b: b,
        };
        let draw_parameters = DrawParameters {
            ..Default::default()
        };
        let output = Texture2d::empty_with_format(
            self.display,
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
            self.width,
            self.height,
        ).unwrap();
        let mut target = output.as_surface();
        draw_with_shader!(channels, self, target, &uniforms, &draw_parameters);
        output
    }

    pub fn make_linear(&self, texture: &SrgbTexture2d) -> Texture2d {
        let uniforms = uniform! {
            image: texture,
        };
        let draw_parameters = DrawParameters {
            ..Default::default()
        };
        let output = Texture2d::empty_with_format(
            self.display,
            UncompressedFloatFormat::F32F32F32F32,
            MipmapsOption::NoMipmap,
            self.width,
            self.height,
        ).unwrap();
        let mut target = output.as_surface();
        draw_with_shader!(visualize, self, target, &uniforms, &draw_parameters);
        output
    }

    pub fn visualize(&self, texture: &Texture2d) {
        let uniforms = uniform! {
            image: texture,
        };
        let draw_parameters = DrawParameters {
            ..Default::default()
        };
        let mut target = self.display.draw();
        draw_with_shader!(visualize, self, target, &uniforms, &draw_parameters);
        target.finish().unwrap();
    }
}
