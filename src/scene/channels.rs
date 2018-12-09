use crate::image::Image;

use super::{SceneT, ViewChange};

pub struct Channels<'a> {
    i: usize,
    images: [Image<'a>; 3],
}

impl<'a> Channels<'a> {
    pub fn new(images: [Image<'a>; 3]) -> Self {
        Self {
            i: 0,
            images,
        }
    }
}

impl ViewChange for Channels<'_> {
    fn current_view(&self) -> usize {
        self.i
    }

    fn n_views(&self) -> usize {
        6
    }

    fn set_view(&mut self, i: usize) {
        self.i = i;
    }
}

impl SceneT for Channels<'_> {
    fn toggle(&mut self) {}

    fn image(&self) -> Image {
        let (r, g, b) = match self.i {
            0 => (&self.images[1], &self.images[0], &self.images[2]),
            1 => (&self.images[2], &self.images[0], &self.images[1]),
            2 => (&self.images[0], &self.images[1], &self.images[2]),
            3 => (&self.images[2], &self.images[1], &self.images[0]),
            4 => (&self.images[0], &self.images[2], &self.images[1]),
            5 => (&self.images[1], &self.images[2], &self.images[0]),
            _ => {
                println!("Invalid channel permutation");
                return self.images[0].uscale(1.0)
            }
        };
        Image::channels(r, g, b)
    }
}
