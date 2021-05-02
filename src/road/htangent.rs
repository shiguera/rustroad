// HTangent is a straight line section of a road
use crate::geom::point::Point;
use crate::geom::vector::Vector;
use super::hsection::HSection;
use std::f64::consts::PI;

pub struct HTangent {
   start_point: Point,
   azimuth: f64,
   length: f64
}
impl HTangent {
   pub fn new(start_point: Point, azimuth: f64, length: f64) -> Self {
      if azimuth.abs()>2.0*PI {
         panic!("Azimuth can't be greater than 2*PI");
      }
      if length < 0.0_f64 {
         panic!("Length must be positive");
      }
      let mut angle = azimuth;
      if angle < 0.0 {
         angle = 2.0*PI + azimuth;
      }
      if angle == 2.0*PI {
         angle = 0.0;
      }
      HTangent{start_point, azimuth:angle, length}
   }
   pub fn vector(&self) -> Vector {
      // Unitary vector in the positive direction of the tangent
      Vector::new(self.azimuth.cos(), self.azimuth.sin())
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
      self.azimuth
   }
   fn end_azimuth(&self) -> f64 {
      self.azimuth
   }
   fn azimuth_at_s(&self, _s:f64) -> f64 {
      self.azimuth
   }
   fn point_at_s(&self, s:f64) -> Point {
      if s<0.0 || s > self.length() {
         panic!("s less than zero or grater than length");
      }
      let x = self.start_x() + s*self.azimuth.cos();
      let y = self.start_y() + s*self.azimuth.sin();
      Point::new(x, y)
   }
}

mod tests {
   #[cfg(test)]
   use super::*;
   #[cfg(test)]
   use crate::eq;
   #[cfg(test)]
   use crate::eq001;

