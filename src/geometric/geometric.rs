extern crate raster;

use geometric::Point2D;
use raster::{Color};

pub trait Geometric2D {
    fn homogenize(&mut self);
    fn transform(&mut self, tx: f64, ty: f64);
    fn scale(&mut self, sx: f64, sy: f64);
    fn rotate(&mut self, angle: f64);
    fn rotate_from_point(&mut self, angle: f64, p: &Point2D);
    fn scale_from_point(&mut self, sx: f64, sy: f64, p: &Point2D);
    fn draw(&self, canvas: &mut raster::Image);
    fn draw_outline(&self, canvas: &mut raster::Image);
}

pub fn interpolate(a: f64, b: f64, t: f64) -> f64 {
    a * (1.0f64 - t) + b * t
}

pub fn interpolate_barycentric(a: f64, b: f64, c: f64, alpha: f64, beta: f64, gamma: f64) -> f64 {
    a * alpha + b * beta + c * gamma
}

pub fn draw_line(from: &Point2D, to: &Point2D, canvas: &mut raster::Image) {
    let dx: i32 = (to.x - from.x).abs() as i32;
    let dy: i32 = (to.y - from.y).abs() as i32;

    let sgnx: i32 = (to.x - from.x).signum() as i32;
    let sgny: i32 = (to.y - from.y).signum() as i32;
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
    let mut x: i32 = from.x as i32;
    let mut y: i32 = (from.y as i32 - canvas.height as i32).abs();
    let mut err: f64 = err_slow / 2f64;

    canvas.set_pixel(x, y, from.get_color()).unwrap();
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
        let dif = ((x as f64) - from.x).abs() / (dx as f64);
        let r = interpolate(from.get_color().r as f64, to.get_color().r as f64, dif);
        let g = interpolate(from.get_color().g as f64, to.get_color().g as f64, dif);
        let b = interpolate(from.get_color().b as f64, to.get_color().b as f64, dif);
        canvas.set_pixel(x, y, Color::rgb(r as u8, g as u8, b as u8)).unwrap();
    }
}

pub fn draw_line_aa(from: &Point2D, to: &Point2D, canvas: &mut raster::Image) {
    let dx = (to.x - from.x).abs() as i32;
    let sx = if from.x < to.x { 1 } else { -1 };
    let dy = (to.y - from.y).abs() as i32;
    let sy = if from.y < to.y { 1 } else { -1 };
    let mut x2: i32;
    let mut e2: i32;
    let mut err: i32 = dx - dy; /* error value e_xy */
    let ed: f32 = if dx + dy == 0 { 1.0f32 } else { ((dx * dx + dy * dy) as f32).sqrt() };
    let ei: i32 = ed as i32;

    let mut x0: i32 = from.x as i32;
    let mut y0: i32 = from.y as i32;
    let x1: i32 = to.x as i32;
    let y1: i32 = to.y as i32;
    loop {
        let test = 1.0f32 - ((err - dx + dy).abs() as f32 / ed);
        let color = from.get_color();
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