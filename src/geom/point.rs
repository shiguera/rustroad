use crate::geom::line::Line;
use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct Point {
   pub x: f64,
   pub y: f64
}
impl Point {
   pub fn new(x:f64, y:f64) -> Self {
      Point{x,y}
   }
   pub fn dist_to(&self, other: Point) -> f64 {
      ((other.x-self.x).powf(2.0) + (other.y-self.y).powf(2.0)).sqrt()
   }
   pub fn dist_to_line(&self, line: Line) -> f64 {
      let mut d = (line.a*self.x + line.b*self.y + line.c) / (line.a*line.a + line.b*line.b).sqrt();
      d = d.abs();
      d
   }
   pub fn middle_point(self, other: Point) -> Self {
      Point::new((self.x + other.x)/2.0, (self.y+other.y)/2.0)
   }
}


mod tests {
   #[cfg(test)]
   use super::*;
   #[cfg(test)]
   use float_cmp::approx_eq;

   #[test]
   fn test_new() {
      let p = Point::new(3.0, -1.5);
      assert_eq!(true, p.x-3.0<0.001);
      assert_eq!(true, p.y-(-1.5)<0.001);
   }
   #[test]
   fn test_dist_to() {
      let p1 = Point::new(0.0,0.0);
      let p2 = Point::new(10.0,0.0);
      assert_eq!(true, p1.dist_to(p2)-p2.dist_to(p1)<0.001);
      //assert_eq!(true, &p1.dist_to(p2)-10.0<0.001);
      //let p3 = Point::new(-1.0,-1.0);
      //assert_eq!(true, &p1.dist_to(p3)-2.0_f64.sqrt()<0.001);           
   }
   #[test]
   fn test_dist_to_line() {
      let r = Line::new(0.0, 1.0, 0.0);
      let p = Point::new(0.0, 10.0);
      assert_eq!(true, approx_eq!(f64, p.dist_to_line(r), 10.0, ulps=2));
      let r = Line::new(1.0, 0.0, 3.0);
      let p = Point::new(0.0, 10.0);
      assert_eq!(true, approx_eq!(f64, p.dist_to_line(r), 3.0, ulps=2));
      let r = Line::new(1.0, -1.0, 0.0);
      let p = Point::new(0.0, 0.0);
      assert_eq!(true, approx_eq!(f64, p.dist_to_line(r), 0.0, ulps=2));
 
   }
   #[test]
   fn test_middle_point() {
      let p1 = Point::new(0.0, 0.0);
      let other = Point::new(10.0, 0.0);
      let mp = p1.middle_point(other);
      assert_eq!(true, eq(mp.x, 5.0));
      let other = Point::new(10.0, 10.0);
      let mp = p1.middle_point(other);
      assert_eq!(true, eq(mp.x, 5.0));
      assert_eq!(true, eq(mp.y, 5.0));
      let p1 = Point::new(-1.0, -1.0);
      let other = Point::new(-2.0, -2.0);
      let mp = p1.middle_point(other);
      assert_eq!(true, eq(mp.x, -1.5));
      assert_eq!(true, eq(mp.y, -1.5));
      
      

   }
}