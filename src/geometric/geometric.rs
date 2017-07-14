extern crate raster;

use geometric::Point2D;

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