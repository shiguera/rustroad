use crate::*;
use std::f64::consts::PI;
//use factorial::Factorial;

/// Número de iteraciones en el desarrollo de las integrales de Fresnel
const NUMITER:i32 = 6_i32;

/// Generic clothoid segment starting in the origin (0, 0) 
/// with horizontal tangent and infinity radius at this point.
/// It uses zero as radius value for infinity radius
/// start_radius is always zero (infinity)
/// If ending radius is positive, clothoid is counterclockwise (1st or 3th quadrant) 
/// If ending radius is negative, the clothoid is clockwise (4th or 2nd quadrant)
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
      if eq001(parameter, 0.0_f64) || parameter < 0.0{
         panic!("Clothoid creation error: parameter can't be zero or negative");
      }      
      Clothoid{parameter, end_radius}
   }
   /// the length is always positive
   pub fn length(&self) -> f64 {
      self.parameter*self.parameter / self.end_radius.abs()
   }
   /// The azimuth_increment is positive in rightward curves
   /// and negative in leftward curves
   /// It's measured in degrees
   pub fn azimuth_increment(&self) -> f64 {
      self.length() / self.end_radius / 2.0 * 180.0/ PI
   }
   pub fn end_azimuth(&self) -> f64 {
      90.0 + self.azimuth_increment()    
   }  
   /// alpha is the angle measured in radians between the x axis and the tanget 
   /// to the clothoid in the generic point. 
   /// If end_radius is negative, angle is positive
   /// s = longitud de arco del punto
   pub fn alpha(&self, s:f64) -> f64 {
      if self.end_radius > 0_f64 {
         -s*s / 2.0 / self.parameter.powi(2)
      } else {
         s*s / 2.0 / self.parameter.powi(2)
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
      if self.end_radius > 0_f64 {
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
      let c = Clothoid::new(190.0, -450.0);
      assert_eq!(true, eq001(190.0, c.parameter));
      assert_eq!(true, eq001(-450.0, c.end_radius));      
   }
   #[test]
   #[should_panic]
   /// Parameter can't be zero
   fn test_new_panic_1() {
      let _c = Clothoid::new(0.0, 400.0);
   }
   #[test]
   #[should_panic]
   /// Parameter can't be negative
   fn test_new_panic_2() {
      let _c = Clothoid::new(-1000.0, 400.0);
   }
   #[test]
   #[should_panic]
   /// end_radius can't be zero
   fn test_new_panic_3() {
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
   fn test_azimuth_increment() {
      // En la hoja de cálculo de la M607 pone 
      // un inremento de azimuth de 5.675 grados
      // para estas clotoides
      let c = Clothoid::new(190.0, 450.0);
      println!("{}",c.azimuth_increment() );
      assert!(eq001(c.azimuth_increment(), 5.107));
      let c = Clothoid::new(190.0, -450.0);
      assert!(eq001(c.azimuth_increment(), -5.107));
   }

   #[test]
   fn test_end_azimuth() {
      // En la hoja de cálculo de la M607 pone 
      // un inremento de azimuth de 5.675 grados
      // para estas clotoides
      let c = Clothoid::new(190.0, 450.0);
      assert!(eq001(c.end_azimuth(),90.0+5.107));
      let c = Clothoid::new(190.0, -450.0);
      assert!(eq001(c.end_azimuth(), 90.0-5.107));
   }
   #[test]
   fn test_alpha() {
      // Positive radius
      let parameter = 500.0;
      let end_radius  = 400.0;
      let cl = Clothoid::new(parameter, end_radius);
      let len = cl.length();
      let azimuth_increment = cl.azimuth_increment();
      let alpha = cl.alpha(len);
      assert!(eq001(azimuth_increment, -alpha*180.0/PI));
      // Negative radius
      let end_radius  = -400.0;
      let cl = Clothoid::new(parameter, end_radius);
      let len = cl.length();
      let azimuth_increment = cl.azimuth_increment();
      let alpha = cl.alpha(len);
      assert!(eq001(azimuth_increment, -alpha*180.0/PI));

   }
   #[test]
   fn test_x() {
      // Case end_radius > 0 => x must be positive
      let length = 80.22_f64;
      let end_radius = 450_f64;
      let parameter = (length*end_radius.abs()).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let x = cl.x(cl.length());
      assert!(x > 0.0);
      assert!(eq001(x, 80.156));
      
      // Case end_radius < 0 => x must be positive
      let length = 80.22_f64;
      let end_radius = -450_f64;
      let parameter = (length*end_radius.abs()).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let x = cl.x(cl.length());
      assert!(x>0.0);
      assert!(eq001(x, 80.156));
   }
   #[test]
   fn test_y() {
      // Case end_radius>0.0 y must be negative
      let length = 80.22_f64;
      let end_radius = 450_f64;
      let parameter = (length*end_radius).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let y = cl.y(cl.length());
      assert!(y<0.0);
      assert!(eq001(y, -2.38207312368941));
      // Case end_radius<0.0 y must be positive
      let length = 80.22_f64;
      let end_radius = -450_f64;
      let parameter = (length*end_radius.abs()).sqrt();
      let cl = Clothoid::new(parameter, end_radius);
      let y = cl.y(cl.length());
      assert!(y>0.0);
      assert!(eq001(y, 2.38207312368941));    
   }
}