use std::f64::consts::PI;
use crate::*;

// 2 D vector
#[derive(Clone, Copy, Debug)]
pub struct Vector {
   pub x: f64,
   pub y: f64
}

impl Vector {
   pub fn new(x: f64, y: f64) -> Self {
      Vector{x,y}
   }
   // Build a Vector from an azimuth angle.
   // Azimuth is measured from east leftward
   pub fn from_azimuth(azimuth: f64) -> Self {
      todo!("Azimuth must be done from sud");
      Vector::new(azimuth.cos(), azimuth.sin())
   }
   pub fn length(self) -> f64 {
      ((self.x*self.x)+(self.y*self.y)).sqrt()
   }
   pub fn unit_vector(self) -> Self {
      // Vector with same direction and length 1
      let module = self.length();
      if eq(module, 0.0) {
         panic!("Trying unit_vector() of vector with length() zero")
      }
      Vector::new(self.x/module, self.y/module)
   }
   pub fn left_normal_vector(&self) -> Self {
      // Perpendicular vector toward the left side (counterclock-wise)
      Vector::new(-self.y, self.x)
   }
   pub fn right_normal_vector(self) -> Self {
      // Perpendicular vector toward the right side (clock-wise)
      Vector::new(self.y, -self.x)
   }
   pub fn azimuth(self) -> f64 {
      // Angle with X axis in radians
      // East==Positive X axis is the origin of angles
      // Counterclockwise is the direction 
      //
      // TODO: Make comparisons with abs(minvalue), not with ==
      let result:f64;
      if eq(self.x, 0.0) && eq(self.y, 0.0) {
         result = 0.0_f64;         
      } else {
         if eq(self.x, 0.0) {
            if self.y > 0.0 {
               result=PI/2.0;
            } else {
               result = 3.0*PI/2.0;
            } 
         } else if eq(self.y, 0.0) {
            if self.x < 0.0 {
               result = PI;
            } else {
               result = 0.0;
            }
         } else {
            let tangent = self.y/self.x;
            let angle = tangent.atan();
            if angle > 0.0 {
               if self.x<0.0 && self.y<0.0 {
                  result = PI + angle;
               } else {
                  result = angle;
               }
            } else {
               if self.x > 0.0 {
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
      assert_eq!(true, v.x-1.0 < 0.001);
      assert_eq!(true, v.y-(-1.0) < 0.001);
   }
   #[test] 
   fn test_from_azimuth() {
      let azimuth = PI/8.0;
      let v = Vector::from_azimuth(azimuth);
      assert_eq!(true, eq001(0.9239, v.x));
      assert_eq!(true, eq001(0.3827, v.y));
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
      assert_eq!(true, v.unit_vector().x - 0.7071 < 0.0001);
      assert_eq!(true, v.unit_vector().y - (-0.7071) < 0.0001); 
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
      assert_eq!(true, approx_eq!(f64, w.x, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.y, 0.0, ulps=2));
      let v = Vector::new(1.0, 0.0);
      let w = v.left_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.x, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.y, 1.0, ulps=2));
      let w = v.right_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.x, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.y, -1.0, ulps=2));
      let v = Vector::new(0.0, 1.0);
      let w = v.left_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.x, -1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.y, 0.0, ulps=2));
      let w = v.right_normal_vector();
      assert_eq!(true, approx_eq!(f64, w.x, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, w.y, 0.0, ulps=2));
      let v = Vector::new(1.0, 1.0);
      let w = v.left_normal_vector();
      assert_eq!(true, eq(w.x, -1.0));
      assert_eq!(true, eq(w.y, 1.0));
      let w = v.right_normal_vector();
      assert_eq!(true, eq(w.x, 1.0));
      assert_eq!(true, eq(w.y, -1.0));
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
      let v = Vector{x:2.0, y:1.0};
      let w = v.left_normal_vector().unit_vector();
      println!("{} {}", w.x, -1.0/5.0_f64.sqrt());
      println!("{} {}", w.y, 2.0/5.0_f64.sqrt());
      assert_eq!(true, eq(w.x, -1.0/5.0_f64.sqrt()));
      assert_eq!(true, eq(w.y, 2.0/5.0_f64.sqrt()));
   }
}
