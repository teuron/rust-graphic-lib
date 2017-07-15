extern crate graphic_library;
extern crate raster;

use raster::Color;
use std::cell::RefCell;
use std::rc::Rc;
use graphic_library::geometric::Point2D;
//use graphic_library::geometric::Triangle2D;
use graphic_library::geometric::Circle2D;
use graphic_library::geometric::Geometric2D;
//use graphic_library::geometric::Line2D;
use graphic_library::graphic::Renderer;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    // let triangle = Triangle2D::new(Point2D::new_color(0.0, 0.0, 1.0, Color::rgb(255, 0, 0)), Point2D::new_color(0.0, 10000.0, 1.0, Color::rgb(0,255,0)), Point2D::new_color(10000.0, 0.0, 1.0, Color::rgb(0, 0, 255)));
    //  let triangle2 = Triangle2D::new(Point2D::new_color(10000.0, 10000.0, 1.0, Color::rgb(255, 0, 0)), Point2D::new_color(0.0, 10000.0, 1.0, Color::rgb(0,0,255)), Point2D::new_color(10000.0, 0.0, 1.0, Color::rgb(0, 255, 0)));
    // let mut triangle = Triangle2D::new(Point2D::new_homogenized(200.0, 800.0), Point2D::new_homogenized(800.0, 800.0), Point2D::new_homogenized(500.0, 200.0));
    //let center = Line2D::new(Point2D::new_color(0.0, 0.0, Color::rgb(255, 0, 0)), Point2D::new(150.0, 500.0));
    //let center1 = Line2D::new(Point2D::new_color(150.0, 0.0, Color::rgb(0, 255, 0)), Point2D::new(150.0, 500.0));
    //let center2 = Line2D::new(Point2D::new_color(0.0, 160.0, Color::rgb(0, 0, 255)), Point2D::new(150.0, 160.0));

    // r.add(&center);
    let c = Rc::new(RefCell::new(Box::new(Circle2D::new(100.0f64, Point2D::new_color(500.0, 500.0, Color::rgb(200, 100, 30)))) as Box<Geometric2D>));
    let mut r = Renderer::new(1000, 1000);
    r.add(&c);
    //r.add(&center2);
    //  r.add(&triangle2);
    //  for i in 0..360 {

    //      triangle.rotate_from_point(1.0, &center);
    //      let mut r = Renderer::new(1000, 1000);
    //      r.add(&triangle);
    r.draw();
    c.borrow_mut().transform(100.0, 100.0);
    r.draw();
    r.save_as("nice_out2.png".to_owned());
    // }


    let elapsed = now.elapsed();
    let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
    println!("Seconds: {}", sec);
}
