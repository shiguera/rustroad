use std::convert::{From};
use std::f64::consts::PI;

/// Angle is an angle in radians, measured from the East
/// toward the North (counter-clockwise). 
/// It can be negative or positive, and allows values 
/// greater than 2*PI. It has a class method, normalize(), 
/// that convert to an angle value to its equivalent 
/// less than 2*PI and positive
#[derive(Debug, Copy, Clone)]
pub struct Angle {
   pub value: f64
}
impl Angle {
   pub fn new(angle: f64) -> Self {
      Angle{value:angle}
   }
}
impl From<Azimuth> for Angle {
   fn from(azimuth: Azimuth) -> Self {
      let az_radian = azimuth.value*PI/180.0;
      Angle{value:crate::normalize_radian(az_radian + PI/2.0)}
   }
}
/// Azimuth is an angle in sexagesimal degrees,
/// between 0 and 360, measured from the North toward the east
/// It desn't allow negative values
#[derive(Debug, Copy, Clone)]
pub struct Azimuth {
   pub value: f64
}

impl Azimuth {
   /// Creates a new Azimuth instance. The angle passed as
   /// argument is normalized between 0 and 360
   pub fn new(azimuth: f64) -> Self {
      Azimuth{value: crate::normalize_radian(azimuth)}
   }
}
/// Azimuth from an Angle
impl From<Angle> for Azimuth {
   fn from(angle: Angle) -> Self {
      Azimuth{value: crate::normalize_radian(270.0 + angle.value)}
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use crate::assert_eq001;

   #[test]
   fn test_azimuth_new() {
      assert!(assert_eq001(Azimuth::new(45.0f64).value, 45.0f64));
      assert!(assert_eq001(Azimuth::new(445.0f64).value, 85.0f64));
      assert!(assert_eq001(Azimuth::new(-45.0f64).value, 315.0f64));
      
   }
   #[test]
   fn test_azimuth_from_angle() {
      assert!(assert_eq001(Azimuth::from(Angle::new(0.0)).value, 270.0));
      assert!(assert_eq001(Azimuth::from(Angle::new(180.0)).value, 90.0));
      assert!(assert_eq001(Azimuth::from(Angle::new(-90.0)).value, 180.0));
      let x: Angle = Azimuth::new(45.0).into(); 
      assert!(assert_eq001(x.value, Angle::new(135.0).value));
   }
   #[test]
   fn test_angle_from_azimuth() {
      assert!(assert_eq001(Angle::from(Azimuth::new(0.0)).value, 90.0*PI/180.0));
      println!("{} {}",Angle::from(Azimuth::new(180.0)).value, 270.0*PI/180.0);
      assert!(assert_eq001(Angle::from(Azimuth::new(180.0)).value, 270.0*PI/180.0));      
      assert!(assert_eq001(Angle::from(Azimuth::new(-90.0)).value, 0.0));
      let az: Azimuth = Angle::new(120.0*PI/180.0).into(); 
      assert!(assert_eq001(az.value , Azimuth::new(30.0).value));     
   }
}