use crate::*;

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
      Clothoid{parameter, end_radius}
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test]
   fn test_new() {
      let c = Clothoid::new(500.0, 400.0);
      assert_eq!(true, eq(500.0, c.parameter));
      assert_eq!(true, eq(400.0, c.end_radius));      
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