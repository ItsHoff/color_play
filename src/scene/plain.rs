use crate::image::Image;

use super::{SceneT, ViewChange};

pub struct Plain<'a> {
    image: Image<'a>,
}

impl<'a> Plain<'a> {
    pub fn new(image: Image<'a>) -> Self {
        Self {
            image
        }
    }
}

impl ViewChange for Plain<'_> {
    fn current_view(&self) -> usize {
        0
    }

    fn n_views(&self) -> usize {
        1
    }

    fn set_view(&mut self, _i: usize) {}
}

impl SceneT for Plain<'_> {
    fn toggle(&mut self) {}

    fn image(&self) -> Image {
        self.image.uscale(1.0)
    }
}
