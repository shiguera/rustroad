use std::f64::consts::PI;
use crate::*;

// 2 D vector
#[derive(Clone, Copy, Debug)]
pub struct Vector {
   pub vx: f64,
   pub vy: f64
}

impl Vector {
   pub fn new(vx: f64, vy: f64) -> Self {
      Vector{vx,vy}
   }
   /// Build an unity Vector from an angle.
   /// Angle is measured from east leftward
   pub fn from_angle(angle: f64) -> Self {
      Vector::new(angle.cos(), angle.sin())
   }
   pub fn length(&self) -> f64 {
      ((self.vx*self.vx)+(self.vy*self.vy)).sqrt()
   }
   pub fn unit_vector(&self) -> Self {
      // Vector with same direction and length 1
      let length = self.length();
      if eq(length, 0.0) {
         panic!("Trying unit_vector() of vector with length() zero")
      }
      Vector::new(self.vx/length, self.vy/length)
   }
   ///
   /// Perpendicular vector toward the left side (counterclock-wise)
   pub fn left_normal_vector(&self) -> Self {
      Vector::new(-self.vy, self.vx)
   }
   ///
   /// Perpendicular vector toward the right side (clock-wise)
   pub fn right_normal_vector(&self) -> Self {
      Vector::new(self.vy, -self.vx)
   }
   /// Angle in radians with X axis 
   /// East==Positive X axis is the origin of angles
   /// Counterclockwise is the direction 
   ///
   /// TODO: Make comparisons with abs(minvalue), not with ==
   pub fn azimuth(&self) -> f64 {
      let result:f64;
      if eq(self.vx, 0.0) && eq(self.vy, 0.0) {
         result = 0.0_f64;         
      } else {
         if eq(self.vx, 0.0) {
            if self.vy > 0.0 {
               result=PI/2.0;
            } else {
               result = 3.0*PI/2.0;
            } 
         } else if eq(self.vy, 0.0) {
            if self.vx < 0.0 {
               result = PI;
            } else {
               result = 0.0;
            }
         } else {
            let tangent = self.vy/self.vx;
            let angle = tangent.atan();
            if angle > 0.0 {
               if self.vx<0.0 && self.vy<0.0 {
                  result = PI + angle;
               } else {
                  result = angle;
               }
            } else {
               if self.vx > 0.0 {
                  result = 2.0*PI - (-angle);
               } else {
                  result = PI - (-angle);
               }
            }
         }
      }
      result
   }
}

mod tests {
   #[cfg(test)]
   use super::*;
   
   #[test]
   fn test_new() {
      let v = Vector::new(1.0,-1.0);
      assert_eq!(true, v.vx-1.0 < 0.001);
      assert_eq!(true, v.vy-(-1.0) < 0.001);
   }
   #[test] 
   fn test_from_angle() {
      let angle = PI/8.0;
      let v = Vector::from_angle(angle);
      assert_eq!(true, eq001(0.9239, v.vx));
      assert_eq!(true, eq001(0.3827, v.vy));
   }
   #[test]
   fn test_length() {
      let v = Vector::new(1.0,-1.0);
      assert_eq!(true, v.length()-2.0_f64.sqrt() < 0.001);
      assert_eq!(true, eq(v.unit_vector().length(), 1.0));
      let v = Vector::new(0.0,0.0);
      assert_eq!(true, v.length()-0.0_f64 < 0.001);   
   }
   #[test]
   fn test_unit_vector() {
      let v = Vector::new(1.0,-1.0);
      assert_eq!(true, v.unit_vector().length()-1.0_f64 < 0.001);
      assert_eq!(true, v.unit_vector().vx - 0.7071 < 0.0001);
      assert_eq!(true, v.unit_vector().vy - (-0.7071) < 0.0001); 
   }
   #[test]
   #[should_panic]
   fn test_unit_vector_with_length_zero() {
      let v = Vector::new(0.0, 0.0);
      let _w = v.unit_vector();
   }
   #[test]
   fn test_normal_vector() {
      // Tests left_normal_vector() and right_normal_vector()
      let v = Vector::new(0.0, 0.0);
      let w = v.left_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.vx, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.vy, 0.0, ulps=2));
      let v = Vector::new(1.0, 0.0);
      let w = v.left_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.vx, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.vy, 1.0, ulps=2));
      let w = v.right_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.vx, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.vy, -1.0, ulps=2));
      let v = Vector::new(0.0, 1.0);
      let w = v.left_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.vx, -1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.vy, 0.0, ulps=2));
      let w = v.right_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.vx, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.vy, 0.0, ulps=2));
      let v = Vector::new(1.0, 1.0);
      let w = v.left_normal_vector();
      assert_eq!(true, eq(w.vx, -1.0));
      assert_eq!(true, eq(w.vy, 1.0));
      let w = v.right_normal_vector();
      assert_eq!(true, eq(w.vx, 1.0));
      assert_eq!(true, eq(w.vy, -1.0));
   }
   #[test]
   fn test_azimuth() {
      let v = Vector::new(0.0, 0.0);
      assert_eq!(true, v.azimuth() == 0.0);
      let v = Vector::new(1.0, 0.0);
      assert_eq!(true, v.azimuth() == 0.0);
      let v = Vector::new(-1.0, 0.0);
      assert_eq!(true, v.azimuth() == PI);
      let v = Vector::new(0.0, 1.0);
      assert_eq!(true, v.azimuth() == PI/2.0);
      let v = Vector::new(0.0, -1.0);
      assert_eq!(true, v.azimuth() == 3.0*PI/2.0);
      let v = Vector::new(1.0, 1.0);
      assert_eq!(true, v.azimuth() == PI/4.0);
      let v = Vector::new(1.0, -1.0);
      assert_eq!(true, v.azimuth() == 2.0*PI - PI/4.0);
      let v = Vector::new(-1.0, 1.0);
      assert_eq!(true, v.azimuth() == PI/2.0 + PI/4.0);
      let v = Vector::new(-1.0, -1.0);
      assert_eq!(true, v.azimuth() == PI + PI/4.0);
      // The following test fails if not uses approx_eq!(), 
      // caused for the problem with ==
      let v = Vector::new(0.15*6.0+0.10, 1.0);
      println!("{} {}", v.azimuth(), PI/4.0);
      assert_eq!(true, eq(v.azimuth(), PI/4.0));
   }
   #[test]
   fn test_1() {
      let a: f64 = 0.15 + 0.15 + 0.15;
      let b: f64 = 0.1 + 0.1 + 0.25;
      println!("{} == {}", a, b);
      println!("{}", a==b);  // Fails, because they are not exactly equal
      println!("{}", f64::MIN);
      println!("{}", f64::EPSILON);
      let v = Vector{vx:2.0, vy:1.0};
      let w = v.left_normal_vector().unit_vector();
      println!("{} {}", w.vx, -1.0/5.0_f64.sqrt());
      println!("{} {}", w.vy, 2.0/5.0_f64.sqrt());
      assert_eq!(true, eq(w.vx, -1.0/5.0_f64.sqrt()));
      assert_eq!(true, eq(w.vy, 2.0/5.0_f64.sqrt()));
   }
}
