use crate::geom::line::Line;

#[derive(Copy, Clone)]
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
   pub fn dist_to_line(&self, line: &Line) -> f64 {
      let mut d = (line.a*self.x + line.b*self.y + line.c) / (line.a*line.a + line.b*line.b).sqrt();
      d = d.abs();
      d
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
      assert_eq!(true, approx_eq!(f64, p.dist_to_line(&r), 10.0, ulps=2));
      let r = Line::new(1.0, 0.0, 3.0);
      let p = Point::new(0.0, 10.0);
      assert_eq!(true, approx_eq!(f64, p.dist_to_line(&r), 3.0, ulps=2));
      let r = Line::new(1.0, -1.0, 0.0);
      let p = Point::new(0.0, 0.0);
      assert_eq!(true, approx_eq!(f64, p.dist_to_line(&r), 0.0, ulps=2));
 
   }
}