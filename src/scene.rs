use std::ops::{Deref, DerefMut};
use std::path::Path;

use cgmath::Vector3;

use crate::image::Image;
use crate::process::Processor;

mod movement;
mod permutation;

use self::movement::Movement;
use self::permutation::Permutation;

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
        }
    }

    fn previous_view(&mut self) {
        if self.current_view() > 0 {
            let i = self.current_view() - 1;
            self.set_view(i);
        }
    }
}

pub enum Scene<'a> {
    Movement(Movement<'a>),
    Permutation(Permutation<'a>),
}

impl<'a> Scene<'a> {
    pub fn movement(processor: &'a Processor, dir: &Path) -> Self {
        Scene::Movement(Movement::new(processor, dir))
    }

    pub fn presentation(processor: &'a Processor, dir: &Path, permutation: Vector3<usize>) -> Self {
        Scene::Permutation(Permutation::new(processor, dir, permutation))
    }
}

impl<'a> Deref for Scene<'a> {
    type Target = dyn SceneT + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            Scene::Movement(inner) => inner,
            Scene::Permutation(inner) => inner,
        }
    }
}

impl<'a> DerefMut for Scene<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Scene::Movement(inner) => inner,
            Scene::Permutation(inner) => inner,
        }
    }
}
