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
        images.push(Image::new(processor, &dir.join("nature.png")));
        images.push(Image::new(processor, &dir.join("urban.png")));
        images.push(Image::new(processor, &dir.join("people.jpg")));

        let mut scenes = Vec::new();
        // Intro images
        scenes.push((Scene::plain(Image::rgb(processor)), false));
        scenes.push((Scene::plain(Image::new(processor, &dir.join("xyz.png"))), false));
        // scenes.push((Scene::plain(Image::new(processor, &dir.join("rgb.png"))), false));
        scenes.push((Scene::plain(Image::new(processor, &dir.join("triangle.png"))), false));
        // scenes.push((Scene::plain(Image::gamma(processor)), false));

        // Permutations
        scenes.push((Scene::permutation(images.clone(), Vector3::new(0, 1, 2)), false));
        scenes.push((Scene::permutation(images.clone(), Vector3::new(0, 2, 1)), true));
        scenes.push((Scene::permutation(images.clone(), Vector3::new(2, 1, 0)), true));
        scenes.push((Scene::permutation(images.clone(), Vector3::new(1, 0, 2)), true));

        // Channels
        scenes.push((Scene::channels([images[0].clone(),
                                     images[1].clone(),
                                     images[2].clone()]), false));

        // Combinations
        let hidden = Image::new(&processor, &dir.join("sibelius.jpg"));
        let n = 21;
        scenes.push((Scene::combination(n, hidden.clone(), Image::random(processor)), false));
        scenes.push((Scene::combination(n, hidden.clone(), Image::grayscale(processor, 1.0)), true));
        scenes.push((Scene::combination(n, hidden.clone(), Image::grayscale(processor, 0.0)), true));

        // Movement
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
            println!("Scene: {}", self.i);
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
            println!("Scene: {}", self.i);
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
