use crate::*;
use std::f64::consts::PI;
//use factorial::Factorial;

// Número de iteraciones en el desarrollo de las integrales de Fresnel
const NUMITER:i32 = 6_i32;

/// Generic clothoid starting in the origin (0, 0) 
/// with horizontal tangent and infinity radius at this point.
/// It uses zero as radius value for infinity radius
/// There are 4 possible solutions. If parameter A is positive, the solution is
/// in the first or fourth quadrant. If parameter A is negative, the solution
/// is in the second or thirth quadrant
/// If ending radius is positive, clothoid is counterclockwise (1st or 3th quadrant) 
/// If ending radius is negative, the clothoid is clockwise (4th or 2nd quadrant)
/// start_radius is always zero (infinity)
/// length is obtained from L = abs(A^2/end_radius)
#[derive(Debug, Clone, Copy)]
pub struct Clothoid {
   pub parameter: f64, 
   pub end_radius: f64 // radius>0 => counterclockwise
}

impl Clothoid {
   pub fn new(parameter: f64, end_radius: f64) -> Self {
      if eq001(end_radius, 0.0_f64) {
         panic!("Clothoid creation error: end_radius can't be zero");
      }
      if eq001(parameter, 0.0_f64) {
         panic!("Clothoid creation error: parameter can't be zero");
      }      
      Clothoid{parameter, end_radius}
   }
   /// the length is always positive
   pub fn length(&self) -> f64 {
      self.parameter*self.parameter / self.end_radius.abs()
   }
   /// The azimuth_increment is positive in rightward curves
   /// and negative in leftward curves
   pub fn azimuth_increment(&self) -> f64 {
      self.length() / self.end_radius / 2.0
   }
   pub fn end_azimuth(&self) -> f64 {
      let alpha_l = self.azimuth_increment();
      if self.end_radius > 0.0 {
            alpha_l
      } else {
            2.0*PI-alpha_l.abs()
      }    
   }  
   /// alpha is the angle with the x axis of the tanget 
   /// to the clothoid in the generic point 
   /// if end_radius is negative, angle is negative
   pub fn alpha(&self, s:f64) -> f64 {
      if self.end_radius > 0_f64 {
         s*s / 2.0 / self.parameter.powi(2)
      } else {
         -s*s / 2.0 / self.parameter.powi(2)
      }
   }
   /// The x coordinate at a given arc length s
   pub fn x(&self, s:f64) -> f64 {
      let alpha = self.alpha(s).abs();
      let mut x =0_f64;
      for n in 0..NUMITER+1 {
         x = x + (-1.0_f64).powi(n)*alpha.powi(2*n) / (4*n+1) as f64 / factorial(2*n as u64) as f64;
      }
      x = x*s;
      x
   }
   /// The y coordinate at a given arc length s
   pub fn y(&self, s: f64) -> f64 {
      let alpha = self.alpha(s).abs();
      let mut y = 0_f64;
      for n in 0..NUMITER {
         y = y + (-1.0_f64).powi(n) * alpha.powi(2*n+1) / (4*n+3) as f64 / factorial((2*n+1) as u64) as f64;
      }
      y=y*s;
      if self.end_radius < 0_f64 {
         y = -y;
      }
      y
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_new() {
      let c = Clothoid::new(190.0, 450.0);
      assert_eq!(true, eq001(190.0, c.parameter));
      assert_eq!(true, eq001(450.0, c.end_radius));      
      let c = Clothoid::new(-190.0, -450.0);
      assert_eq!(true, eq001(-190.0, c.parameter));
      assert_eq!(true, eq001(-450.0, c.end_radius));      
      let c = Clothoid::new(-190.0, 450.0);
      assert_eq!(true, eq001(-190.0, c.parameter));
      assert_eq!(true, eq001(450.0, c.end_radius));      
   }
   #[test]
   #[should_panic]
   /// Parameter can't be zero
   fn test_new_panic_1() {
      let _c = Clothoid::new(0.0, 400.0);
   }
   #[test]
   #[should_panic]
   /// end_radius can't be zero
   fn test_new_panic_2() {
      let _c = Clothoid::new(1000.0, 0.0);
   }

   #[test]
   fn test_length() {
      let c = Clothoid::new(190.0, 450.0);
      assert_eq!(true, (c.length()-80.222).abs()<0.001);
      let c = Clothoid::new(190.0, -450.0);
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
   }
   #[test]
   fn test_1() {
      let start_radius = 0.0;
      let end_radius = 100.0;
      println!("{} {} {}", !eq001(start_radius, 0.0), !eq001(end_radius, 0.0), !eq001(start_radius, 0.0) || !eq001(end_radius, 0.0));
      let x = 2.0_f64;
      let y = x.powi(3);
      println!("{}",y);
   }
   #[test]
   fn test_alpha() {
      let parameter = 500.0;
      let end_radius  = 400.0;
      let cl = Clothoid::new(parameter, end_radius);
      let len = cl.length();
      let endaz = cl.azimuth_increment();
      let alpha = cl.alpha(len);
      assert_eq!(true, eq001(endaz, alpha));
   }
   #[test]
   fn test_x() {
      // Case end_radius > 0 => x must be positive
      let length = 80.22_f64;
      let end_radius = 450_f64;
      let parameter = (length*end_radius.abs()).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let len = cl.length();
      let x = cl.x(len);
      assert!(x > 0.0);
      assert_eq!(true, (x.abs()-80.156).abs()<0.001);
      
      // Case end_radius < 0 => x must be positive
      let length = 80.22_f64;
      let end_radius = -450_f64;
      let parameter = (length*end_radius.abs()).sqrt();
      println!("{}", parameter);
      let cl = Clothoid::new(parameter, end_radius);
      let len = cl.length();
      println!("{}", len);
      let x = cl.x(len);
      println!("x={}", x);
      assert_eq!(true, x>0.0);
      assert_eq!(true, (x.abs()-80.156).abs()<0.001);

   }
   #[test]
   fn test_y() {
      // Case end_radius>0.0 y must be positive
      let length = 80.22_f64;
      let end_radius = 450_f64;
      let parameter = (length*end_radius).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let y = cl.y(cl.length());
      assert!(y>0.0);
      println!("{}", y);
      assert!(eq001(y, 2.38207312368941));
      // Case end_radius<0.0 y must be negative
      let length = 80.22_f64;
      let end_radius = -450_f64;
      let parameter = -(length*end_radius.abs()).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let y = cl.y(cl.length());
      assert!(y<0.0);
      assert!(eq001(y, -2.38207312368941));    
   }
}