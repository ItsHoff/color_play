use std::path::Path;

use crate::image::Image;
use crate::process::Processor;

use super::{SceneT, ViewChange};

pub struct Movement<'a> {
    background: Image<'a>,
    mask: Image<'a>,
    neg_mask: Image<'a>,
    shift: bool,
}

impl<'a> Movement<'a> {
    pub fn new(processor: &'a Processor, dir: &Path) -> Self {
        let mask = Image::new(&processor, &dir.join("pikachu.jpg"));
        let neg_mask = Image::diff(&mask, &Image::grayscale(&processor, 1.0), true);
        let background = Image::random(&processor);
        Self {
            background,
            mask,
            neg_mask,
            shift: false,
        }
    }
}

impl ViewChange for Movement<'_> {
    fn current_view(&self) -> usize {
        0
    }

    fn n_views(&self) -> usize {
        1
    }

    fn set_view(&mut self, _i: usize) {
    }
}

impl SceneT for Movement<'_> {
    fn toggle(&mut self) {
        self.shift = !self.shift;
    }

    fn image(&self) -> Image {
        let (dx, dy) = if self.shift {
            rand::random::<(f32, f32)>()
        } else {
            (0.0, 0.0)
        };
        let shifted_bg = self.background.shift(dx, dy);
        let mask_bg = Image::mul(&shifted_bg, &self.neg_mask);
        let mask_fg = Image::mul(&self.background, &self.mask);
        Image::add(&mask_bg, &mask_fg)
    }
}
