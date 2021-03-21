use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Clothoid {
   // Clothoid.
   // Assumes one of the radius is zero (infinity)
   // It uses zero for infinity radius
   pub length: f64,
   pub start_radius: f64, 
   pub end_radius: f64 // radius>0 => counterclockwise
}

impl Clothoid {
   pub fn new(length: f64, start_radius: f64, end_radius: f64) -> Self {
      if eq(start_radius, 0.0) && eq(end_radius, 0.0) {
         // The two radius can't be zero
         panic!("Clothoid with zero radius in the two extremes")
      }
      if !eq(start_radius, 0.0) && !eq(end_radius, 0.0) {
         // One of the radius must be zero
         panic!("Clothoid without zero radius in any extreme")
      }    
      Clothoid{length, start_radius, end_radius}
   }
   pub fn parameter(self) -> f64 {
      // Parameter is A with L*R=A^2
      if eq(self.start_radius, 0.0) {
         (self.length*self.end_radius.abs()).sqrt()
      } else {
         (self.length*self.start_radius.abs()).sqrt()
      }
   }
   
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test]
   fn test_new() {
      let c = Clothoid::new(100.0, 0.0, 400.0);
      assert_eq!(true, eq(100.0, c.length));
      assert_eq!(true, eq(0.0, c.start_radius));
      assert_eq!(true, eq(400.0, c.end_radius));      
   }
   #[test]
   #[should_panic]
   fn test_new_panic() {
      // One of the radius must be zero
      let _c = Clothoid::new(100.0, 200.0, 400.0);
   }
   #[test]
   #[should_panic]
   fn test_new_panic_2() {
      // The two radius can't be zero
      let _c = Clothoid::new(100.0, 0.0, 0.0);
   }
   #[test]
   fn test_parameter() {
      let c = Clothoid::new(100.0, 0.0, 400.0);
      assert_eq!(true, eq(40000.0_f64.sqrt(), c.parameter()));
      let c = Clothoid::new(100.0, 400.0, 0.0);
      assert_eq!(true, eq(40000.0_f64.sqrt(), c.parameter()));
      let c = Clothoid::new(100.0, -400.0, 0.0);
      assert_eq!(true, eq(40000.0_f64.sqrt(), c.parameter()));
      let c = Clothoid::new(100.0, 0.0, -400.0);
      assert_eq!(true, eq(40000.0_f64.sqrt(), c.parameter()));
      
   }
   #[test]
   fn test_1() {
      let start_radius = 0.0;
      let end_radius = 100.0;
      println!("{} {} {}", !eq(start_radius, 0.0), !eq(end_radius, 0.0), !eq(start_radius, 0.0) || !eq(end_radius, 0.0));

   }
}