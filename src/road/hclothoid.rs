use crate::geom::point::Point;
//use crate::geom::vector::Vector;
use crate::assert_eq001;
use super::hsection::HSection;
use std::f64::consts::PI;

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
         if assert_eq001(0.0, start_radius) && assert_eq001(0.0, end_radius) {
            panic!("Clothoid can't have start and end radius equal zero");
         }
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
      let inc_az: f64;
      if self.start_radius.abs() > 0.0 {
         inc_az = self.length() / 2.0 / self.start_radius();
      } else {
         inc_az = self.length() / 2.0 / self.end_radius();
      }
      let mut end_az = self.start_azimuth() + inc_az;
      if end_az < 0.0 {
         end_az = 2.0 * PI  + end_az;
      } 
      if end_az > 2.0 * PI {
         end_az = end_az - 2.0 * PI;
      }
      end_az
   }
   fn azimuth_at_s(&self, _s:f64) -> f64 {
      todo!();
   }
   fn point_at_s(&self, _s:f64) -> Point {
      todo!();
   }
}
#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   #[should_panic]
   fn test_new_panic() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 0.0;
      let length = 80.22;
      let _cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
   }
   #[test]
   fn test_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, assert_eq001(start_point.x, cl.start_point.x));
      assert_eq!(true, assert_eq001(start_point.y, cl.start_point.y));
      assert_eq!(true, assert_eq001(start_azimuth, cl.start_azimuth));
      assert_eq!(true, assert_eq001(start_radius, cl.start_radius));
      assert_eq!(true, assert_eq001(end_radius, cl.end_radius));
      assert_eq!(true, assert_eq001(length, cl.length));
   }
   #[test]
   fn test_end_azimuth() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, assert_eq001(0.08913, cl.end_azimuth()));
      // end azimuth greater than 2PI
      let start_azimuth = 2.0*PI - 0.05;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, assert_eq001(0.03913, cl.end_azimuth()));
      // end azimuth less than zero
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, assert_eq001(6.194055, cl.end_azimuth()));     
   }
}