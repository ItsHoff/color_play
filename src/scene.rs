use std::ops::{Deref, DerefMut};
use std::path::Path;

use cgmath::Vector3;

use crate::image::Image;
use crate::process::Processor;

mod channels;
mod combination;
mod movement;
mod permutation;
mod plain;

use self::channels::Channels;
use self::combination::Combination;
use self::movement::Movement;
use self::permutation::Permutation;
use self::plain::Plain;

pub trait ViewChange {
    fn current_view(&self) -> usize;

    fn n_views(&self) -> usize;

    fn set_view(&mut self, i: usize);
}

pub trait SceneT: ViewChange {
    fn image(&self) -> Image;

    fn toggle(&mut self);

    fn next_view(&mut self) {
        let i = self.current_view() + 1;
        if i < self.n_views() {
            self.set_view(i);
            println!("View: {}", i);
        }
    }

    fn previous_view(&mut self) {
        if self.current_view() > 0 {
            let i = self.current_view() - 1;
            self.set_view(i);
            println!("View: {}", i);
        }
    }
}

pub enum Scene<'a> {
    Channels(Channels<'a>),
    Combination(Combination<'a>),
    Movement(Movement<'a>),
    Permutation(Permutation<'a>),
    Plain(Plain<'a>),
}

impl<'a> Scene<'a> {
    pub fn channels(images: [Image<'a>; 3]) -> Self {
        Scene::Channels(Channels::new(images))
    }

    pub fn combination(n: usize, image1: Image<'a>, image2: Image<'a>) -> Self {
        Scene::Combination(Combination::new(n, image1, image2))
    }

    pub fn movement(processor: &'a Processor, dir: &Path) -> Self {
        Scene::Movement(Movement::new(processor, dir))
    }

    pub fn permutation(images: Vec<Image<'a>>, permutation: Vector3<usize>) -> Self {
        Scene::Permutation(Permutation::new(images, permutation))
    }

    pub fn plain(image: Image<'a>) -> Self {
        Scene::Plain(Plain::new(image))
    }
}

impl<'a> Deref for Scene<'a> {
    type Target = dyn SceneT + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            Scene::Channels(inner) => inner,
            Scene::Combination(inner) => inner,
            Scene::Movement(inner) => inner,
            Scene::Permutation(inner) => inner,
            Scene::Plain(inner) => inner,
        }
    }
}

impl<'a> DerefMut for Scene<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Scene::Channels(inner) => inner,
            Scene::Combination(inner) => inner,
            Scene::Movement(inner) => inner,
            Scene::Permutation(inner) => inner,
            Scene::Plain(inner) => inner,
        }
    }
}
