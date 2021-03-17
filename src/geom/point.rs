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
}


mod tests {
   #[cfg(test)]
   use super::*;

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
}