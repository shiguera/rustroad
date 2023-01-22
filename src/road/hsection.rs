use crate::geom::point::Point;

/// Trait for horizontal alignment sections: Tangent, Spiral and CircularCurve\
///
/// They are well defined with six parameters: start_x, start_y, start_radius, 
/// start_azimuth, end_radius and length. Known these, the others (end_x, end_y,
/// end_azimuth, azimuth_at_s, point_at_s) can be calculated   
///    
/// The criteria used is:    
///    
/// **Radius:** 0 is for infinitus radius. Positive is for leftward curves\
/// **Azimuth:** is measured in sexagesimal degrees from the North counterclock-wise
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
   fn azimuth_at_s(&self, s:f64) -> f64;
   /// Incremento de azimuth entre los dos
   /// extremos de la alineación
   fn azimuth_increment(&self) -> f64 {
      self.end_azimuth() - self.start_azimuth()
   }
   fn point_at_s(&self, s:f64) -> Point;
}
