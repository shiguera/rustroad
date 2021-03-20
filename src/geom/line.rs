use float_cmp::approx_eq;
use crate::geom::vector::Vector;
use crate::geom::point::Point;

#[derive(Debug)]
pub struct Line {
   // Straight ine in the form ax + by + c = 0
   pub a: f64,
   pub b: f64,
   pub c: f64
}
impl Line {
   pub fn new(a:f64, b:f64, c:f64) -> Self {
      Line{a,b,c}
   }
   pub fn from_point_vector(p:&Point, u:&Vector) -> Self {
      let a:f64;
      let b:f64;
      let c: f64;
      if approx_eq!(f64, u.x, 0.0, ulps=2) {
         a = 1.0;
         b = 0.0;
         c = -p.x;
      }  else {
         a = u.y/u.x;
         b = -1.0;
         c = p.y - a*p.x;
      }
      Line::new(a, b, c)
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test] 
   fn test_new() {
      let r1 = Line::new(1.0,1.0,1.0);
      assert_eq!(true, approx_eq!(f64, r1.a, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r1.b, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r1.c, 1.0, ulps=2));
      
   }
   #[test]
   fn test_from_point_vector() {
      let p = Point::new(0.0, 0.0);
      let u = Vector::new(1.0,1.0);
      let r = Line::from_point_vector(&p, &u);
      assert_eq!(true, approx_eq!(f64, r.a, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r.b, -1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r.c, 0.0, ulps=2));
      let p = Point::new(0.0, -10.0);
      let u = Vector::new(0.0,1.0);
      let r = Line::from_point_vector(&p, &u);
      println!("{:?}", r);
      assert_eq!(true, approx_eq!(f64, r.a, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r.b, 0.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r.c, 0.0, ulps=2));

   }
}