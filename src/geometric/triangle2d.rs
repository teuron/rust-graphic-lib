extern crate raster;

use std;
use geometric::{Geometric2D, Point2D, Line2D, interpolate_barycentric};
use raster::Color;

/// Represents a 2D Triangle
#[derive(Debug)]
pub struct Triangle2D {
    /// Point of Triangle
    a: Point2D,
    /// Point of Triangle
    b: Point2D,
    /// Point of Triangle
    c: Point2D
}

impl Triangle2D {
    /// Returns a colored Triangle
    ///
    /// # Arguments
    ///
    /// * `a` - Point of a Triangle
    /// * `b` - Point of a Triangle
    /// * `c` - Point of a Triangle
    ///
    /// # Example
    ///
    /// ```
    /// extern crate graphic_library;
    /// use graphic_library::geometric::{Triangle2D, Point2D};
    /// fn main(){
    /// //Creates a colored Triangle
    /// let triangle = Triangle2D::new(Point2D::new(0.0, 0.0), Point2D::new(5.0, 0.0), Point2D::new(2.5, 5.0));
    /// }
    /// ```
    pub fn new(a: Point2D, b: Point2D, c: Point2D) -> Triangle2D {
        Triangle2D {
            a: a,
            b: b,
            c: c
        }
    }
}

impl std::fmt::Display for Triangle2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Triangle a:{}, b:{}, c:{}", self.a, self.b, self.c)
    }
}

impl Geometric2D for Triangle2D {
    fn draw(&self, canvas: &mut raster::Image) {
        let a = (self.c.x - self.a.x) * (self.b.y - self.a.y) - (self.b.x - self.a.x) * (self.c.y - self.a.y);
        let x1 = self.a.x;
        let y1 = self.a.y;
        let mut x2 = self.b.x;
        let mut y2 = self.b.y;
        let mut x3 = self.c.x;
        let mut y3 = self.c.y;
        if a.abs() < 1e-10 {
            return;
        }
        if a < 0.0 {
            let mut t = x2;
            x2 = x3;
            x3 = t;
            t = y2;
            y2 = y3;
            y3 = t;
        }
        let a_1 = y2 - y3;
        let b_1 = x3 - x2;
        let a_2 = y3 - y1;
        let b_2 = x1 - x3;
        let a_3 = y1 - y2;
        let b_3 = x2 - x1;
        let c_1 = -(a_1 * x2 + b_1 * y2);
        let c_2 = -(a_2 * x3 + b_2 * y3);
        let c_3 = -(a_3 * x1 + b_3 * y1);
        let prp_alpha = 1.0f64 / (a_1 * x1 + b_1 * y1 + c_1);
        let prp_beta = 1.0f64 / (a_2 * x2 + b_2 * y2 + c_2);
        let prp_gamma = 1.0f64 / (a_3 * x3 + b_3 * y3 + c_3);
        let x_min = x1.min(x2.min(x3)) as i32;
        let x_max = x1.max(x2.max(x3)) as i32;
        let y_min = y1.min(y2.min(y3)) as i32;
        let y_max = y1.max(y2.max(y3)) as i32;
        for xs in x_min..x_max {
            for ys in y_min..y_max {
                let x = xs as f64;
                let y = ys as f64;
                if (a_1 * x + b_1 * y + c_1) <= 0.0 && (a_2 * x + b_2 * y + c_2) <= 0.0 && (a_3 * x + b_3 * y + c_3) <= 0.0 {
                    let alpha = (a_1 * x + b_1 * y + c_1) * prp_alpha;
                    let beta = (a_2 * x + b_2 * y + c_2) * prp_beta;
                    let gamma = (a_3 * x + b_3 * y + c_3) * prp_gamma;
                    let r = interpolate_barycentric(self.a.get_color().r as f64, self.b.get_color().r as f64, self.c.get_color().r as f64, alpha, beta, gamma);
                    let g = interpolate_barycentric(self.a.get_color().g as f64, self.b.get_color().g as f64, self.c.get_color().g as f64, alpha, beta, gamma);
                    let b = interpolate_barycentric(self.a.get_color().b as f64, self.b.get_color().b as f64, self.c.get_color().b as f64, alpha, beta, gamma);
                    canvas.set_pixel(xs, ys, Color::rgb(r as u8, g as u8, b as u8)).unwrap();
                }
            }
        }
    }

    fn draw_outline(&self, canvas: &mut raster::Image) {
        let line1 = Line2D::new(self.a.clone(), self.b.clone());
        line1.draw_outline(canvas);
        let line2 = Line2D::new(self.b.clone(), self.c.clone());
        line2.draw_outline(canvas);
        let line3 = Line2D::new(self.c.clone(), self.a.clone());
        line3.draw_outline(canvas);
    }

    fn homogenize(&mut self) {
        self.a.homogenize();
        self.b.homogenize();
        self.c.homogenize();
    }

    fn transform(&mut self, tx: f64, ty: f64) {
        self.a.transform(tx, ty);
        self.b.transform(tx, ty);
        self.c.transform(tx, ty);
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.a.scale(sx, sy);
        self.b.scale(sx, sy);
        self.c.scale(sx, sy);
    }

    fn rotate(&mut self, angle: f64) {
        self.a.rotate(angle);
        self.b.rotate(angle);
        self.c.rotate(angle);
    }

    fn rotate_from_point(&mut self, angle: f64, p: &Point2D) {
        self.a.rotate_from_point(angle, p);
        self.b.rotate_from_point(angle, p);
        self.c.rotate_from_point(angle, p);
    }

    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D) {
        self.a.scale_from_point(sx, sy, p);
        self.b.scale_from_point(sx, sy, p);
        self.c.scale_from_point(sx, sy, p);
    }

    fn draw_aa(&self, canvas: &mut raster::Image) {
        self.draw_outline_aa(canvas);
        self.draw(canvas);
    }

    fn draw_outline_aa(&self, canvas: &mut raster::Image) {
        let line1 = Line2D::new(self.a.clone(), self.b.clone());
        line1.draw_outline_aa(canvas);
        let line2 = Line2D::new(self.b.clone(), self.c.clone());
        line2.draw_outline_aa(canvas);
        let line3 = Line2D::new(self.c.clone(), self.a.clone());
        line3.draw_outline_aa(canvas);
    }
}