
use core::panic;

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
      if eq(a, 0.0) && eq(b,0.0) {
         panic!("Coeficients are zero!");
      }
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
   pub fn from_two_points(p1: Point, p2: Point) -> Self {
      let a;
      let b;
      let c;
      if eq(p2.x-p1.x, 0.0) {
         a = 1.0;
         c = -p1.x;
         b = 0.0;
      } else {
         a = (p2.y-p1.y)/(p2.x-p1.x);
         c = p1.y-p1.x*(p2.y-p1.y)/(p2.x-p1.x);
         b = -1.0;
      }
      Line::new(a, b, c).canonical()
   }
   pub fn from_point_and_slope(p: Point, slope: f64) -> Self {
      todo!();
   }
   pub fn unitary_vector(&self) -> Vector {
      let mut uv: Vector; 
      if eq(self.b, 0.0) {
         uv = Vector::new(0.0, 1.0);
      } else {
         let slope = -self.a / self.b;
         let angle = slope.atan();
         uv = Vector::new(angle.cos(), angle.sin());
         if uv.vx < 0.0 {
            // Convierto siempre a primer o tercer cuadrante
            uv.vx = uv.vx * -1.0;
            uv.vy = uv.vy * -1.0;
         }
      }
      uv
   }
   pub fn y_intercept(&self) -> f64 {
      if eq(self.b, 0.0) {
         f64::INFINITY
      } else {
         -self.c / self.b
      }
   }
   pub fn slope(&self) -> f64{
      if eq(self.b, 0.0) {
         f64::INFINITY
      } else {
         -self.a / self.b
      }
   }
   pub fn canonical(&self) -> Self {
      if eq(self.a, 0.0) {
         Line::new(0.0, 1.0, self.c/self.b)
      } else {
         Line::new(self.a/self.a, self.b/self.a, self.c/self.a)
      }
   }
   pub fn contains_point(&self, p: Point) -> bool {
      eq(self.a*p.x + self.b*p.y + self.c, 0.0)
   }
   pub fn parallel_by_distance(&self, distance: f64) -> (Line, Line) {
      let a = self.a;
      let b = self.b;
      let c = self.c;
      let discriminante = 4.0*c*c - 4.0*(c*c-distance*distance*(a*a+b*b));
      let c2_1 = (2.0*c + discriminante.sqrt())/ 2.0;
      let c2_2 = (2.0*c - discriminante.sqrt())/ 2.0;
      let r1 = Line{a, b, c:c2_1}.canonical();
      let r2 = Line{a, b, c:c2_2}.canonical();
      (r1, r2)
   }
   pub fn intersection(&self, other: Self) -> Point {
      if eq(self.a, 0.0) && eq(other.a, 0.0) {
         panic!("lines are parallel!");
      }
      if eq(self.b, 0.0) && eq(other.b, 0.0) {
         panic!("lines are parallel!");
      }
      if eq(self.a/other.a, self.b/other.b) {
         panic!("lines are parallel!");
      }
      if eq(self.b, 0.0) {
         let x = -self.c/self.a;
         let y = -other.a*x/other.b -other.c/other.b;
         Point::new(x, y)
      } else {
         let denom = other.a - self.a*other.b/self.b;
         let x = (-other.c + self.c*other.b/self.b)/denom;
         let y = (-self.a*x -self.c)/self.b;
         Point { x, y }
      }
   }
   pub fn perpendicular_by_point(&self, p: Point) -> Self {
      let v = self.unitary_vector().left_normal_vector();
      Line::from_point_vector(p, v).canonical()
   }
   pub fn projection_of_point(&self, p: Point) -> Point {
      let r = self.perpendicular_by_point(p);
      self.intersection(r)
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_from_two_points() {
      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(10.0, 0.0);
      let r = Line::from_two_points(p1, p2);
      assert!(eq(r.a, 0.0));
      assert!(eq(r.b, 1.0));
      assert!(eq(r.c, 0.0));

      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(0.0, 1.0);
      let r = Line::from_two_points(p1, p2);
      assert!(eq(r.a, 1.0));
      assert!(eq(r.b, 0.0));
      assert!(eq(r.c, 0.0));

      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(10.0, 10.0);
      let r = Line::from_two_points(p1, p2);
      assert!(eq(r.a, 1.0));
      assert!(eq(r.b, -1.0));
      assert!(eq(r.c, 0.0));

      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(-10.0, 10.0);
      let r = Line::from_two_points(p1, p2);
      println!("{} {} {}", r.a, r.b, r.c);
      assert!(eq(r.a, 1.0));
      assert!(eq(r.b, 1.0));
      assert!(eq(r.c, 0.0));
   }
   #[test]
   fn test_from_point_and_slope() {
      todo!();
   }
   #[test]
   fn test_y_intercept() {
      todo!();
   }
   #[test]
   fn test_slope() {
      let r = Line::new(1.0, 1.0, 1.0);
      assert!(eq(r.slope(), -1.0));

      let r = Line::new(0.0, 1.0, 1.0);
      assert!(eq(r.slope(), 0.0));

      let r = Line::new(1.0, 0.0, 1.0);
      assert!(r.slope().is_infinite());
   }
   #[test]
   fn test_canonical() {
      let r = Line::new(2.0, 2.0, 2.0).canonical();
      assert!(eq(r.a, 1.0));
      assert!(eq(r.b, 1.0));
      assert!(eq(r.c, 1.0));

      let r = Line::new(0.0, 2.0, 2.0).canonical();
      assert!(eq(r.a, 0.0));
      assert!(eq(r.b, 1.0));
      assert!(eq(r.c, 1.0));

      let r = Line::new(2.0, 0.0, 2.0).canonical();
      assert!(eq(r.a, 1.0));
      assert!(eq(r.b, 0.0));
      assert!(eq(r.c, 1.0));

      let r = Line::new(2.0, 0.0, 0.0).canonical();
      assert!(eq(r.a, 1.0));
      assert!(eq(r.b, 0.0));
      assert!(eq(r.c, 0.0));

      let r = Line::new(0.0, 2.0, 0.0).canonical();
      assert!(eq(r.a, 0.0));
      assert!(eq(r.b, 1.0));
      assert!(eq(r.c, 0.0));

   }
   #[test]
   fn test_contains_point() {
      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(1.0, 1.0);
      let r = Line::from_two_points(p1, p2);
      assert!(r.contains_point(p1));
      assert!(r.contains_point(p2));

      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(1.0, 0.0);
      let r = Line::from_two_points(p1, p2);
      assert!(r.contains_point(p1));
      assert!(r.contains_point(p2));

      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(0.0, 1.0);
      let r = Line::from_two_points(p1, p2);
      assert!(r.contains_point(p1));
      assert!(r.contains_point(p2));

   }
   #[test] 
   fn test_intersection() {
      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(10.0, 0.0);
      let p3 = Point::new(0.0, 10.0);
      let r1 = Line::from_two_points(p1, p2);
      let r2 = Line::from_two_points(p1, p3);
      let p4 = r1.intersection(r2);
      println!("{} {}", p4.x, p4.y);
      assert!(eq(p4.x, p1.x));
      assert!(eq(p4.y, p1.y));

      let p1 = Point::new(0.0, 0.0);
      let p2 = Point::new(0.0, 10.0);
      let p3 = Point::new(10.0, 0.0);
      let r1 = Line::from_two_points(p1, p2);
      let r2 = Line::from_two_points(p1, p3);
      let p4 = r1.intersection(r2);
      println!("{} {}", p4.x, p4.y);
      assert!(eq(p4.x, p1.x));
      assert!(eq(p4.y, p1.y));

      let p1 = Point::new(1.0, 2.0);
      let p2 = Point::new(10.0, 10.0);
      let p3 = Point::new(10.0, -20.0);
      let r1 = Line::from_two_points(p1, p2);
      let r2 = Line::from_two_points(p1, p3);
      let p4 = r1.intersection(r2);
      println!("{} {}", p4.x, p4.y);
      assert!(eq(p4.x, p1.x));
      assert!(eq(p4.y, p1.y));
   }
   #[test]
   #[should_panic]
   fn test_intersection_panic_1() {
      let r1 = Line::new(2.0, 2.0, 2.0);
      let r2 = Line::new(4.0, 4.0, 1.0);
      let _p = r1.intersection(r2);
   }
   #[test]
   #[should_panic]
   fn test_intersection_panic_2() {
      let r1 = Line::new(0.0, 1.0, 2.0);
      let r2 = Line::new(0.0, 1.0, 1.0);
      let _p = r1.intersection(r2);
   }
   #[test]
   #[should_panic]
   fn test_intersection_panic_3() {
      let r1 = Line::new(1.0, 0.0, 2.0);
      let r2 = Line::new(1.0, 0.0, 1.0);
      let _p = r1.intersection(r2);
   }
   #[test] 
   fn test_projection_of_point() {
      let r = Line::new(1.0, 0.0, 0.0);
      let p = Point::new(3.0, 2.0);
      let q = r.projection_of_point(p);
      assert!(eq(q.x, 0.0));
      assert!(eq(q.y, 2.0));

      let r = Line::new(0.0, 1.0, 0.0);
      let p = Point::new(3.0, 2.0);
      let q = r.projection_of_point(p);
      assert!(eq(q.x, 3.0));
      assert!(eq(q.y, 0.0));

      let r = Line::new(-1.0, 1.0, -1.0);
      let p = Point::new(1.0, 0.0);
      let q = r.projection_of_point(p);
      println!("{} {}", q.x, q.y);
      assert!(eq(q.x, 0.0));
      assert!(eq(q.y, 1.0));

   }
   #[test] 
   fn test_perpendicular_by_point() {
      let r = Line::new(1.0, 0.0, 0.0);
      let r2 = r.perpendicular_by_point(Point::new(0.0, 0.0));
      assert!(eq(r2.a, 0.0));
      assert!(eq(r2.b, 1.0));
      assert!(eq(r2.c, 0.0));
      
      let r = Line::new(0.0, 1.0, 0.0);
      let r2 = r.perpendicular_by_point(Point::new(0.0, 0.0));
      assert!(eq(r2.a, 1.0));
      assert!(eq(r2.b, 0.0));
      assert!(eq(r2.c, 0.0));

      let r = Line::new(1.0, -1.0, 0.0);
      let r2 = r.perpendicular_by_point(Point::new(0.0, 0.0));
      assert!(eq(r2.a, 1.0));
      assert!(eq(r2.b, 1.0));
      assert!(eq(r2.c, 0.0));
      
   }
   #[test] 
   fn test_unitary_vector() {
      let r = Line::new(2.0, 0.0, 3.0);
      let uv = r.unitary_vector();
      println!("{} {}", uv.vx, uv.vy);
      assert!(eq(uv.vx, 0.0));
      assert!(eq(uv.vy, 1.0));

      let r = Line::new(0.0, 4.0, 3.0);
      let uv = r.unitary_vector();
      println!("{} {}", uv.vx, uv.vy);
      assert!(eq(uv.vx, 1.0));
      assert!(eq(uv.vy, 0.0));

      let r = Line::new(1.0, -1.0, 20.0);
      let uv = r.unitary_vector();
      println!("{} {}", uv.vx, uv.vy);
      assert!(eq(uv.vx, (PI/4.0).cos()));
      assert!(eq(uv.vy, (PI/4.0).sin()));
   }   
   #[test] 
   fn test_new() {
      let r1 = Line::new(1.0,1.0,1.0);
      assert_eq!(true, eq001(r1.a, 1.0));
      assert_eq!(true, eq001(r1.b, 1.0));
      assert_eq!(true, eq001(r1.c, 1.0));
   }
   #[test]
   #[should_panic]
   fn test_new_panic_1() {
      let _r1 = Line::new(0.0,0.0,1.0);
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

      let r = Line{a:1.0, b: 0.0, c:0.0};
      let d = 3.0f64;
      let (r1, r2) = r.parallel_by_distance(d);
      assert!(eq001(r1.a, 1.0) && eq001(r2.a, 1.0));
      assert!(eq001(r1.b, 0.0) && eq001(r2.b, 0.0));
      assert!(eq001(r1.c, 3.0) || eq001(r2.c, 3.0) );
      assert!(eq001(r1.c, -3.0) || eq001(r2.c, -3.0) );
     
      let r = Line{a:1.0, b: 1.0, c:1.0};
      let d = (PI/4.0).cos();
      let (r1, r2) = r.parallel_by_distance(d);
      println!("{} {} {}", r1.a, r1.b, r1.c);
      println!("{} {} {}", r2.a, r2.b, r2.c);
      assert!(eq001(r1.a, 1.0) && eq001(r2.a, 1.0));
      assert!(eq001(r1.b, 1.0) && eq001(r2.b, 1.0));
      assert!(eq001(r1.c, 0.0) || eq001(r2.c, 0.0) );
      assert!(eq001(r1.c, 2.0) || eq001(r2.c, 2.0) );
   }

}