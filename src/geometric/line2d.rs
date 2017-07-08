extern crate raster;

use std;
use geometric::Point2D;
use geometric::Geometric2D;
use geometric::draw_line_aa;

pub struct Line2D {
    from: Point2D,
    to: Point2D
}

impl Line2D {
    //Construct a Line
    pub fn new(from: Point2D, to: Point2D) -> Line2D {
        Line2D {
            from: from,
            to: to
        }
    }
}

impl std::fmt::Display for Line2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Line from {} to {}", &self.from, &self.to)
    }
}

impl Geometric2D for Line2D {
    fn draw(&self, canvas: &mut raster::Image) {
        draw_line_aa(&self.from, &self.to, canvas);
    }
    fn draw_outline(&self, canvas: &mut raster::Image) {
        draw_line_aa(&self.from, &self.to, canvas);
    }
    fn homogenize(&mut self) {
        self.from.homogenize();
        self.to.homogenize();
    }

    fn transform(&mut self, tx: f64, ty: f64) {
        self.from.transform(tx, ty);
        self.to.transform(tx, ty);
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.from.scale(sx, sy);
        self.to.scale(sx, sy);
    }

    fn rotate(&mut self, angle: f64) {
        self.from.rotate(angle);
        self.to.rotate(angle);
    }

    fn rotate_from_point(&mut self, angle: f64, p: &Point2D) {
        self.from.rotate_from_point(angle, p);
        self.to.rotate_from_point(angle, p);
    }

    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D) {
        self.from.scale_from_point(sx, sy, p);
        self.to.scale_from_point(sx, sy, p);
    }
}
