use super::point::Point;

// Circle with positive and not zero radius
pub struct Circle {
   pub center: Point,
   pub radius: f64
}

impl Circle {
   pub fn new(center: Point, radius: f64) -> Self {
      if radius <= 0.0 {
         panic!("Error, radius is negative or zero");
      }
      Circle{center, radius}
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test]
   fn test_new() {
      let p = Point::new(0.0, 0.0);
      let r = 10.0_f64;
      let c = Circle::new(p, r);
      assert_eq!(true, c.center.x-0.0< 0.001);
      assert_eq!(true, c.center.y-0.0< 0.001);
      assert_eq!(true, c.radius-10.0< 0.001);
      let p = Point::new(1.0, -1.0);
      let r = 1.0_f64;
      let c = Circle::new(p, r);
      assert_eq!(true, c.center.x-1.0< 0.001);
      assert_eq!(true, c.center.y-(-1.0)< 0.001);
      assert_eq!(true, c.radius-1.0< 0.001);
   }
   #[test]
   #[should_panic]
   fn test_new_panic() {
      let p = Point::new(0.0, 0.0);
      let r = 0.0_f64;
      let _c = Circle::new(p, r); 
   }
}