use crate::geom::point::Point;
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
      HCircle{start_point, start_azimuth, radius, length}
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
      let angle = self.length() / self.radius;
      let mut end_azimuth = self.start_azimuth + angle;
      if end_azimuth > 2.0*PI {
         end_azimuth = end_azimuth - 2.0*PI;
      }
      if end_azimuth < 0.0 {
         end_azimuth = 2.0*PI + end_azimuth;
      }
      end_azimuth
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
}