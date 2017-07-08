extern crate raster;

use std::collections::LinkedList;
use geometric::Geometric2D;

pub struct Renderer<'a> {
    vertices: LinkedList<&'a Geometric2D>,
    image: raster::Image
}

impl<'a> Renderer<'a> {
    //Construct a new Renderer
    pub fn new(height: i32, width: i32) -> Renderer<'a> {
        Renderer {
            vertices: LinkedList::new(),
            image: raster::Image::blank(height, width)
        }
    }
    pub fn save(self) {
        self.save_as("test_tmp.png".to_owned());
    }
    pub fn add<S: Geometric2D>(&mut self, geo: &'a S) {
        self.vertices.push_front(geo);
    }
    pub fn save_as(&self, filename: String) {
        raster::save(&self.image, &*filename);
    }
    pub fn draw_outline(&mut self) {
        for v in &self.vertices {
            v.draw_outline(&mut self.image);
        }
    }
    pub fn draw(&mut self) {
        for v in &self.vertices {
            v.draw(&mut self.image);
        }
    }
}