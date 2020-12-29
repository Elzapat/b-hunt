/*extern crate sfml;

use std::{collections::HashMap, hash::Hash};
use sfml::graphics::{Texture, Font};
use sfml::system::SfBox;
use sfml::system::SfResource;

pub trait LoadRes: Sized {
    fn load_from_file(filename: &str) -> SfBox<Self>;
}

impl LoadRes for Texture {
    fn load_from_file(filename: &str) -> SfBox<Self> {
        Texture::from_file(filename)
    }
}

impl LoadRes for Font {
    fn load_from_file(filename: &str) -> SfBox<Self> {
        Font::from_file(filename)
    }
}

pub struct ResourceManager<I: Hash + Eq, R: Dispose> {
    resource_map: HashMap<I, Box<R>>
}

impl<I: Hash + Eq, R: Dispose> ResourceManager<I, R> {
    pub fn new() -> Self {
        ResourceManager {
            resource_map: HashMap::<I, SfBox<R>>::new()
        }
    }

    pub fn load(&mut self, identifier: I, filename: &str) {
        let resource = R::load_from_file(filename).unwrap();
        self.resource_map.insert(identifier, SfBox::new(resource));
    }

    pub fn get(&self, identifier: I) -> &SfBox<R> {
        match self.resource_map.get(&identifier) {
            Some(resource) => resource,
            None => panic!("Tried to access non-existant index in resource map.")
        }
    }
}*/
