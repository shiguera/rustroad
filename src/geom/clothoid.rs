use crate::*;
use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Clothoid {
   // Generic clothoid starting in the origin (0, 0) 
   // with tangent horizontal and infinity radius in this point.
   // It uses zero as radius value for infinity radius
   // There are 4 possible solutions. If parameter A is positive, the solution is
   // in the first or fourth quadrant. If parameter A is negative, the solution
   // is in the second or thirth quadrant
   // If ending radius is positive, clothoid is counterclockwise (1st or 3th quadrant) 
   // If ending radius is negative, the clothoid is clockwise (4th or 2nd quadrant)
   // start_radius is always zero (infinity)
   // length is obtained from L = A^2/end_radius
   pub parameter: f64, 
   pub end_radius: f64 // radius>0 => counterclockwise
}

impl Clothoid {
   pub fn new(parameter: f64, end_radius: f64) -> Self {
      if eq(end_radius, 0.0_f64) {
         panic!("Clothoid creation error: end_radius can't be zero");
      }
      if eq(parameter, 0.0_f64) {
         panic!("Clothoid creation error: parameter can't be zero");
      }
      
      Clothoid{parameter, end_radius}
   }
   pub fn length(self) -> f64 {
      self.parameter*self.parameter / self.end_radius.abs()
   }
   pub fn end_azimuth(self) -> f64 {
      let alpha_l = self.azimuth_increment();
      if self.parameter > 0.0 {
         if self.end_radius > 0.0 {
               alpha_l
         } else {
               2.0*PI-alpha_l.abs()
         }
      }  else {
            PI+alpha_l
      }    
   }  
   pub fn azimuth_increment(self) -> f64 {
      self.length() / self.end_radius / 2.0
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test]
   fn test_new() {
      let c = Clothoid::new(190.0, 450.0);
      assert_eq!(true, eq(190.0, c.parameter));
      assert_eq!(true, eq(450.0, c.end_radius));      
   }
   #[test]
   #[should_panic]
   fn test_new_panic_1() {
      let _c = Clothoid::new(0.0, 400.0);
   }
   #[test]
   #[should_panic]
   fn test_new_panic_2() {
      let _c = Clothoid::new(1000.0, 0.0);
   }

   #[test]
   fn test_length() {
      let c = Clothoid::new(190.0, 450.0);
      assert_eq!(true, (c.length()-80.222).abs()<0.001);
      let c = Clothoid::new(190.0, -450.0);
      assert_eq!(true, (c.length()-80.222).abs()<0.001);
      let c = Clothoid::new(-190.0, 450.0);
      assert_eq!(true, (c.length()-80.222).abs()<0.001);
      let c = Clothoid::new(-190.0, -450.0);
      assert_eq!(true, (c.length()-80.222).abs()<0.001);
   }
   #[test]
   fn test_alpha_l() {
      let c = Clothoid::new(190.0, 450.0);
      assert_eq!(true, (c.azimuth_increment()-0.08914).abs()<0.001);
      let c = Clothoid::new(190.0, -450.0);
      assert_eq!(true, (c.azimuth_increment()+0.08914).abs()<0.001);
   }

   #[test]
   fn test_end_azimuth() {
      let c = Clothoid::new(190.0, 450.0);
      assert_eq!(true, (c.end_azimuth()-0.08914).abs()<0.001);
      let c = Clothoid::new(190.0, -450.0);
      assert_eq!(true, (c.end_azimuth()-6.1940).abs()<0.001);
      let c = Clothoid::new(-190.0, 450.0);
      assert_eq!(true, (c.end_azimuth()-3.2307).abs()<0.001);
      let c = Clothoid::new(-190.0, -450.0);
      assert_eq!(true, (c.end_azimuth()-3.0525).abs()<0.001);
   }
   #[test]
   fn test_1() {
      let start_radius = 0.0;
      let end_radius = 100.0;
      println!("{} {} {}", !eq(start_radius, 0.0), !eq(end_radius, 0.0), !eq(start_radius, 0.0) || !eq(end_radius, 0.0));
      let x = 2.0_f64;
      let y = x.powi(3);
      println!("{}",y);
   }
}