use std::f64::consts::PI;
// 2 D vector
pub struct Vector {
   pub x: f64,
   pub y: f64
}

impl Vector {
   pub fn new(x: f64, y: f64) -> Self {
      Vector{x,y}
   }
   pub fn length(&self) -> f64 {
      ((self.x*self.x)+(self.y*self.y)).sqrt()
   }
   pub fn unit_vector(&self) -> Self {
      let module = self.length();
      Vector::new(self.x/module, self.y/module)
   }
   pub fn bearing(&self) -> f64 {
      // Angle with X axis in radians
      // East==Positive X axis is the origin of angles
      // Counterclockwise is the direction 
      //
      // TODO: Meke comparisons with abs(minvalue), not with ==
      let result:f64;
      if self.x==0.0 && self.y==0.0 {
         result = 0.0_f64;         
      } else {
         if self.x == 0.0 {
            if self.y > 0.0 {
               result=PI/2.0;
            } else {
               result = 3.0*PI/2.0;
            } 
         } else if self.y == 0.0 {
            if self.x < 0.0 {
               result = PI;
            } else {
               result = 0.0;
            }
         } else {
            let tangent = self.y/self.x;
            let angle = tangent.atan();
            if angle > 0.0 {
               if self.x > 0.0 {
                  result = angle;
               } else {
                  result = PI + (- angle);
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
   fn test_length() {
      let v = Vector::new(1.0,-1.0);
      assert_eq!(true, v.length()-2.0_f64.sqrt() < 0.001);
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
   fn test_bearing() {
      let v = Vector::new(0.0, 0.0);
      assert_eq!(true, v.bearing() == 0.0);
      let v = Vector::new(1.0, 0.0);
      assert_eq!(true, v.bearing() == 0.0);
      let v = Vector::new(-1.0, 0.0);
      assert_eq!(true, v.bearing() == PI);
      let v = Vector::new(0.0, 1.0);
      assert_eq!(true, v.bearing() == PI/2.0);
      let v = Vector::new(0.0, -1.0);
      assert_eq!(true, v.bearing() == 3.0*PI/2.0);
      let v = Vector::new(1.0, 1.0);
      assert_eq!(true, v.bearing() == PI/4.0);
      let v = Vector::new(1.0, -1.0);
      assert_eq!(true, v.bearing() == 2.0*PI - PI/4.0);
      let v = Vector::new(-1.0, 1.0);
      assert_eq!(true, v.bearing() == PI/2.0 + PI/4.0);
      let v = Vector::new(-1.0, -1.0);
      println!("{} {}", v.bearing(), PI - PI/4.0);
      assert_eq!(true, v.bearing() == PI - PI/4.0);
 
   }
   #[test]
   fn test_1() {
      let a: f64 = 0.15 + 0.15 + 0.15;
      let b: f64 = 0.1 + 0.1 + 0.25;
      println!("{} == {}", a, b);
      println!("{}", a==b);  // Fails, because they are not exactly equal
      println!("{}", f64::MIN);
      println!("{}", f64::EPSILON);
   }
}
