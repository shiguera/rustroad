
use crate::*;
use crate::geom::vector::Vector;
use crate::geom::point::Point;

/// Straight line in the form ax + by + c = 0
#[derive(Debug, Clone, Copy)]
pub struct Line {
   pub a: f64,
   pub b: f64,
   pub c: f64
}
impl Line {
   pub fn new(a:f64, b:f64, c:f64) -> Self {
      Line{a,b,c}
   }
   pub fn from_point_vector(p:Point, u:Vector) -> Self {
      // Line from a Point and a Vector
      let a:f64;
      let b:f64;
      let c: f64;
      if eq001(u.vx, 0.0) {
         a = 1.0;
         b = 0.0;
         c = -p.x;
      }  else {
         a = u.vy/u.vx;
         b = -1.0;
         c = p.y - a*p.x;
      }
      Line::new(a, b, c)
   }
   pub fn parallel_by_distance(&self, distance: f64) -> (Line, Line) {
      let a = self.a;
      let b = self.b;
      let c = self.c;
      let discriminante = 4.0*c*c - 4.0*(c*c-distance*distance*(a*a+b*b));
      let c2_1 = (2.0*c + discriminante)/ 2.0;
      let c2_2 = (2.0*c - discriminante)/ 2.0;
      let r1 = Line{a, b, c:c2_1};
      let r2 = Line{a, b, c:c2_2};
      (r1, r2)
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test] 
   fn test_new() {
      let r1 = Line::new(1.0,1.0,1.0);
      assert_eq!(true, eq001(r1.a, 1.0));
      assert_eq!(true, eq001(r1.b, 1.0));
      assert_eq!(true, eq001(r1.c, 1.0));
      
   }
   #[test]
   fn test_from_point_vector() {
      let p = Point::new(0.0, 0.0);
      let u = Vector::new(1.0,1.0);
      let r = Line::from_point_vector(p, u);
      assert_eq!(true, eq001(r.a, 1.0));
      assert_eq!(true, eq001(r.b, -1.0));
      assert_eq!(true, eq001(r.c, 0.0));
      let p = Point::new(0.0, -10.0);
      let u = Vector::new(0.0,1.0);
      let r = Line::from_point_vector(p, u);
      assert_eq!(true, eq001(r.a, 1.0));
      assert_eq!(true, eq001(r.b, 0.0));
      assert_eq!(true, eq001(r.c, 0.0));
      println!("{:?}", p);
      println!("{:?}", u);
      println!("{:?}", r);

   }
   #[test]
   fn test_parallel_by_distance() {
      let r = Line{a:0.0, b: 1.0, c:0.0};
      let d = 3.0f64;
      let (r1, r2) = r.parallel_by_distance(d);
      assert!(eq001(r1.a, 0.0) && eq001(r2.a, 0.0));
      assert!(eq001(r1.b, 1.0) && eq001(r2.b, 1.0));
      assert!(eq001(r1.c, 3.0) || eq001(r2.c, 3.0) );
      assert!(eq001(r1.c, -3.0) || eq001(r2.c, -3.0) );
      
   }

}