extern crate raster;

use std;
use std::f64::consts::PI;

use raster::Color;
use geometric::Geometric2D;
use geometric::Point2D;

/// Represents a 2D Circle
#[derive(Debug)]
pub struct Circle2D {
    /// Radius of the Circle
    pub r: f64,
    /// Center-Point of the Circle
    pub m: Point2D
}

impl Circle2D {
    /// Returns a Circle with given Radius and Center-Point
    ///
    /// # Arguments
    ///
    /// * `r` - Radius of Circle
    /// * `m` - Center-Point of the circle
    ///
    /// # Example
    ///
    /// ```
    /// use geometric::{Circle2D, Point2D};
    ///
    /// //Creates a white Circle with center in (0,0)
    /// let circle = Circle2D::new(5.0, Point2D::new(0.0,0.0));
    /// ```
    pub fn new(r: f64, m: Point2D) -> Circle2D {
        Circle2D { r: r, m: m }
    }

    /// Returns the color of the Circle
    pub fn get_color(&self) -> Color { self.m.get_color() }
}

impl std::fmt::Display for Circle2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Circle Radius: {}, Middle: {}", self.r, self.m)
    }
}

impl Geometric2D for Circle2D {
    fn homogenize(&mut self) {
        self.m.homogenize();
    }

    fn transform(&mut self, tx: f64, ty: f64) {
        self.m.transform(tx, ty);
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.r = self.r * sx;
    }

    fn rotate(&mut self, angle: f64) {
        self.m.rotate(angle);
    }

    fn rotate_from_point(&mut self, angle: f64, p: &Point2D) {
        self.m.rotate_from_point(angle, p);
    }

    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D) {
        self.r *= sx;
    }

    fn draw(&self, canvas: &mut raster::Image) {
        let xm: i32 = self.m.x as i32;
        let ym: i32 = self.m.y as i32;
        let mut radius: i32 = self.r as i32;
        let mut x: i32 = -radius;
        let mut y: i32 = 0;
        let mut err: i32 = 2 - 2 * radius;
        loop {
            canvas.set_pixel(xm - x, ym - y, self.m.get_color().clone());/* I. Quadrant +x +y */
            canvas.set_pixel(xm + x, ym - y, self.m.get_color().clone());/* II. Quadrant -x +y */
            canvas.set_pixel(xm + x, ym + y, self.m.get_color().clone());/* III. Quadrant -x -y */
            canvas.set_pixel(xm - x, ym + y, self.m.get_color().clone());/* IV. Quadrant +x -y */

            radius = err;
            if radius <= y {
                /* e_xy+e_y < 0 */
                y += 1;
                err += y * 2 + 1;
            }
            if radius > x || err > y {
                /* e_xy+e_x > 0 or no 2nd y-step */
                x += 1;
                err += x * 2 + 1;
            }

            if x > 0 { break; }
        }
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

    fn draw_aa(&self, canvas: &mut raster::Image) {
        self.draw_outline_aa(canvas);
        self.draw(canvas);
    }

    fn draw_outline_aa(&self, canvas: &mut raster::Image) {
        let xm: i32 = self.m.x as i32;
        let ym: i32 = self.m.y as i32;
        let mut x: i32 = self.r as i32;
        let mut y: i32 = 0; /* II. quadrant from bottom left to top right */
        let mut i: i32;
        let mut x2: i32;
        let mut e2: i32;
        let mut err: i32 = 2 - 2 * x; /* error of 1.step */
        let mut r: i32 = 1 - err;

        loop {
            i = (255 * (err + 2 * (x + y) - 2).abs()) / r; /* get blend value of pixel */
            let color = Color::rgba(self.get_color().r, self.get_color().g, self.get_color().b, i as u8);
            canvas.set_pixel(xm + x, ym - y, color.clone()); /* I. Quadrant */
            canvas.set_pixel(xm + y, ym + x, color.clone()); /* II. Quadrant */
            canvas.set_pixel(xm - x, ym + y, color.clone()); /* III. Quadrant */
            canvas.set_pixel(xm - y, ym - x, color.clone()); /* IV. Quadrant */
            if x == 0 { break; }
            e2 = err;
            x2 = x; /* remember values */
            if err > y {
                /* x step */
                i = (255 * (err + 2 * x - 1)) / r; /* outward pixel */
                if i < 255 {
                    let cc = Color::rgba(self.get_color().r, self.get_color().g, self.get_color().b, i as u8);
                    canvas.set_pixel(xm + x, ym - y + 1, cc.clone());
                    canvas.set_pixel(xm + y - 1, ym + x, cc.clone());
                    canvas.set_pixel(xm - x, ym + y - 1, cc.clone());
                    canvas.set_pixel(xm - y + 1, ym - x, cc.clone());
                }
                x -= 1;
                err -= x * 2 - 1;
            }
            x2 -= 1;
            if e2 <= x2 + 1 {
                /* y step */
                i = (255 * (1 - 2 * y - e2)) / r; /* inward pixel */
                if i < 255 {
                    let cc = Color::rgba(self.get_color().r, self.get_color().g, self.get_color().b, i as u8);
                    canvas.set_pixel(xm + x2, ym - y, cc.clone());
                    canvas.set_pixel(xm + y, ym + x2, cc.clone());
                    canvas.set_pixel(xm - x2, ym + y, cc.clone());
                    canvas.set_pixel(xm - y, ym - x2, cc.clone());
                }
                y -= 1;
                err -= y * 2 - 1;
            }
        }
    }
}