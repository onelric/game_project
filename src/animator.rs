use std::{collections::HashMap, fs::File, path::PathBuf};

use serde_json::Value;

use crate::prelude::*;

pub struct Animation {
    frames: Vec<Texture>,
    data: Value,
    meta_data: Value,
    index: usize,
    duration: f64,
    flip: bool,
}

impl Animation {
    pub fn from_dir(path: &str) -> Self {
        let pathbuf = PathBuf::from(path);

        let mut png_path = vec![];
        let mut json_path = PathBuf::new();
        for entry in pathbuf.read_dir().unwrap() {
            let path = entry.unwrap().path();
            if let Some(ext) = path.extension() {
                if ext == "png" {
                    png_path.push(path)
                } else if ext == "json" {
                    json_path = path
                }
            }
        }

        let mut frames = vec![];
        for path in png_path.iter() {
            frames.push(Texture::from_path(path, &TextureSettings::new().filter(Filter::Nearest)).unwrap())
        }

        let data: Value = serde_json::from_reader(File::open(&json_path).unwrap()).unwrap();

        let mut metabuf = pathbuf.clone();
        metabuf.pop();

        let meta_data: Value = serde_json::from_reader(File::open(metabuf.join("meta").with_extension("json")).unwrap()).unwrap();

        Self {
            duration: data["duration"].as_f64().unwrap() / 1000.0,
            frames,
            index: 0,
            data,
            meta_data,
            flip: false,
        }
    }

    pub fn render(&self, c: Context, gl: &mut GlGraphics, transform: &Transform) {
        use graphics::image;

        let size = DVec2::new(self.data["size"]["w"].as_f64().unwrap(), self.data["size"]["h"].as_f64().unwrap());

        let translation = DVec2::new(transform.translation.x - size.x / 2.0, transform.translation.y - size.y / 2.0);
        let offset = DVec2::new(
            self.meta_data["offset"]["x"].as_f64().unwrap(),
            self.meta_data["offset"]["y"].as_f64().unwrap(),
        );

        let final_translation = translation + offset;

        if self.flip {
            let transform = c.transform.trans(final_translation.x + size.x + offset.x, final_translation.y).flip_h();

            image(&self.frames[self.index], transform, gl)
        } else {
            image(&self.frames[self.index], c.transform.trans(final_translation.x, final_translation.y), gl)
        }
    }
}

pub fn update_animations(world: &mut World) {
    world.query::<&mut Animator>().into_iter().for_each(|(_, animator)| {
        let animation = animator.get_current();
        animation.duration -= get_delta_time();
        if animation.duration <= 0.0 {
            animation.duration = animation.data["duration"].as_f64().unwrap() / 1000.0;
            animation.index += 1;
            if animation.index >= animation.frames.len() {
                animation.index = 0
            }
        }
    });
}

pub struct Animator {
    current: &'static str,
    animations: HashMap<&'static str, Animation>,
    flip: bool,
}

impl Animator {
    pub fn new(animation: &'static str) -> Self {
        Self {
            current: animation,
            animations: HashMap::new(),
            flip: false,
        }
    }

    pub fn set_flip(&mut self, flip: bool) {
        self.flip = flip;
        // TODO WARNING !!!TEMP SOLUTION!!!
        for (_, v) in self.animations.iter_mut() {
            v.flip = flip
        }
    }

    pub fn add_animation(&mut self, id: &'static str, animation: Animation) {
        self.animations.insert(id, animation);
    }

    pub fn change_to(&mut self, animation: &'static str) {
        for (k, _) in self.animations.iter() {
            if animation == *k {
                self.current = animation;
            }
        }
    }

    pub fn render(&self, c: Context, gl: &mut GlGraphics, transform: &Transform) {
        if self.animations.len() > 0 {
            self.animations[self.current].render(c, gl, transform)
        }
    }

    pub fn get_current(&mut self) -> &mut Animation {
        self.animations.get_mut(self.current).unwrap()
    }
}

pub fn render_animations_system(world: &mut World, c: Context, gl: &mut GlGraphics) {
    // Render animations
    world
        .query::<(&Animator, &Transform)>()
        .iter()
        .for_each(|(_, (animator, transform))| animator.render(c, gl, transform));
}