   #[test]
   #[should_panic]
   fn test_new_azimuth_greater_than_2pi() {
      // Tests that azimuth can't be grater than 2*PI
      let p = Point::new(-1.0, -1.0);
      let angle = 720.0*PI/180.0;
      let _r1 = HTangent::new(p, angle, 100.0);
      let angle = -720.0*PI/180.0;
      let _r1 = HTangent::new(p, angle, 100.0);
   }
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
      let angle = 45.0*PI/180.0;
      let r1 = HTangent::new(p, angle, 100.0);
      assert_eq!(true, eq(r1.start_point.x, -1.0));
      assert_eq!(true, eq(r1.start_point.y, -1.0));
      assert_eq!(true, eq(r1.azimuth, angle));
      assert_eq!(true, eq(r1.length, 100.0));
      // Check that negative angle is converted to positive as 2PI-angle
      let angle = -45.0*PI/180.0;
      let r1 = HTangent::new(p, angle, 100.0);
      assert_eq!(true, eq(r1.start_point.x, -1.0));
      assert_eq!(true, eq(r1.start_point.y, -1.0));
      assert_eq!(true, eq(r1.azimuth, 2.0*PI+angle));
      assert_eq!(true, eq(r1.length, 100.0));

   }
   #[test]
   fn test_vector() {
      // Q1
      let p = Point::new(-1.0, -1.0);
      let angle = 45.0*PI/180.0;
      let r1 = HTangent::new(p, angle, 100.0);
      let v = r1.vector();
      assert_eq!(true, eq(v.length(), 1.0));
      assert_eq!(true, eq(v.x, angle.cos()));
      assert_eq!(true, eq(v.y, angle.sin()));
      // Q2
      let angle = 120.0*PI/180.0;
      let r1 = HTangent::new(p, angle, 100.0);
      let v = r1.vector();
      assert_eq!(true, eq(v.length(), 1.0));
      assert_eq!(true, (v.x+(30.0*PI/180.0).sin()).abs()<0.001);
      assert_eq!(true, (v.y-(30.0*PI/180.0).cos()).abs()<0.001);
      //Q3
      let angle = 210.0*PI/180.0;
      let r1 = HTangent::new(p, angle, 100.0);
      let v = r1.vector();
      assert_eq!(true, eq(v.length(), 1.0));
      assert_eq!(true, (v.x+(30.0*PI/180.0).cos()).abs()<0.001);
      assert_eq!(true, (v.y+(30.0*PI/180.0).sin()).abs()<0.001);
      // Q4
      let angle = 300.0*PI/180.0;
      let r1 = HTangent::new(p, angle, 100.0);
      let v = r1.vector();
      println!("{} {}", angle.cos(), angle.sin());
      println!("{} {}", v.x, v.y);
      assert_eq!(true, eq(v.length(), 1.0));
      assert_eq!(true, (v.x-(30.0*PI/180.0).sin()).abs()<0.001);
      assert_eq!(true, (v.y+(30.0*PI/180.0).cos()).abs()<0.001);

      
   }
   #[test]
   fn test_end_point() {
      let p1 = Point::new(0.0, 0.0);
      let angle = 0.0;
      let length = 100.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(r1.end_point().x, 100.0);
      assert_eq!(r1.end_point().y, 0.0);
      let angle = 30.0*PI/180.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(r1.end_point().x, 100.0*angle.cos());
      assert_eq!(true, (r1.end_point().y - 50.0).abs()<0.001);
      let angle = 120.0*PI/180.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(true, (r1.end_point().x+50.0).abs()<0.001);
      assert_eq!(true, (r1.end_point().y - 100.0*angle.sin()).abs()<0.001);
      let angle = 210.0*PI/180.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(true, (r1.end_point().x-100.0*angle.cos()).abs()<0.001);
      assert_eq!(true, (r1.end_point().y + 50.0).abs()<0.001);
      let angle = 300.0*PI/180.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(true, (r1.end_point().x - 50.0).abs()<0.001);
      assert_eq!(true, (r1.end_point().y-100.0*angle.sin()).abs()<0.001);      
      let angle = PI/2.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(true, (r1.end_point().x).abs()< 0.001);
      assert_eq!(true, (r1.end_point().y - 100.0).abs()<0.001);      
      let angle = PI;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(true, (r1.end_point().x+100.0).abs()<0.001);
      assert_eq!(true, (r1.end_point().y).abs()<0.001);      
      let angle = 3.0*PI/2.0;
      let r1 = HTangent::new(p1, angle, length);
      assert_eq!(true, (r1.end_point().x).abs()<0.001);
      assert_eq!(true, (r1.end_point().y+100.0).abs()<0.001);
      let angle = 2.0*PI;
      let r1 = HTangent::new(p1, angle, length);
      println!("az={}", r1.azimuth); // Check that 2PI changes to 0.0
      assert_eq!(r1.end_point().x, 100.0);
      assert_eq!(true, (r1.end_point().y).abs()<0.001);            
   }
   #[test]
   fn test_point_at_s() {
      let p = Point::new(0.0, 0.0);
      let v = Vector::new(1.0, 0.0);
      let r = HTangent::new(p, v.azimuth(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(5.0, q.x));
      assert_eq!(true, eq001(0.0, q.y));
      //
      let v = Vector::new(0.0, -1.0);
      let r = HTangent::new(p, v.azimuth(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(0.0, q.x));
      assert_eq!(true, eq001(-5.0, q.y));
      //
      let v = Vector::new(-1.0, -1.0);
      println!("{}", v.azimuth());
      let r = HTangent::new(p, v.azimuth(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(-5.0*(PI/4.0).cos(), q.x));
      assert_eq!(true, eq001(-5.0*(PI/4.0).sin(), q.y));      
      //
      let v = Vector::new(-1.0, 1.0);
      println!("{}", v.azimuth());
      let r = HTangent::new(p, v.azimuth(), 10.0);
      let q = r.point_at_s(5.0);
      assert_eq!(true, eq001(-5.0*(PI/4.0).cos(), q.x));
      assert_eq!(true, eq001(5.0*(PI/4.0).sin(), q.y));      
   }
   #[test]
   #[should_panic]
   fn test_point_at_s_panic_1() {
      let p = Point::new(0.0, 0.0);
      let v = Vector::new(1.0, 0.0);
      let r = HTangent::new(p, v.azimuth(), 10.0);
      let _q = r.point_at_s(15.0);
   }
   #[test]
   #[should_panic]
   fn test_point_at_s_panic_2() {
      let p = Point::new(0.0, 0.0);
      let v = Vector::new(1.0, 0.0);
      let r = HTangent::new(p, v.azimuth(), 10.0);
      let _q = r.point_at_s(-5.0);
   }

}
