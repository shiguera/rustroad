use crate::geom::point::Point;
use crate::geom::vector::Vector;
use super::hsection::HSection;
use std::f64::consts::PI;

pub struct HCircle {
   pub start_point: Point,
   pub start_azimuth: f64,
   pub radius: f64,
   pub length: f64
}

impl HCircle {
   pub fn new(start_point: Point, start_azimuth: f64, radius: f64, length: f64) -> Self {
      if radius == 0.0_f64 || length < 0.0_f64 {
         panic!("Radius zero or negative length");
      }
      let mut angle = start_azimuth;
      if angle < 0.0 {
         angle = 2.0*PI + angle;
      }
      HCircle{start_point, start_azimuth: angle, radius, length}
   }
   pub fn center(&self) -> Point {
      let mut direction = Vector::from_azimuth(self.start_azimuth());
      if self.radius > 0.0 {
         direction = direction.left_normal_vector();
      } else {
         direction = direction.right_normal_vector();
      }
      let distance = self.radius.abs();
      let x = self.start_x() + distance * direction.azimuth().cos();
      let y = self.start_y() + distance * direction.azimuth().sin();
      Point::new(x, y)
   }
}
impl HSection for HCircle {
   fn start_point(&self) -> Point {
      self.start_point
   }
   fn end_point(&self) -> Point {
      todo!();
   }
   fn start_radius(&self) -> f64 {
      self.radius
   }
   fn end_radius(&self) -> f64 {
      self.radius
   }
   fn length(&self) -> f64 {
      self.length
   }
   fn start_azimuth(&self) -> f64 {
      self.start_azimuth
   }
   fn end_azimuth(&self) -> f64 {
      self.azimuth_at_s(self.length())
   }
   fn azimuth_at_s(&self, s:f64) -> f64 {
      if s > self.length() {
         panic!("s is larger than length");
      }
      let angle = s / self.radius;
      let mut s_azimuth = self.start_azimuth + angle;
      if s_azimuth > 2.0*PI {
         s_azimuth = s_azimuth - 2.0*PI;
      }
      if s_azimuth < 0.0 {
         s_azimuth = 2.0*PI + s_azimuth;
      }
      s_azimuth
   }
   fn point_at_s(&self, _s:f64) -> Point {
      todo!()
   }
}

mod tests {
   #[cfg(test)]
   use super::*;
   #[cfg(test)]
   use crate::eq001;   

   #[test]
   #[should_panic]
   fn test_new_panic_1() {
      // radius zero not allowed
      HCircle::new(Point::new(0.0, 0.0), 0.0, 0.0, 100.0);
   }
   #[test]
   #[should_panic]
   fn test_new_panic_2() {
      // negative length not allowed
      HCircle::new(Point::new(0.0, 0.0), 0.0, 100.0, -100.0);
   }

   #[test]
   fn test_new() {
      // tests new and some functions of trait HSection
      let c = HCircle::new(Point::new(0.0, 0.0), 0.0, 400.0, 300.0);
      assert_eq!(true, eq001(0.0, c.start_point.x));
      assert_eq!(true, eq001(0.0, c.start_x()));
      assert_eq!(true, eq001(0.0, c.start_point.y));
      assert_eq!(true, eq001(0.0, c.start_y()));
      assert_eq!(true, eq001(0.0, c.start_azimuth));
      assert_eq!(true, eq001(0.0, c.start_azimuth()));
      assert_eq!(true, eq001(400.0, c.radius));
      assert_eq!(true, eq001(400.0, c.start_radius()));
      assert_eq!(true, eq001(400.0, c.end_radius()));      
      assert_eq!(true, eq001(300.0, c.length));
      assert_eq!(true, eq001(300.0, c.length()));
   }
   #[test]
   fn test_end_azimuth() {
      let p = Point::new(0.0, 0.0);
      let angle = PI/4.0;
      // positive radius
      let c = HCircle::new(p, angle, 400.0, 300.0);
      assert_eq!(true, eq001(1.5354, c.end_azimuth()));
      // negative radius
      let c = HCircle::new(p, angle, -400.0, 300.0);
      assert_eq!(true, eq001(0.0354, c.end_azimuth()));
      // End azimuth negative
      let angle = PI/8.0;
      let c = HCircle::new(p, angle, -400.0, 300.0);
      assert_eq!(true, eq001(5.9259, c.end_azimuth()));
      // End azimuth greater then 2PI
      let angle = 5.9259;
      let c = HCircle::new(p, angle, 400.0, 300.0);
      assert_eq!(true, eq001(0.3927, c.end_azimuth()));
   }
   #[test]
   #[should_panic]
   fn test_azimuth_at_s_panic() {
      let p = Point::new(400.0, 0.0);
      let angle = PI/2.0;
      // positive radius
      let c = HCircle::new(p, angle, 400.0, 628.3185);
      let _az = c.azimuth_at_s(1000.0);
   }
   #[test]
   fn test_azimuth_at_s() {
      let p = Point::new(400.0, 0.0);
      let angle = PI/2.0;
      // positive radius
      let c = HCircle::new(p, angle, 400.0, 628.3185);
      assert_eq!(true, eq001(2.3562, c.azimuth_at_s(314.1593)));
      // negative radius
      let angle = -PI/2.0;
      let c = HCircle::new(p, angle, -400.0, 628.3185);
      assert_eq!(true, eq001(3.9270, c.azimuth_at_s(314.1593)));
      // End azimuth negative
      let p = Point::new(282.8427, 282.8427);
      let angle = -PI/4.0;
      let c = HCircle::new(p, angle, -400.0, 942.4778);
      assert_eq!(true, eq001(3.9270, c.azimuth_at_s(628.3185)));
      // End azimuth greater then 2PI
      let p = Point::new(0.0, -400.0);
      let angle = 0.0;
      let c = HCircle::new(p, angle, 400.0, 1256.6371);
      assert_eq!(true, eq001(2.3562, c.azimuth_at_s(942.4778)));
   }
   #[test]
   fn test_center() {
      let c = Point::new(0.0, 0.0);
      let r = 1.0;
      // positive radius
      let p = Point::new(0.0, -1.0);
      let hc = HCircle::new(p, 0.0, r, 3.0);
      let center = hc.center();
      assert_eq!(true, eq001(c.x, center.x));
      assert_eq!(true, eq001(c.y, center.y));
      // negative radius
      let p = Point::new(-1.0, 0.0);
      let hc = HCircle::new(p, PI/2.0, -r, 3.0);
      let center = hc.center();
      assert_eq!(true, eq001(c.x, center.x));
      assert_eq!(true, eq001(c.y, center.y));
   }
}