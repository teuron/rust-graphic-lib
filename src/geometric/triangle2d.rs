extern crate raster;

use std;
use geometric::geometric::Geometric2D;
use geometric::Point2D;
use geometric::draw_line;
use geometric::interpolate_barycentric;
use raster::Color;

pub struct Triangle2D {
    a: Point2D,
    b: Point2D,
    c: Point2D
}

impl Triangle2D {
    //Construct a Point(x,y)
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
        draw_line(&self.a, &self.b, canvas);
        draw_line(&self.b, &self.c, canvas);
        draw_line(&self.c, &self.a, canvas);
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
}