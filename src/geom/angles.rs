use crate::eq001;

/// Angle is an angle sexagesimal, measured from the East
/// toward the North (counter-clockwise)
pub struct Angle {
   pub angle: f64
}
impl Angle {
   pub fn new(angle: f64) -> Self {
      Angle{angle}
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

/// Azimuth is an angle in sexagesimal degrees,
/// between 0 and 360, measured from Sud toward the east
pub struct Azimuth {
   pub angle: f64
}

impl Azimuth {
   /// Creates a new Azimuth instance. The angle passed as
   /// argument is normalized between 0 and 360
   fn new(angle: f64) -> Self {
      Azimuth{angle:Angle::normalize(angle)}
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

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
      assert!(eq001(Azimuth::new(45.0f64).angle, 45.0f64));
   }
}