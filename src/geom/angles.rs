use std::convert::{From};

/// Angle is an angle sexagesimal, measured from the East
/// toward the North (counter-clockwise)
pub struct Angle {
   pub value: f64
}
impl Angle {
   pub fn new(angle: f64) -> Self {
      Angle{value:angle}
   }
   /// If the angle's absolute value is greater than 360º, 
   /// it is changed by its equivalent between 0 and 360. 
   /// If the angle is negative, it is changed by its 
   /// equivalent positive angle
   pub fn normalize(angle: f64) -> f64 {
      let mut newangle = angle;
      if angle.abs() > 360.0 {
         newangle = angle % 360.0;
      }
      if newangle < 0.0 {
         newangle = 360.0 + newangle;
      }
      newangle
   }
}
impl From<Azimuth> for Angle {
   fn from(azimuth: Azimuth) -> Self {
      Angle{value:Angle::normalize(270.0 + azimuth.value)}
   }
}
/// Azimuth is an angle in sexagesimal degrees,
/// between 0 and 360, measured from Sud toward the east
pub struct Azimuth {
   pub value: f64
}

impl Azimuth {
   /// Creates a new Azimuth instance. The angle passed as
   /// argument is normalized between 0 and 360
   fn new(angle: f64) -> Self {
      Azimuth{value:Angle::normalize(angle)}
   }
}
impl From<Angle> for Azimuth {
   fn from(angle: Angle) -> Self {
      Azimuth{value: Angle::normalize(angle.value + 90.0)}
   }
}

mod tests {
   #[cfg(test)]
   use super::*;
   use crate::eq001;

   #[test]
   fn test_angle_normalize() {
      assert!(eq001(Angle::normalize(0f64), 0.0f64));   
      assert!(eq001(Angle::normalize(45.0f64), 45.0f64));
      assert!(eq001(Angle::normalize(405.50f64), 45.50f64));
      assert!(eq001(Angle::normalize(765.0f64), 45.0f64));
      assert!(eq001(Angle::normalize(-45.0f64), 315.0f64));
      assert!(eq001(Angle::normalize(-450.0f64), 270.0f64));
      assert!(eq001(Angle::normalize(-765.0f64), 315.0f64));               
   }
   #[test]
   fn test_azimuth_new() {
      assert!(eq001(Azimuth::new(45.0f64).value, 45.0f64));
   }
   #[test]
   fn test_azimuth_from_angle() {
      assert!(eq001(Azimuth::from(Angle::new(0.0)).value, 90.0));
      assert!(eq001(Azimuth::from(Angle::new(180.0)).value, 270.0));
      assert!(eq001(Azimuth::from(Angle::new(-90.0)).value, 0.0));
      let x: Angle = Azimuth::new(45.0).into(); 
      assert!(eq001(x.value, Angle::new(315.0).value));
   }
   #[test]
   fn test_angle_from_azimuth() {
      assert!(eq001(Angle::from(Azimuth::new(0.0)).value, 270.0));
      assert!(eq001(Angle::from(Azimuth::new(180.0)).value, 90.0));
      assert!(eq001(Angle::from(Azimuth::new(-90.0)).value, 180.0));
      let az: Azimuth = Angle::new(120.0).into(); 
      assert!(eq001(az.value , Azimuth::new(210.0).value));     
   }
}