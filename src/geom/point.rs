use crate::geom::line::Line;
//use crate::*;

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
      let d = (line.a*self.x + line.b*self.y + line.c) / (line.a*line.a + line.b*line.b).sqrt();
      d.abs()
   }
   ///
   /// Punto medio entre el punto instancia y otro punto
   /// 
   pub fn middle_point(&self, other: Point) -> Self {
      Point::new((self.x + other.x)/2.0, (self.y+other.y)/2.0)
   }
   ///
   /// New Point whose coordinates are these of the 
   /// instance point but in new axis rotated an angle 
   /// theta ccw 
   ///  
   pub fn rotate_axis(&self, theta: f64) -> Self {
      let xprim = self.x*theta.cos() + self.y*theta.sin();
      let yprim = -self.x*theta.sin() + self.y*theta.cos();
      Point::new(xprim, yprim)
   }
   ///
   /// New Point whose coordinates are traslated (incx, incy)
   /// # 
   /// (Nota: Traslada las coordenadas, no da las coordenadas
   /// resultantes de una traslación de los ejes)
   /// 
   pub fn traslate_point(&self, incx: f64, incy: f64) -> Self {
      //   
      Point::new(self.x+incx, self.y+incy)
   }
}


#[cfg(test)]
mod tests {
   use super::*;
   #[cfg(test)]
   use crate::*;
   #[cfg(test)]
   use std::f64::consts::*;

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
      assert_eq!(true, eq001(p.dist_to_line(r), 10.0));
      let r = Line::new(1.0, 0.0, 3.0);
      let p = Point::new(0.0, 10.0);
      assert_eq!(true, eq001(p.dist_to_line(r), 3.0));
      let r = Line::new(1.0, -1.0, 0.0);
      let p = Point::new(0.0, 0.0);
      assert_eq!(true, eq001(p.dist_to_line(r), 0.0));
 
   }
   #[test]
   fn test_middle_point() {
      let p1 = Point::new(0.0, 0.0);
      let other = Point::new(10.0, 0.0);
      let mp = p1.middle_point(other);
      assert_eq!(true, eq001(mp.x, 5.0));
      let other = Point::new(10.0, 10.0);
      let mp = p1.middle_point(other);
      assert_eq!(true, eq001(mp.x, 5.0));
      assert_eq!(true, eq001(mp.y, 5.0));
      let p1 = Point::new(-1.0, -1.0);
      let other = Point::new(-2.0, -2.0);
      let mp = p1.middle_point(other);
      assert_eq!(true, eq001(mp.x, -1.5));
      assert_eq!(true, eq001(mp.y, -1.5));
   }
   #[test]
   fn test_rotate_axis() {
      let p = Point::new(1.0, 0.0);
      let pprim = p.rotate_axis(PI);
      assert_eq!(true, (pprim.x-(-1.0_f64)).abs()<1e-10);
      assert_eq!(true, (pprim.y-0.0_f64).abs()<1e-10);
      let p = Point::new(1.0, 0.0);
      let pprim = p.rotate_axis(PI/2.0);
      assert_eq!(true, (pprim.x-0.0_f64).abs()<1e-10);
      assert_eq!(true, (pprim.y-(-1.0_f64)).abs()<1e-10);
   }
   #[test]
   fn test_traslate_axis() {
      let p = Point::new(1.0, 0.0);
      let pprim = p.traslate_point(10.0, -10.0);
      assert_eq!(true, eq001(pprim.x, 11.0_f64));
      assert_eq!(true, eq001(pprim.y,-10.0_f64));      
   }
   #[test]
   fn test_1() {
      println!("{}", FRAC_PI_2.cos());
      println!("{}", eq001((PI/2.0).cos(), 0.0));
      println!("{}", (PI/2.0).sin());
      let x = 90.0_f64.to_radians();
      println!("{}", x.cos());
      
   }

}