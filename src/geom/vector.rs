use std::f64::consts::PI;
use crate::*;

// 2 D vector
#[derive(Clone, Copy, Debug)]
pub struct Vector {
   pub vx: f64,
   pub vy: f64
}

impl Vector {
   /// Creates a new vector
   pub fn new(vx: f64, vy: f64) -> Self {
      Vector{vx,vy}
   }
   /// Build an unity Vector from an angle.
   /// Angle is measured in radians from east leftward
   pub fn from_angle(angle: f64) -> Self {
      Vector::new(angle.cos(), angle.sin())
   }
   /// Calculates the length of the Vector
   pub fn length(&self) -> f64 {
      ((self.vx*self.vx)+(self.vy*self.vy)).sqrt()
   }
   /// It returns a new Vector with same direction and length 1
   pub fn unit_vector(&self) -> Self {
      let length = self.length();
      if eq001(length, 0.0) {
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
   pub fn angle(&self) -> f64 {
      let result:f64;
      if eq001(self.vx, 0.0) && eq001(self.vy, 0.0) {
         result = 0.0_f64;         
      } else {
         if eq001(self.vx, 0.0) {
            if self.vy > 0.0 {
               result=PI/2.0;
            } else {
               result = 3.0*PI/2.0;
            } 
         } else if eq001(self.vy, 0.0) {
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

#[cfg(test)]
mod tests {
   use super::*;
   
   #[test]
   fn test_new() {
      let v = Vector::new(1.0,-1.0);
      assert!(eq001(v.vx, 1.0));
      assert!(eq001(v.vy, -1.0));
   }
   #[test] 
   fn test_from_angle() {
      let angle = PI/8.0;
      let v = Vector::from_angle(angle);
      assert!(eq001(0.9239, v.vx));
      assert!(eq001(0.3827, v.vy));
   }
   #[test]
   fn test_length() {
      let v = Vector::new(1.0,-1.0);
      assert!(eq001(v.length(), 2.0f64.sqrt()));
      assert!(eq001(v.unit_vector().length(), 1.0));
      let v = Vector::new(0.0,0.0);
      assert!(eq001(v.length(), 0.0_f64));   
   }
   #[test]
   fn test_unit_vector() {
      let v = Vector::new(1.0,-1.0);
      assert!(eq001(v.unit_vector().length(), 1.0));
      assert!(eq001(v.unit_vector().vx, 0.7071));
      assert!(eq001(v.unit_vector().vy, -0.7071)); 
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
      assert!(eq001(w.vx, 0.0));
      assert!(eq001(w.vy, 0.0));
      let v = Vector::new(1.0, 0.0);
      let w = v.left_normal_vector();
      assert!(eq001(w.vx, 0.0));
      assert!(eq001(w.vy, 1.0));
      let w = v.right_normal_vector();
      assert!(eq001(w.vx, 0.0));
      assert!(eq001(w.vy, -1.0));
      let v = Vector::new(0.0, 1.0);
      let w = v.left_normal_vector();
      assert!(eq001(w.vx, -1.0));
      assert!(eq001(w.vy, 0.0));
      let w = v.right_normal_vector();
      assert!(eq001(w.vx, 1.0));
      assert!(eq001(w.vy, 0.0));
      let v = Vector::new(1.0, 1.0);
      let w = v.left_normal_vector();
      assert!(eq001(w.vx, -1.0));
      assert!(eq001(w.vy, 1.0));
      let w = v.right_normal_vector();
      assert!(eq001(w.vx, 1.0));
      assert!(eq001(w.vy, -1.0));
   }
   #[test]
   fn test_angle() {
      let v = Vector::new(0.0, 0.0);
      assert_eq!(true, v.angle() == 0.0);
      let v = Vector::new(1.0, 0.0);
      assert_eq!(true, v.angle() == 0.0);
      let v = Vector::new(-1.0, 0.0);
      assert_eq!(true, v.angle() == PI);
      let v = Vector::new(0.0, 1.0);
      assert_eq!(true, v.angle() == PI/2.0);
      let v = Vector::new(0.0, -1.0);
      assert_eq!(true, v.angle() == 3.0*PI/2.0);
      let v = Vector::new(1.0, 1.0);
      assert_eq!(true, v.angle() == PI/4.0);
      let v = Vector::new(1.0, -1.0);
      assert_eq!(true, v.angle() == 2.0*PI - PI/4.0);
      let v = Vector::new(-1.0, 1.0);
      assert_eq!(true, v.angle() == PI/2.0 + PI/4.0);
      let v = Vector::new(-1.0, -1.0);
      assert_eq!(true, v.angle() == PI + PI/4.0);
      // The following test fails if not uses approx_eq!(), 
      // caused for the problem with ==
      let v = Vector::new(0.15*6.0+0.10, 1.0);
      println!("{} {}", v.angle(), PI/4.0);
      assert_eq!(true, eq(v.angle(), PI/4.0));
   }
}
