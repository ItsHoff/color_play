use std::ops::{Deref, DerefMut};
use std::path::Path;

use cgmath::Vector3;

use crate::process::Processor;
use crate::scene::Scene;

pub struct Presentation<'a> {
    i: usize,
    scenes: Vec<Scene<'a>>,
}

impl<'a> Presentation<'a> {
    pub fn new(processor: &'a Processor, dir: &Path) -> Self {
        let mut scenes = Vec::new();
        scenes.push(Scene::presentation(&processor, dir, Vector3::new(0, 2, 1)));
        scenes.push(Scene::movement(&processor, dir));
        Self {
            i: 0,
            scenes,
        }
    }

    pub fn next_scene(&mut self) {
        let i = self.i + 1;
        if i < self.scenes.len() {
            self.i = i;
        } else {
            println!("The end!");
        }
    }

    pub fn previous_scene(&mut self) {
        if self.i > 0 {
            self.i -= 1;
        } else {
            println!("The beginning!");
        }
    }
}

impl<'a> Deref for Presentation<'a> {
    type Target = Scene<'a>;

    fn deref(&self) -> &Self::Target {
        &self.scenes[self.i]
    }
}

impl<'a> DerefMut for Presentation<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scenes[self.i]
    }
}
