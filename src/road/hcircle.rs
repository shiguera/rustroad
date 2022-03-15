use crate::{normalize_360, azimuth_to_angle, angle_to_azimuth};
use crate::geom::point::Point;
use crate::geom::vector::Vector;
use super::hsection::HSection;
use std::f64::consts::PI;

/// It represents a horizontal circular curve section of a road.  
/// It is defined through the start point, the start azimuth,
/// the radius and the section's length.
/// If the radius is positive, the curve is rightward, else
/// it is leftward
pub struct HCircle {
   pub start_point: Point,
   pub start_azimuth: f64,
   pub radius: f64,
   pub length: f64
}

impl HCircle {
   pub fn new(start_point: Point, start_azimuth: f64, radius: f64, length: f64) -> Self {
      if crate::eq001(radius, 0.0) || length < 0.0_f64 {
         panic!("Radius zero or negative length");
      }
      let az = normalize_360(start_azimuth);
      HCircle{start_point, start_azimuth: az, radius, length}
   }
   pub fn center(&self) -> Point {
      let angle = azimuth_to_angle(self.start_azimuth());
      let direction = Vector::from_angle(angle);
      let normal: Vector;
      
      if self.radius > 0.0 {
         normal = direction.right_normal_vector();
      } else {
         normal = direction.left_normal_vector();
      }
      let distance = self.radius.abs();
      let x = self.start_x() + distance * normal.angle().cos();
      let y = self.start_y() + distance * normal.angle().sin();
      Point::new(x, y)
   }
}
impl HSection for HCircle {
   fn start_point(&self) -> Point {
      self.start_point
   }
   fn end_point(&self) -> Point {
      self.point_at_s(self.length())
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
      if s>(2.0*PI*self.radius).abs() || s<0.0 {
         panic!("Arc greater than circle length or negative");
      }
      let start_angle = azimuth_to_angle(self.start_azimuth());
      //println!("start_angle:{}", start_angle);
      let inc_angle = - s / self.radius;
      //println!("inc_angle:{}", inc_angle); 
      // if radius>0 => inc_angle is negative
      let end_angle = start_angle + inc_angle;
      //println!("end_angle:{}", end_angle);      
      angle_to_azimuth(end_angle)
   }
   fn point_at_s(&self, s:f64) -> Point {
      // Azimuth at point s
      let az = self.azimuth_at_s(s);
      let v = Vector::from_angle(azimuth_to_angle(az));
      let normal: Vector;
      // normal is the vector from center to point at s
      if self.radius > 0.0 {
         normal = v.left_normal_vector();
      } else {
         normal = v.right_normal_vector();
      }
      let direction = normal.angle();
      let x = self.center().x + self.radius.abs()*direction.cos();
      let y = self.center().y + self.radius.abs()*direction.sin();
      Point::new(x, y)      
   }
}

