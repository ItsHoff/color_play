use std::ops::{Deref, DerefMut};
use std::path::Path;

use cgmath::Vector3;

use crate::image::Image;
use crate::process::Processor;
use crate::scene::Scene;

pub struct Presentation<'a> {
    i: usize,
    scenes: Vec<(Scene<'a>, bool)>,
}

impl<'a> Presentation<'a> {
    pub fn new(processor: &'a Processor, dir: &Path) -> Self {
        let mut images = Vec::new();
        images.push(Image::new(&processor, &dir.join("1.jpg")));
        images.push(Image::new(&processor, &dir.join("2.jpg")));
        images.push(Image::new(&processor, &dir.join("3.jpg")));
        let mut scenes = Vec::new();
        scenes.push((Scene::plain(Image::gamma(processor)), false));
        scenes.push((Scene::combination(20, images[0].clone(), Image::random(processor)), false));
        scenes.push((Scene::combination(20, images[0].clone(), Image::grayscale(processor, 1.0)), true));
        scenes.push((Scene::combination(20, images[0].clone(), Image::grayscale(processor, 0.0)), true));
        scenes.push((Scene::permutation(images.clone(), Vector3::new(0, 2, 1)), false));
        scenes.push((Scene::permutation(images.clone(), Vector3::new(2, 1, 0)), true));
        scenes.push((Scene::permutation(images.clone(), Vector3::new(1, 0, 2)), true));
        scenes.push((Scene::channels([images[0].clone(),
                                     images[1].clone(),
                                     images[2].clone()]), false));
        scenes.push((Scene::movement(&processor, dir), false));
        Self {
            i: 0,
            scenes,
        }
    }

    pub fn next_scene(&mut self) {
        let i = self.i + 1;
        if i < self.scenes.len() {
            let (_, keep_view) = self.scenes[i];
            if keep_view {
                let view = self.scenes[self.i].0.current_view();
                self.scenes[i].0.set_view(view);
            }
            self.i = i;
        } else {
            println!("The end!");
        }
    }

    pub fn previous_scene(&mut self) {
        if self.i > 0 {
            let i = self.i;
            self.i -= 1;
            let (_, keep_view) = self.scenes[i];
            if keep_view {
                let view = self.scenes[i].0.current_view();
                self.scenes[self.i].0.set_view(view);
            }
        } else {
            println!("The beginning!");
        }
    }
}

impl<'a> Deref for Presentation<'a> {
    type Target = Scene<'a>;

    fn deref(&self) -> &Self::Target {
        &self.scenes[self.i].0
    }
}

impl<'a> DerefMut for Presentation<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.scenes[self.i].0
    }
}
