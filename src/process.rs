use std::collections::HashMap;

use glium::texture::{SrgbTexture2d, MipmapsOption, Texture2d, UncompressedFloatFormat};
use glium::{implement_vertex, uniform, DrawParameters, IndexBuffer, Surface, VertexBuffer};
use glium::backend::glutin::Display;

#[derive(Clone, Copy)]
struct Vertex {
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, tex_coords);

macro_rules! get_shader {
    ($self:ident, $name:ident) => {
        {
            let key = stringify!($name).to_string();
            if let Some(shader) = $self.shaders.get(&key) {
                shader
            } else {
                let vertex_shader_src = include_str!("shaders/passthrough.vert");
                let fragment_shader_src = include_str!(concat!("shaders/", stringify!($name), ".frag"));
                let shader = glium::Program::from_source(
                    $self.display,
                    vertex_shader_src,
                    fragment_shader_src,
                    None,
                ).unwrap();
                $self.shaders.insert(key.clone(), shader);
                &$self.shaders[&key]
            }
        }
    };
}

pub struct Processor<'a> {
    width: u32,
    height: u32,
    display: &'a Display,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u32>,
    shaders: HashMap<String, glium::Program>,
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
            shaders: HashMap::new(),
        }
    }
    pub fn scale(&mut self, texture: &Texture2d, scale: [f32; 3]) -> Texture2d {
        let shader = get_shader!(self, scale);
        let uniforms = uniform! {
            image: texture,
            scale: scale,
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
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                shader,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        output
    }

    pub fn permute(&mut self, texture: &Texture2d, permutation: [u32; 3]) -> Texture2d {
        let shader = get_shader!(self, permute);
        let uniforms = uniform! {
            image: texture,
            permutation: permutation,
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
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                shader,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        output
    }

    pub fn channels(&mut self, r: &Texture2d, g: &Texture2d, b: &Texture2d) -> Texture2d {
        // Simple passthrough works with proper texture types
        let shader = get_shader!(self, channels);
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
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                shader,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        output
    }

    pub fn make_linear(&mut self, texture: &SrgbTexture2d) -> Texture2d {
        // Simple passthrough works with proper texture types
        let shader = get_shader!(self, visualize);
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
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                shader,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        output
    }

    pub fn visualize(&mut self, texture: &Texture2d) {
        let shader = get_shader!(self, visualize);
        let uniforms = uniform! {
            image: texture,
        };
        let draw_parameters = DrawParameters {
            ..Default::default()
        };
        let mut target = self.display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                shader,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        target.finish().unwrap();
    }
}
