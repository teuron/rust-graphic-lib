extern crate raster;

use std;
use std::f64::consts::PI;

use raster::Color;
use geometric::Geometric2D;
use geometric::Point2D;

#[derive(Debug)]
pub struct Circle2D {
    pub r: f64,
    pub m: Point2D
}

impl Circle2D {
    //Construct a Point(x,y)
    pub fn new(r: f64, m: Point2D) -> Circle2D {
        Circle2D { r: r, m: m }
    }
    pub fn get_color(&self) -> Color { self.m.get_color() }
}

impl std::fmt::Display for Circle2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Circle Radius:{}, Middle:{}", self.r, self.m)
    }
}

impl Geometric2D for Circle2D {
    fn homogenize(&mut self) {
        unimplemented!();
    }

    fn transform(&mut self, tx: f64, ty: f64) {
        unimplemented!();
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        unimplemented!();
    }

    fn rotate(&mut self, angle: f64) {
        unimplemented!();
    }

    fn rotate_from_point(&mut self, angle: f64, p: &Point2D) {
        unimplemented!();
    }

    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D) {
        unimplemented!();
    }

    fn draw(&self, canvas: &mut raster::Image) {
        unimplemented!();
    }

    fn draw_outline(&self, canvas: &mut raster::Image) {
        let r2 = (self.r * self.r) as i32;
        let area = r2 << 2;
        let rr = (self.r as i32) << 1;
        let r = self.r as i32;
        for i in 0..area {
            let tx = (i % rr) - r;
            let ty = (i / rr) - r;
            if tx * tx + ty * ty <= r2 {
                canvas.set_pixel(self.m.x as i32 + tx, self.m.y as i32 + ty, self.m.get_color());
            }
        }
    }
}