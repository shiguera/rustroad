use crate::geom::point::Point;
//use crate::geom::vector::Vector;
use super::hsection::HSection;
//use std::f64::consts::PI;

pub struct HClothoid {
   pub start_point: Point,
   pub start_azimuth: f64,
   pub start_radius: f64,
   pub end_radius: f64,
   pub length: f64
}

impl HClothoid {
   pub fn new(start_point: Point, start_azimuth: f64, start_radius: f64, 
      end_radius: f64, length: f64) -> Self {
         HClothoid{start_point, start_azimuth, start_radius, end_radius, length}
      }
}

impl HSection for HClothoid {
   fn start_point(&self) -> Point {
      self.start_point
   }
   fn end_point(&self) -> Point {
      todo!();
   }
   fn start_radius(&self) -> f64 {
      self.start_radius
   }
   fn end_radius(&self) -> f64 {
      self.end_radius
   }
   fn length(&self) -> f64 {
      self.length
   }
   fn start_azimuth(&self) -> f64 {
      self.start_azimuth
   }
   fn end_azimuth(&self) -> f64 {
      todo!();
   }
   fn azimuth_at_s(&self, _s:f64) -> f64 {
      todo!();
   }
   fn point_at_s(&self, _s:f64) -> Point {
      todo!();
   }
}
mod tests {
   #[cfg(test)]
   use super::*;
   #[cfg(test)]
   use crate::eq001;

   #[test]
   fn test_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 400.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(start_point.x, cl.start_point.x));
      assert_eq!(true, eq001(start_point.y, cl.start_point.y));
      assert_eq!(true, eq001(start_azimuth, cl.start_azimuth));
      assert_eq!(true, eq001(start_radius, cl.start_radius));
      assert_eq!(true, eq001(end_radius, cl.end_radius));
      assert_eq!(true, eq001(length, cl.length));

   }
}