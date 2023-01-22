use crate::geom::clothoid::Clothoid;
use crate::geom::point::Point;
//use crate::geom::vector::Vector;
use crate::eq001;
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
   /// Creates a new HClothoid
   /// One of the radius must be zero, but not both
   pub fn new(start_point: Point, start_azimuth: f64, start_radius: f64, 
      end_radius: f64, length: f64) -> Self {
         if eq001(0.0, start_radius) {
            if eq001(0.0, end_radius) {
               panic!("Clothoid can't have start and end radius equal zero");
            } 
         } else {
            if !eq001(0.0, end_radius) {
               panic!("Clothoid must have one of the radius equals zero");
            } 
         }
         if eq001(length, 0.0) {
            panic!("Clothoid can't have length equals zero");
         }
         HClothoid{start_point, start_azimuth, start_radius, end_radius, length}
      }
}
impl HClothoid {
   pub fn clothoid(&self) -> Clothoid {
      Clothoid { parameter: self.parameter(), end_radius: self.radius_in_tangent_to_circle_point() }
   }
   pub fn radius_in_tangent_to_circle_point(&self) -> f64 {
      if eq001(self.start_radius, 0.0) {
         self.end_radius
      } else {
         self.start_radius
      }
   }
   pub fn parameter(&self) -> f64 {
      let r = self.radius_in_tangent_to_circle_point();
      let a = (r.abs()* self.length).sqrt();
      if r<0.0 {
         a * -1.0
      } else {
         a
      }
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
   /// Tests if method new() panics when it 
   /// receives start_radius==0 && end_radius==0
   fn test_panic_1_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 0.0;
      let length = 80.22;
      let _cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
   }
   #[test]
   #[should_panic]
   /// One of the radius must be zero
   fn test_panic_2_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 90.0;
      let start_radius = -400.0;
      let end_radius = 400.0;
      let length = 0.0;
      let _cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
   }
   #[test]
   #[should_panic]
   /// Length can't be zero
   fn test_panic_3_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 90.0;
      let start_radius = 0.0;
      let end_radius = 400.0;
      let length = 0.0;
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
      assert_eq!(true, eq001(start_point.x, cl.start_point.x));
      assert_eq!(true, eq001(start_point.y, cl.start_point.y));
      assert_eq!(true, eq001(start_azimuth, cl.start_azimuth));
      assert_eq!(true, eq001(start_radius, cl.start_radius));
      assert_eq!(true, eq001(end_radius, cl.end_radius));
      assert_eq!(true, eq001(length, cl.length));
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
      assert_eq!(true, eq001(0.08913, cl.end_azimuth()));
      // end azimuth greater than 2PI
      let start_azimuth = 2.0*PI - 0.05;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(0.03913, cl.end_azimuth()));
      // end azimuth less than zero
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(6.194055, cl.end_azimuth()));     
   }
   #[test]
   fn test_radius_in_tangent_to_circle_point() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let length = 80.22;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length); 
      assert!(eq001(cl.end_radius, cl.radius_in_tangent_to_circle_point()));
      let end_radius = 450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length);
      assert!(eq001(cl.end_radius, cl.radius_in_tangent_to_circle_point()));
      let start_radius = 450.0;
      let end_radius = 0.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length); 
      assert!(eq001(cl.start_radius, cl.radius_in_tangent_to_circle_point()));
      let start_radius = -450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length); 
      assert!(eq001(cl.start_radius, cl.radius_in_tangent_to_circle_point()));

   }
   #[test]
   fn test_parameter() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let length = 80.22;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      let a = -(cl.length*cl.end_radius.abs()).sqrt();
      assert!(eq001(a,  cl.parameter()));
      let end_radius = 450.0;
      let a = (cl.length*cl.end_radius.abs()).sqrt();
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert!(eq001(a, cl.parameter()));
   }
}