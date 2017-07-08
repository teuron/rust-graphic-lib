extern crate raster;

use std;
use std::f64::consts::PI;

use raster::Color;
use geometric::geometric::Geometric2D;

#[derive(Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    color: Color
}

impl Point2D {
    //Construct a Point(x,y)
    pub fn new(x: f64, y: f64, z: f64) -> Point2D {
        Point2D::new_color(x, y, z, Color::white())
    }
    pub fn new_homogenized(x: f64, y: f64) -> Point2D {
        Point2D::new_color(x, y, 1.0f64, Color::white())
    }
    pub fn new_color_homogenized(x: f64, y: f64, color: Color) -> Point2D {
        Point2D::new_color(x, y, 1.0f64, color)
    }
    pub fn new_color(x: f64, y: f64, z: f64, color: Color) -> Point2D {
        Point2D {
            x: x,
            y: y,
            z: z,
            color: color
        }
    }
    pub fn get_color(&self) -> Color { self.color.clone() }
}

impl std::fmt::Display for Point2D {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point x:{}, y:{}, z:{}", self.x, self.y, self.z)
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
        self.x = self.x * a.cos() - self.y * a.sin();
        self.y = self.x * a.sin() + self.y * a.cos();
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
}