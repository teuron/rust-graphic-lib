extern crate raster;

use std;
use std::f64::consts::PI;

use raster::Color;
use geometric::geometric::Geometric2D;

/// Represents a 2D Point
#[derive(Debug)]
pub struct Point2D {
    /// X-Coordinate of the Point
    pub x: f64,
    /// Y-Coordinate of the Point
    pub y: f64,
    /// Z-Coordinate of the Point
    pub z: f64,
    /// Color of the Point
    color: Color
}

impl Point2D {
    /// Returns a white Point
    ///
    /// # Arguments
    ///
    /// * `x` - X-Coordinate of the Point
    /// * `y` - Y-Coordinate of the Point
    ///
    /// # Example
    ///
    /// ```
    /// extern crate graphic_library;
    /// use graphic_library::geometric::{Point2D};
    /// fn main(){
    /// //Creates a white Point
    /// let point = Point2D::new(5.0, 5.0);
    /// }
    /// ```
    pub fn new(x: f64, y: f64) -> Point2D {
        Point2D::new_color_inhomogenized(x, y, 1.0f64, Color::white())
    }

    /// Returns an inhomogenized white Point
    ///
    /// # Arguments
    ///
    /// * `x` - X-Coordinate of the Point
    /// * `y` - Y-Coordinate of the Point
    /// * `z` - Homogeneous-Value
    ///
    /// # Example
    ///
    /// ```
    /// extern crate graphic_library;
    /// use graphic_library::geometric::{Point2D};
    /// fn main(){
    /// //Creates a white Point
    /// let point = Point2D::new_inhomogenized(5.0, 5.0, 5.0);
    /// }
    /// ```
    pub fn new_inhomogenized(x: f64, y: f64, z: f64) -> Point2D {
        Point2D::new_color_inhomogenized(x, y, z, Color::white())
    }

    /// Returns a colored Point
    ///
    /// # Arguments
    ///
    /// * `x` - X-Coordinate of the Point
    /// * `y` - Y-Coordinate of the Point
    /// * `color` - Color of the Point
    ///
    /// # Example
    ///
    /// ```
    /// extern crate graphic_library;
    /// extern crate raster;
    /// use raster::{Color};
    /// use graphic_library::geometric::{Point2D};
    /// fn main(){
    /// //Creates a colored Point
    /// let point = Point2D::new_color(5.0, 5.0, Color::rgb(10,10,10));
    /// }
    /// ```
    pub fn new_color(x: f64, y: f64, color: Color) -> Point2D {
        Point2D::new_color_inhomogenized(x, y, 1.0f64, color)
    }

    /// Returns an inhomogenized colored Point
    ///
    /// # Arguments
    ///
    /// * `x` - X-Coordinate of the Point
    /// * `y` - Y-Coordinate of the Point
    /// * `z` - Homogeneous-Value
    /// * `color` - Color of the Point
    ///
    /// # Example
    ///
    /// ```
    /// extern crate graphic_library;
    /// extern crate raster;
    /// use raster::{Color};
    /// use graphic_library::geometric::{Point2D};
    /// fn main(){
    /// //Creates a inhomogenized colored Point
    /// let point = Point2D::new_color_inhomogenized(5.0, 5.0, 5.0, Color::rgb(10,10,10));
    /// }
    /// ```
    pub fn new_color_inhomogenized(x: f64, y: f64, z: f64, color: Color) -> Point2D {
        Point2D {
            x: x,
            y: y,
            z: z,
            color: color
        }
    }

    /// Returns the color of the Point
    pub fn get_color(&self) -> Color { self.color.clone() }
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point x:{}, y:{}, z:{}", self.x, self.y, self.z)
    }
}

impl Clone for Point2D {
    fn clone(&self) -> Point2D {
        Point2D {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
            color: Color::rgba(self.color.r.clone(), self.color.g.clone(), self.color.b.clone(), self.color.a.clone())
        }
    }
}

impl Geometric2D for Point2D {
    fn homogenize(&mut self) {
        self.x /= self.z;
        self.y /= self.z;
        self.z = 1.0f64;
    }

    fn transform(&mut self, tx: f64, ty: f64) {
        self.x += tx;
        self.y += ty;
    }

    fn scale(&mut self, sx: f64, sy: f64) {
        self.x *= sx;
        self.y *= sy;
    }

    fn rotate(&mut self, angle: f64) {
        let a = angle * PI / 180.0;
        let x = self.x;
        let y = self.y;
        self.x = x * a.cos() - y * a.sin();
        self.y = x * a.sin() + y * a.cos();
    }

    fn rotate_from_point(&mut self, angle: f64, p: &Point2D) {
        self.transform(-p.x, -p.y);
        self.rotate(angle);
        self.transform(p.x, p.y);
    }

    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D) {
        self.transform(-p.x, -p.y);
        self.scale(sx, sy);
        self.transform(p.x, p.y);
    }

    fn draw(&self, canvas: &mut raster::Image) {
        canvas.set_pixel(self.x as i32, self.y as i32, self.color.clone()).unwrap();
    }

    fn draw_outline(&self, canvas: &mut raster::Image) {
        canvas.set_pixel(self.x as i32, self.y as i32, self.color.clone()).unwrap();
    }
    fn draw_aa(&self, canvas: &mut raster::Image) {
        canvas.set_pixel(self.x as i32, self.y as i32, self.color.clone()).unwrap();
    }

    fn draw_outline_aa(&self, canvas: &mut raster::Image) {
        canvas.set_pixel(self.x as i32, self.y as i32, self.color.clone()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn homogenize() {
        let mut point = Point2D::new_inhomogenized(10.0, 6.0, 2.0);
        point.homogenize();
        assert_eq!(5.0, point.x);
        assert_eq!(3.0, point.y);
        assert_eq!(1.0, point.z);
    }

    #[test]
    fn transform() {
        let mut point = Point2D::new(10.0, 10.0);
        point.transform(5.0, -5.0);
        assert_eq!(15.0, point.x);
        assert_eq!(5.0, point.y);
    }

    #[test]
    fn scale() {
        let mut point = Point2D::new(10.0, 5.0);
        point.scale(2.0, 0.2);
        assert_eq!(20.0, point.x);
        assert_eq!(1.0, point.y);
    }

    #[test]
    fn rotate() {
        let mut point = Point2D::new(1.0, 0.0);
        point.rotate(90.0);
        assert!((0.0 - point.x) < 0.00001);
        assert!((1.0 - point.y) < 0.00001);
    }

    #[test]
    fn scale_from_point() {
        let mut point = Point2D::new(10.0, 5.0);
        point.scale_from_point(2.0, 0.2, &Point2D::new(1.0, 1.0));
        assert_eq!(19.0, point.x);
        assert_eq!(1.8, point.y);
    }

    #[test]
    fn rotate_from_point() {
        let mut point = Point2D::new(2.0, 0.0);
        point.rotate_from_point(90.0, &Point2D::new(1.0, 0.0));
        assert!((0.0 - point.x) < 0.00001);
        assert!((1.0 - point.y) < 0.00001);
    }
}