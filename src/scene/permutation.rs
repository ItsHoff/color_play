use std::path::Path;

use cgmath::Vector3;

use crate::image::Image;
use crate::process::Processor;

use super::{SceneT, ViewChange};

pub struct Permutation<'a> {
    i: usize,
    views: Vec<Image<'a>>,
    permutation: Vector3<usize>,
}

impl<'a> Permutation<'a> {
    pub fn new(processor: &'a Processor, dir: &Path, permutation: Vector3<usize>) -> Self {
        let mut views = Vec::new();
        views.push(Image::new(&processor, &dir.join("1.jpg")));
        views.push(Image::new(&processor, &dir.join("2.jpg")));
        Self {
            i: 0,
            views,
            permutation,
        }
    }
}

impl ViewChange for Permutation<'_> {
    fn current_view(&self) -> usize {
        self.i
    }

    fn n_views(&self) -> usize {
        self.views.len()
    }

    fn set_view(&mut self, i: usize) {
        self.i = i;
    }
}

impl SceneT for Permutation<'_> {
    fn toggle(&mut self) {}

    fn image(&self) -> Image {
        let tex = &self.views[self.i];
        tex.permute(
            self.permutation.x,
            self.permutation.y,
            self.permutation.z
        )
    }
}
