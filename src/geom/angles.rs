//use std::f64::consts::PI;

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
}