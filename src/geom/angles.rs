use std::f64::consts::PI;
use crate::{normalize_360, rad2deg};

/// Azimuth is an angle in sexagesimal degrees,
/// between 0 and 360, measured from the North toward the east
/// It desn't allow negative values or values greater than 360º
#[derive(Debug, Copy, Clone)]
pub struct Azimuth {
   pub value: f64
}

impl Azimuth {
   /// Creates a new Azimuth instance. The angle passed as
   /// argument is normalized between 0 and 360
   pub fn new(azimuth: f64) -> Self {
      Azimuth{value: crate::normalize_360(azimuth)}
   }
   /// Calculates the azimuth equivalent to an angle
   pub fn from_angle(angle:f64) -> Self {
      Azimuth::new(normalize_360(rad2deg(PI/2.0 - angle)))
   }
}


#[cfg(test)]
mod tests {
   use super::*;
   use crate::eq001;

   #[test]
   fn test_azimuth_new() {
      eq001(Azimuth::new(45.0f64).value, 45.0f64);
      eq001(Azimuth::new(445.0f64).value, 85.0f64);
      eq001(Azimuth::new(-45.0f64).value, 315.0f64);
   }
   #[test]
   fn test_from_angle() {
      let ang = 0.0;
      assert!(eq001(Azimuth::from_angle(ang).value, 90.0));
      let ang = PI/6.0;
      assert!(eq001(Azimuth::from_angle(ang).value, 60.0));
      let ang = PI/3.0;
      assert!(eq001(Azimuth::from_angle(ang).value, 30.0));
      let ang = PI/2.0;
      assert!(eq001(Azimuth::from_angle(ang).value, 0.0));
      let ang = -PI/2.0;
      assert!(eq001(Azimuth::from_angle(ang).value, 180.0));
      
      
      
   }
}