use crate::geom::point::Point;

// Sections from horizontal alignment (Tangent, Spiral and CircularCurve)
// are well defined with these five parameters
pub trait HSection {
   fn start_point() -> Point;
   fn end_point() -> Point;
   fn start_radius() -> f64;
   fn end_radius() -> f64;
   fn length() -> f64;
}