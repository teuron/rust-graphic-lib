extern crate raster;

use geometric::Point2D;
use geometric::Circle2D;
use raster::{Color};

/// Basic 2D Geometric Trait with standard functions
pub trait Geometric2D {
    /// Homogenizes the Coordinates of the given struct
    fn homogenize(&mut self);

    /// Transforms a Geometric-Object to a specified Position
    ///
    /// # Arguments
    ///
    /// * `tx` - Translation on the x-axis
    /// * `ty` - Translation on the y-axis
    ///
    fn transform(&mut self, tx: f64, ty: f64);

    /// Scales a Geometric-Object
    ///
    /// # Arguments
    ///
    /// * `sx` - Scales on the x-axis
    /// * `sy` - Scales on the y-axis
    ///
    fn scale(&mut self, sx: f64, sy: f64);

    /// Rotates a Geometric-Object from the coordinate origin (0,0)
    ///
    /// # Arguments
    ///
    /// * `angle` - Angle in degrees, positive values rotate counter-clock vice
    ///
    fn rotate(&mut self, angle: f64);

    /// Rotates a Geometric-Object from a specified point as origin
    ///
    /// # Arguments
    ///
    /// * `angle` - Angle in degrees, positive values rotate counter-clock vice
    /// * `p` - Point of origin to rotate from
    ///
    fn rotate_from_point(&mut self, angle: f64, p: &Point2D);

    /// Scales a Geometric-Object from a specified point as origin
    ///
    /// # Arguments
    ///
    /// * `sx` - Scales on the x-axis
    /// * `sy` - Scales on the y-axis
    /// * `angle` - Angle in degrees, positive values rotate counter-clock vice
    ///
    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D);

    /// Draws a Geometric-Object onto an Image
    ///
    /// # Arguments
    ///
    /// * `canvas` - Drawing Surface
    ///
    fn draw(&self, canvas: &mut raster::Image);

    /// Draws the outline of a Geometric-Object onto an Image
    ///
    /// # Arguments
    ///
    /// * `canvas` - Drawing Surface
    ///
    fn draw_outline(&self, canvas: &mut raster::Image);

    /// Draws an anti-aliased Geometric-Object onto an Image
    ///
    /// # Arguments
    ///
    /// * `canvas` - Drawing Surface
    ///
    fn draw_aa(&self, canvas: &mut raster::Image);

    /// Draws the anti-aliased outline of a Geometric-Object onto an Image
    ///
    /// # Arguments
    ///
    /// * `canvas` - Drawing Surface
    ///
    fn draw_outline_aa(&self, canvas: &mut raster::Image);
}

/// Linearly interpolates two values together
///
/// # Arguments
///
/// * `a` - 1st value to interpolate
/// * `b` - 2nd value to interpolate
/// * `t` - Percentage Value between 0..1
/// # Example
///
/// ```
/// //returns 7.5
/// //a*(1-t)+b*t
/// let interpolated = interpolate(0.0, 10.0, 0.75);
/// ```
pub fn interpolate(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0f64 - t) + b * t
}

/// Barycentricly interpolates three values together
/// Condition: alpha + beta + gamma == 1
/// # Arguments
///
/// * `a` - 1st value to interpolate
/// * `b` - 2nd value to interpolate
/// * `c` - 2nd value to interpolate
/// * `alpha` - Percentage Value between 0..1
/// * `beta` - Percentage Value between 0..1
/// * `gamma` - Percentage Value between 0..1
/// # Example
///
/// ```
/// //a*alpha + b*beta + c*gamma
/// let interpolated = interpolate_barycentric(0.0, 10.0, 20.0, 0.25, 0.25, 0.50);
/// ```
pub fn interpolate_barycentric(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> f64 {
    a * alpha + b * beta + c * gamma
}

pub fn draw_circle_aa(circle: &Circle2D, canvas: &mut raster::Image) {
    let xm: i32 = circle.m.x as i32;
    let ym: i32 = circle.m.y as i32;
    let mut x: i32 = circle.r as i32;
    let mut y: i32 = 0; /* II. quadrant from bottom left to top right */
    let mut i: i32;
    let mut x2: i32;
    let mut e2: i32;
    let mut err: i32 = 2 - 2 * x; /* error of 1.step */
    let mut r: i32 = 1 - err;

    loop {
        i = (255 * (err + 2 * (x + y) - 2).abs()) / r; /* get blend value of pixel */
        let color = Color::rgba(circle.get_color().r, circle.get_color().g, circle.get_color().b, i as u8);
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
                let cc = Color::rgba(circle.get_color().r, circle.get_color().g, circle.get_color().b, i as u8);
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
                let cc = Color::rgba(circle.get_color().r, circle.get_color().g, circle.get_color().b, i as u8);
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