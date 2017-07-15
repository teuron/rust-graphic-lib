extern crate raster;

use std::collections::LinkedList;
use std::cell::RefCell;
use std::rc::Rc;
use geometric::Geometric2D;

pub struct Renderer<'a> {
    vertices: LinkedList<&'a Rc<RefCell<Box<Geometric2D>>>>,
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

    pub fn add(&mut self, geo: &'a Rc<RefCell<Box<Geometric2D>>>) {
        self.vertices.push_front(geo);
    }
    pub fn save_as(&self, filename: String) {
        raster::save(&self.image, &*filename);
    }
    pub fn draw_outline(&mut self) {
        for v in &self.vertices {
            v.borrow_mut().draw_outline(&mut self.image);
        }
    }
    pub fn draw(&mut self) {
        for v in &self.vertices {
            v.borrow_mut().draw(&mut self.image);
        }
    }
}