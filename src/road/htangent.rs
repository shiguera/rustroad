/// HTangent is a straight line section of a road
use crate::geom::point::Point;
use crate::geom::vector::Vector;
use crate::geom::angles::Azimuth;
use super::hsection::HSection;
use std::f64::consts::PI;
use crate::{eq001, deg2rad, normalize_360};


pub struct HTangent {
   start_point: Point,
   azimuth: Azimuth,
   length: f64
}
impl HTangent {
   pub fn new(start_point: Point, azimuth_value: f64, length: f64) -> Self {
      if length < 0.0_f64 || eq001(length, 0.0) {
         panic!("Length must be greater than zero");
      }
      HTangent{start_point, azimuth: Azimuth::new(azimuth_value), length}
   }
   /// Unit vector in the positive direction of the tangent
   pub fn vector(&self) -> Vector {
      Vector::new(self.angle().cos(), self.angle().sin())
   }
   /// The trigonometric angle measured in radians from East
   /// angle = PI/2 - azimut_in_radians
   pub fn angle(&self) -> f64 {
      let ang_degrees = normalize_360(90.0 - self.azimuth.value);
      deg2rad(ang_degrees)
   }
}

impl HSection for HTangent {
   fn start_point(&self) -> Point {
      self.start_point
   }
   fn end_point(&self) -> Point {
      self.point_at_s(self.length())
   }
   fn start_radius(&self) -> f64 {
      0.0_f64
   }
   fn end_radius(&self) -> f64 {
      0.0_f64
   }
   fn length(&self) -> f64 {
      self.length
   }
   fn start_azimuth(&self) -> f64 {
      self.azimuth.value
   }
   fn end_azimuth(&self) -> f64 {
      self.azimuth.value
   }
   fn azimuth_at_s(&self, _s:f64) -> f64 {
      self.azimuth.value
   }
   fn point_at_s(&self, s:f64) -> Point {
      if s<0.0 || s > self.length() {
         panic!("s less than zero or grater than length");
      }
      let x = self.start_x() + s*self.angle().cos();
      let y = self.start_y() + s*self.angle().sin();
      Point::new(x, y)
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::eq001;

   #[test]
   #[should_panic]
   fn test_new_length_negative() {
      // Tests that length can't be negative
      let p = Point::new(-1.0, -1.0);
      let angle = 20.0*PI/180.0;
      let _r1 = HTangent::new(p, angle, -100.0);
   }

   #[test]
   fn test_new() {
      let p = Point::new(-1.0, -1.0);
      let az = 30.0;
      let r1 = HTangent::new(p, az, 100.0);
      eq001(r1.start_point.x, -1.0);
      eq001(r1.start_point.y, -1.0);
      eq001(r1.azimuth.value, az);
      eq001(r1.length, 100.0);
      // Check that negative angle is converted to normalized positive 
      let az = -45.0;
      let r1 = HTangent::new(p, az, 100.0);
      eq001(r1.azimuth.value, 315.0);
   }
   #[test]
   fn test_angle() {
      let p: Point = Point::new(0.0,0.0);
      let t = HTangent::new(p, 0.0, 1.0);
      assert!(eq001(deg2rad(90.0), t.angle()));
      let t = HTangent::new(p, 30.0, 1.0);
      assert!(eq001(deg2rad(60.0), t.angle()));
      let t = HTangent::new(p, 120.0, 1.0);
      assert!(eq001(deg2rad(330.0), t.angle()));
      let t = HTangent::new(p, 315.0, 1.0);
      assert!(eq001(deg2rad(135.0), t.angle()));
      let t = HTangent::new(p, 720.0, 1.0);
      assert!(eq001(deg2rad(90.0), t.angle()));
   }
   #[test]
   fn test_vector() {
      // Azimuth 30 => Q1
      let p = Point::new(-1.0, -1.0);
      let az = 30.0;
      let r1 = HTangent::new(p, az, 100.0);
      let v = r1.vector();
      assert!(eq001(v.length(), 1.0));
      assert!(eq001(v.vx, deg2rad(60.0).cos()));
      assert!(eq001(v.vy, deg2rad(60.0).sin()));
      // Q4
      let az = 120.0;
      let r1 = HTangent::new(p, az, 100.0);
      let v = r1.vector();
      assert!(eq001(v.length(), 1.0));
      assert!(eq001(v.vx, deg2rad(330.0).cos()));
      assert!(eq001(v.vy, deg2rad(330.0).sin()));
      //Q3
      let angle = 210.0;
      let r1 = HTangent::new(p, angle, 100.0);
      let v = r1.vector();
      assert!(eq001(v.length(), 1.0));
      assert!(eq001(v.vx, deg2rad(240.0).cos()));
      assert!(eq001(v.vy, deg2rad(240.0).sin()));
      // Q2
      let angle = 300.0;
      let r1 = HTangent::new(p, angle, 100.0);
      let v = r1.vector();
      assert!(eq001(v.length(), 1.0));
      assert!(eq001(v.vx, deg2rad(150.0).cos()));
      assert!(eq001(v.vy, deg2rad(150.0).sin()));      
   }
   #[test]
   fn test_end_point() {
      // azimuth zero
      let p1 = Point::new(0.0, 0.0);
      let az = 0.0;
      let length = 100.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 0.0));
      assert!(eq001(r1.end_point().y, 100.0));
      // Q1
      let az = 30.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 100.0*deg2rad(60.0).cos()));
      assert!(eq001(r1.end_point().y,  100.0*deg2rad(60.0).sin()));
      // Q4
      let az = 120.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 100.0*deg2rad(330.0).cos()));
      assert!(eq001(r1.end_point().y, 100.0*deg2rad(330.0).sin()));
      // Q3
      let az = 210.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 100.0*deg2rad(240.0).cos()));
      assert!(eq001(r1.end_point().y, 100.0*deg2rad(240.0).sin()));
      // Q2
      let az = 300.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 100.0*deg2rad(150.0).cos()));
      assert!(eq001(r1.end_point().y, 100.0*deg2rad(150.0).sin()));      
      // Azimuth 90.0
      let az = 90.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 100.0));
      assert!(eq001(r1.end_point().y, 0.0));      
      // Azimuth 180.0
      let az = 180.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 0.0));
      assert!(eq001(r1.end_point().y, -100.0));      
      // Azimuth 270.0
      let az = 270.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, -100.0));
      assert!(eq001(r1.end_point().y, 0.0));
      // Azimuth 360.0
      let az = 360.0;
      let r1 = HTangent::new(p1, az, length);
      assert!(eq001(r1.end_point().x, 0.0));
      assert!(eq001(r1.end_point().y, 0.0));            
   }
   #[test]
   fn test_point_at_s() {
      let p = Point::new(0.0, 0.0);
      let v = Vector::new(1.0, 0.0);
      let r = HTangent::new(p, v.angle(), 10.0);
      let q = r.point_at_s(5.0);
      assert!(eq001(5.0, q.x));
      assert!(eq001(0.0, q.y));
      //
      let v = Vector::new(0.0, -1.0);
      let r = HTangent::new(p, v.angle(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(0.0, q.x));
      assert_eq!(true, eq001(-5.0, q.y));
      //
      let v = Vector::new(-1.0, -1.0);
      println!("{}", v.angle());
      let r = HTangent::new(p, v.angle(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(-5.0*(PI/4.0).cos(), q.x));
      assert_eq!(true, eq001(-5.0*(PI/4.0).sin(), q.y));      
      //
      let v = Vector::new(-1.0, 1.0);
      println!("{}", v.angle());
      let r = HTangent::new(p, v.angle(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(-5.0*(PI/4.0).cos(), q.x));
      assert_eq!(true, eq001(5.0*(PI/4.0).sin(), q.y));      
   }
   #[test]
   #[should_panic]
   fn test_point_at_s_panic_1() {
      let p = Point::new(0.0, 0.0);
      let v = Vector::new(1.0, 0.0);
      let r = HTangent::new(p, v.angle(), 10.0);
      let _q = r.point_at_s(15.0);
   }
   #[test]
   #[should_panic]
   fn test_point_at_s_panic_2() {
      let p = Point::new(0.0, 0.0);
      let v = Vector::new(1.0, 0.0);
      let r = HTangent::new(p, v.angle(), 10.0);
      let _q = r.point_at_s(-5.0);
   }

}
