use crate::image::Image;

use super::{SceneT, ViewChange};

pub struct Combination<'a> {
    i: usize,
    n: usize,
    image1: Image<'a>,
    image2: Image<'a>,
}

impl<'a> Combination<'a> {
    pub fn new(n: usize, image1: Image<'a>, image2: Image<'a>) -> Self {
        Self {
            i: n - 1,
            n,
            image1,
            image2,
        }
    }
}

impl ViewChange for Combination<'_> {
    fn current_view(&self) -> usize {
        self.i
    }

    fn n_views(&self) -> usize {
        self.n
    }

    fn set_view(&mut self, i: usize) {
        self.i = i;
    }
}

impl SceneT for Combination<'_> {
    fn toggle(&mut self) {}

    fn image(&self) -> Image {
        let scale = (self.i as f32 / (self.n - 1) as f32).min(0.995);
        Image::add(&self.image1.uscale(1.0 - scale), &self.image2.uscale(scale))
    }
}
