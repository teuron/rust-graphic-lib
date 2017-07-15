extern crate raster;

use std;
use geometric::{Point2D, Geometric2D, interpolate};
use raster::Color;

/// Represents a 2D Line
#[derive(Debug)]
pub struct Line2D {
    /// Start-Point of the Line
    from: Point2D,
    /// End-Point of the Line
    to: Point2D
}

impl Line2D {
    /// Returns a colored Line
    ///
    /// # Arguments
    ///
    /// * `a` - Point of a Line
    /// * `b` - Point of a Line
    ///
    /// # Example
    ///
    /// ```
    /// extern crate graphic_library;
    /// use graphic_library::geometric::{Line2D, Point2D};
    /// fn main(){
    /// //Creates a colored Line
    /// let line = Line2D::new(Point2D::new(0.0, 0.0), Point2D::new(5.0, 5.0));
    /// }
    /// ```
    pub fn new(from: Point2D, to: Point2D) -> Line2D {
        Line2D {
            from: from,
            to: to
        }
    }

    fn draw(&self, canvas: &mut raster::Image) {
        let dx: i32 = (self.to.x - self.from.x).abs() as i32;
        let dy: i32 = (self.to.y - self.from.y).abs() as i32;

        let sgnx: i32 = (self.to.x - self.from.x).signum() as i32;
        let sgny: i32 = (self.to.y - self.from.y).signum() as i32;
        //Parallel Case
        let mut ppx: i32 = 0;
        let mut ppy: i32 = 0;

        //Diagonal Case
        let ddx: i32 = sgnx;
        let ddy: i32 = -sgny;

        let err_fast: f64;
        let err_slow: f64;
        if dx > dy {
            ppx = sgnx;
            err_fast = dy as f64;
            err_slow = dx as f64;
        } else {
            ppy = -sgny;
            err_fast = dx as f64;
            err_slow = dy as f64;
        }
        let mut x: i32 = self.from.x as i32;
        let mut y: i32 = (self.from.y as i32 - canvas.height as i32).abs();
        let mut err: f64 = err_slow / 2f64;

        canvas.set_pixel(x, y, self.from.get_color()).unwrap();
        let c: i32 = err_slow as i32;
        for _ in 0..c {
            err -= err_fast;
            //Diagonal Case
            if
                err < 0f64 {
                err += err_slow;
                x += ddx;
                y += ddy;
            } else //Parallel Case
            {
                x += ppx;
                y += ppy;
            }
            let dif = ((x as f64) - self.from.x).abs() / (dx as f64);
            let r = interpolate(self.from.get_color().r as f64, self.to.get_color().r as f64, dif);
            let g = interpolate(self.from.get_color().g as f64, self.to.get_color().g as f64, dif);
            let b = interpolate(self.from.get_color().b as f64, self.to.get_color().b as f64, dif);
            canvas.set_pixel(x, y, Color::rgb(r as u8, g as u8, b as u8)).unwrap();
        }
    }

    fn draw_aa(&self, canvas: &mut raster::Image) {
        let dx = (self.to.x - self.from.x).abs() as i32;
        let sx = if self.from.x < self.to.x { 1 } else { -1 };
        let dy = (self.to.y - self.from.y).abs() as i32;
        let sy = if self.from.y < self.to.y { 1 } else { -1 };
        let mut x2: i32;
        let mut e2: i32;
        let mut err: i32 = dx - dy; /* error value e_xy */
        let ed: f32 = if dx + dy == 0 { 1.0f32 } else { ((dx * dx + dy * dy) as f32).sqrt() };
        let ei: i32 = ed as i32;

        let mut x0: i32 = self.from.x as i32;
        let mut y0: i32 = self.from.y as i32;
        let x1: i32 = self.to.x as i32;
        let y1: i32 = self.to.y as i32;
        loop {
            let test = 1.0f32 - ((err - dx + dy).abs() as f32 / ed);
            let color = self.from.get_color();
            canvas.set_pixel(x0, y0, Color::rgb(((color.r as f32) * test) as u8, (color.g as f32 * test) as u8, (color.b as f32 * test) as u8)).unwrap();//TODO Add Color and mixing
            e2 = err;
            x2 = x0;

            if 2 * e2 >= -dx {
                if x0 == x1 { break; }
                if e2 + dy < ei {
                    let tt = 1.0f32 - (e2 + dy) as f32 / ed;
                    canvas.set_pixel(x0, y0 + sy, Color::rgb((color.r as f32 * tt) as u8, (color.g as f32 * tt) as u8, (color.b as f32 * tt) as u8)).unwrap();
                }
                err -= dy;
                x0 += sx;
            }
            if 2 * e2 <= dy {
                if y0 == y1 { break; }
                if dx - e2 < ei {
                    let base = 1.0f32 - ((dx - e2) as f32 / ed);
                    canvas.set_pixel(x2 + sx, y0, Color::rgb((color.r as f32 * base) as u8, (color.g as f32 * base) as u8, (color.b as f32 * base) as u8)).unwrap();
                }
                err += dx;
                y0 += sy;
            }
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
        self.draw(canvas);
    }

    fn draw_outline(&self, canvas: &mut raster::Image) {
        self.draw(canvas);
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

    fn draw_aa(&self, canvas: &mut raster::Image) {
        self.draw_aa(canvas);
    }

    fn draw_outline_aa(&self, canvas: &mut raster::Image) {
        self.draw_aa(canvas);
    }
}