#[cfg(test)]
mod tests {
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
   /// tests HCircle::new() and some other methods of trait HSection
   fn test_new() {
      let p = Point::new(0.0, 0.0);
      let az = 0.0;
      let radius = 400.0;
      let length = 300.0;
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(0.0, c.start_point.x));
      assert!(eq001(0.0, c.start_x()));
      assert!(eq001(0.0, c.start_point.y));
      assert!(eq001(0.0, c.start_y()));
      assert!(eq001(0.0, c.start_azimuth));
      assert!(eq001(0.0, c.start_azimuth()));
      assert!(eq001(400.0, c.radius));
      assert!(eq001(400.0, c.start_radius()));
      assert!(eq001(400.0, c.end_radius()));      
      assert!(eq001(300.0, c.length));
   }
   #[test]
   fn test_end_azimuth() {
      // negative radius
      let p = Point::new(0.0, 400.0);
      let az = 270.0;
      let radius = -400.0;
      let length = (PI*radius/2.0).abs();
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(180.0, c.end_azimuth()));
      // positive radius
      let radius = 400.0;
      let az = 90.0;
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(180.0, c.end_azimuth()));
      // End azimuth negative
      let p = Point::new(400.0, 0.0);
      let az = 0.0;
      let radius = -400.0;
      let length = (PI*radius).abs();
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(180.0, c.end_azimuth()));
      // End azimuth greater then 2PI
      let p = Point::new(-400.0, 0.0);
      let az = 0.0;
      let radius = 400.0;
      let length = (PI*radius).abs();
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(180.0, c.end_azimuth()));
   }
   #[test]
   #[should_panic]
   fn test_panic_in_azimuth_at_s() {
      // panic if s length grater than circle length
      let p = Point::new(400.0, 0.0);
      let angle = 90.0;
      let radius = 400.0;
      // positive radius
      let c = HCircle::new(p, angle, radius, PI*radius/2.0);
      let _az = c.azimuth_at_s(2.01*PI*radius);
   }
   #[test]
   #[should_panic]
   fn test_panic2_in_azimuth_at_s() {
      // panic if s is negative
      let p = Point::new(400.0, 0.0);
      let az = 90.0;
      let radius = 400.0;
      let length = (PI*radius/4.0).abs();
      // positive radius
      let c = HCircle::new(p, az, radius, length);
      let _az = c.azimuth_at_s(-100.0);
   }
   #[test]
   fn test_azimuth_at_s() {
      // positive radius
      let p = Point::new(400.0, 0.0);
      let az = 180.0;
      let radius = 400.0;
      let length = (PI*radius/2.0).abs();
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(c.azimuth_at_s(length), 270.0));
      // negative radius
      let p = Point::new(-400.0, 0.0);
      let az = 90.0;
      let radius = -400.0;
      let length = (PI*radius/2.0).abs();
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(c.azimuth_at_s(length), 0.0));
   }
   #[test]
   fn test_center() {
      // positive radius
      let p = Point::new(0.0, 400.0);
      let az = 90.0;
      let radius = 400.0;
      let length = PI*radius/2.0;
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(c.center().x, 0.0));
      assert!(eq001(c.center().y, 0.0));
      // negative radius
      let p = Point::new(0.0, 400.0);
      let az = 270.0;
      let radius = -400.0;
      let length = (PI*radius/2.0).abs();
      let c = HCircle::new(p, az, radius, length);
      assert!(eq001(c.center().x, 0.0));
      assert!(eq001(c.center().y, 0.0));
   }
   #[test]
   fn test_point_at_s() {
      // Q1, R>0
      let start_pt = Point::new(0.0, 400.0);
      let start_az = 90.0;
      let radius = 400.0;
      let length = (PI/2.0)*radius;
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(400.0, 0.0);
      assert!(eq001(end_pt.x, circle.point_at_s(length).x));
      assert!(eq001(end_pt.y, circle.point_at_s(length).y));
      // Q1, R<0
      let start_pt = Point::new(400.0, 0.0);
      let start_az = 0.0;
      let radius = -400.0;
      let length = (PI/2.0*radius).abs();
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(0.0, 400.0);
      assert!(eq001(end_pt.x, circle.point_at_s(length).x));
      assert!(eq001(end_pt.y, circle.point_at_s(length).y));
      // Q4, R>0
      let start_pt = Point::new(-400.0, 0.0);
      let start_az = 0.0;
      let radius = 400.0;
      let length = (PI/2.0)*radius;
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(0.0, 400.0);
      assert!(eq001(end_pt.x, circle.point_at_s(length).x));
      assert!(eq001(end_pt.y, circle.point_at_s(length).y));
      // Q3, R<0
      let start_pt = Point::new(0.0, -400.0);
      let start_az = 90.0;
      let radius = -400.0;
      let length = (PI/2.0*radius).abs();
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(400.0, 0.0);
      println!("{} {}", circle.point_at_s(length).x, circle.point_at_s(length).y);
      assert!(eq001(end_pt.x, circle.point_at_s(length).x));
      assert!(eq001(end_pt.y, circle.point_at_s(length).y));
   }
   #[test]
   fn test_end_point() {
      // Q1, R>0
      let start_pt = Point::new(0.0, 400.0);
      let start_az = 90.0;
      let radius = 400.0;
      let length = (PI/2.0*radius).abs();
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(400.0, 0.0);
      assert!(eq001(end_pt.x, circle.end_point().x));
      assert!(eq001(end_pt.y, circle.end_point().y));
      // Q1, R<0
      let start_pt = Point::new(400.0, 0.0);
      let start_az = 0.0;
      let radius = -400.0;
      let length = (PI/2.0*radius).abs();
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(0.0, 400.0);
      assert!(eq001(end_pt.x, circle.end_point().x));
      assert!(eq001(end_pt.y, circle.end_point().y));
      // Q4, R>0
      let start_pt = Point::new(-400.0, 0.0);
      let start_az = 0.0;
      let radius = 400.0;
      let length = (PI/2.0*radius).abs();
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(0.0, 400.0);
      assert!(eq001(end_pt.x, circle.end_point().x));
      assert!(eq001(end_pt.y, circle.end_point().y));
      // Q3, R<0
      let start_pt = Point::new(-400.0, 0.0);
      let start_az = 180.0;
      let radius = -400.0;
      let length = (PI/2.0*radius).abs();
      let circle = HCircle::new(start_pt, start_az, radius, length);
      let end_pt = Point::new(0.0, -400.0);
      assert!(eq001(end_pt.x, circle.end_point().x));
      assert!(eq001(end_pt.y, circle.end_point().y));
   }

}