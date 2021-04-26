use crate::geom::point::Point;

// Sections from horizontal alignment (Tangent, Spiral and CircularCurve)
// are well defined with these five parameters
pub trait HSection {
   fn start_point(&self) -> Point;
   fn start_x(&self) -> f64 {
      self.start_point().x
   }
   fn start_y(&self) -> f64 {
      self.start_point().y
   }
   fn end_point(&self) -> Point;
   fn end_x(&self) -> f64 {
      self.end_point().x
   }
   fn end_y(&self) -> f64 {
      self.end_point().y
   }
   fn start_radius(&self) -> f64;
   fn end_radius(&self) -> f64;
   fn length(&self) -> f64;
   fn start_azimuth(&self) -> f64;
   fn end_azimuth(&self) -> f64;
}
