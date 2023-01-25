use crate::geom::clothoid::Clothoid;
use crate::geom::point::Point;
//use crate::geom::vector::Vector;
use crate::*;
use super::hsection::HSection;
use std::f64::consts::PI;

pub struct HClothoid {
   pub start_point: Point,
   pub start_azimuth: f64,
   pub start_radius: f64,
   pub end_radius: f64,
   pub length: f64
}

impl HClothoid {
   /// Creates a new HClothoid
   /// One of the radius must be zero, but not both
   pub fn new(start_point: Point, start_azimuth: f64, start_radius: f64, 
      end_radius: f64, length: f64) -> Self {
         if eq001(0.0, start_radius) {
            if eq001(0.0, end_radius) {
               panic!("Clothoid can't have start and end radius equal zero");
            } 
         } else {
            if !eq001(0.0, end_radius) {
               panic!("Clothoid must have one of the radius equals zero");
            } 
         }
         if eq001(length, 0.0) {
            panic!("Clothoid can't have length equals zero");
         }
         HClothoid{start_point, start_azimuth, start_radius, end_radius, length}
      }
}
impl HClothoid {
   pub fn clothoid(&self) -> Clothoid {
      Clothoid { parameter: self.parameter(), end_radius: self.radius_in_tangent_to_circle_point() }
   }
   pub fn radius_in_tangent_to_circle_point(&self) -> f64 {
      if eq001(self.start_radius, 0.0) {
         self.end_radius
      } else {
         self.start_radius
      }
   }
   pub fn center_x(&self) -> f64 {
      let incx = (self.radius_in_tangent_to_circle_point()*self.alpha_l().sin()).abs();
      let xc_local = self.clothoid().x(self.length) - incx;
      let theta = azimuth_to_angle(self.start_azimuth);
      let (xc,_y) = rotation(xc_local, 0.0, -theta);
      let xc_global = self.start_point.x + xc;
      xc_global
   }
   pub fn center_y(&self) -> f64 {
      let mut yc_local = self.radius_in_tangent_to_circle_point().abs()+self.retranqueo();
      if self.radius_in_tangent_to_circle_point()>0.0 {
         yc_local = -yc_local;
      }
      let theta = azimuth_to_angle(self.start_azimuth);
      let (_x, yc) = rotation(0.0, yc_local, -theta);
      let yc_global = self.start_point.y + yc;
      yc_global
   }
   pub fn center(&self) -> Point {
      Point{x: self.center_x(), y: self.center_y()}
   }
   /// El retranqueo siempre es una cantidad positiva
   pub fn retranqueo(&self) -> f64 {
      let y = (self.clothoid().y(self.length)).abs();
      y - self.radius_in_tangent_to_circle_point().abs()*(1.0-self.alpha_l().cos())
   }
   /// Ángulo alpha_L en radianes
   pub fn alpha_l(&self) -> f64 {
      -self.length / 2.0 / self.radius_in_tangent_to_circle_point()
   }
   pub fn parameter(&self) -> f64 {
      let r = self.radius_in_tangent_to_circle_point();
      let a = (r.abs()* self.length).sqrt();
      a
   }

   pub fn origin_point(&self) -> Point {
      // De una alineación clotoide en la que el punto inicial no es el origen de la clotoide
      todo!();
      Point{x: 0.0, y: 0.0} 
   }
}
impl HSection for HClothoid {
   fn start_point(&self) -> Point {
      self.start_point
   }
   fn end_point(&self) -> Point {
      if eq001(self.start_radius, 0.0) {
         // Cálculo directo
         let xprim = self.clothoid().x(self.length);
         let yprim = self.clothoid().y(self.length);
         let theta = azimuth_to_angle(self.start_azimuth);
         //println!("xprim={} yprim={} theta={}", xprim, yprim, rad2deg(theta));
         let (x, y) = rotation(xprim, yprim, -theta);
         //println!("x={} y={}", x, y);         
         let end_x = self.start_point.x + x;
         let end_y = self.start_point.y + y;
         Point{x: end_x, y: end_y}

      } else {
         // Cálculo enrevesado partiendo del punto de tangencia con el círculo
         Point{x:0.0, y: 0.0}
      }
   }
   fn start_radius(&self) -> f64 {
      self.start_radius
   }
   fn end_radius(&self) -> f64 {
      self.end_radius
   }
   fn length(&self) -> f64 {
      self.length
   }
   fn start_azimuth(&self) -> f64 {
      self.start_azimuth
   }
   fn end_azimuth(&self) -> f64 {
      let inc_az: f64;
      if self.start_radius.abs() > 0.0 {
         inc_az = self.length() / 2.0 / self.start_radius();
      } else {
         inc_az = self.length() / 2.0 / self.end_radius();
      }
      let mut end_az = self.start_azimuth() + inc_az;
      if end_az < 0.0 {
         end_az = 2.0 * PI  + end_az;
      } 
      if end_az > 2.0 * PI {
         end_az = end_az - 2.0 * PI;
      }
      end_az
   }
   fn azimuth_at_s(&self, _s:f64) -> f64 {
      todo!();
   }
   fn point_at_s(&self, _s:f64) -> Point {
      todo!();
   }
}
#[cfg(test)]
mod tests {
   use super::*;

   fn sample_clothoid_direct_positive_radius() -> HClothoid {
      // start_radius = 0, end_radius= 450
      // parameter = 190,
      // end_point.x = 432730.377
      // end_point.y = 4503969.09
      // alpha_L = -0.08913 radianes
      let start_point = Point{x:433067.939, y: 4503906.797};
      let start_azimuth = 281.031;
      let start_radius = 0.0;
      let end_radius = 450.0;
      let length = 80.222;
      HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length)
   }
   fn sample_clothoid_direct_negative_radius() -> HClothoid {
      // start_radius = 0, end_radius= -450
      // parameter = 190,
      // end_point.x = 431675.544
      // end_point.y = 4504330.311
      // alpha_L = 0.08913 radianes
      let start_point = Point{x:431771.661, y: 4504338.386};
      let start_azimuth = 307.174;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let length = 80.222;
      HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length)
   }

   fn sample_clothoid_inverse_positive_radius() -> HClothoid {
      // start_radius = 450, end_radius= 0
      // parameter = 190,
      // end_point.x = 431114.709
      // end_point.y = 4504298.492
      // alpha_L = -0.08913 radianes
      let start_point = Point{x:431192.883, y: 4504280.607};
      let start_azimuth = 310.535;
      let start_radius = 450.0;
      let end_radius = 0.0;
      let length = 80.222;
      HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length)
   }
   fn sample_clothoid_inverse_negative_radius() -> HClothoid {
      // start_radius = -450, end_radius= 0
      // parameter = 190,
      // end_point.x = 430654.005
      // end_point.y = 4504200.712
      // alpha_L = -0.08913 radianes
      let start_point = Point{x:430859.986, y: 4504269.405};
      let start_azimuth = 285.182;
      let start_radius = -450.0;
      let end_radius = 0.0;
      let length = 80.222;
      HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length)
   }
   #[test]
   fn test_center_x() {
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, 250.0, 90.0);
      assert!(eq001(44.951, hcl.center_x()));
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, -250.0, 90.0);
      assert!(eq001(44.951, hcl.center_x()));
   }
   #[test]
   fn test_center_y() {
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, 250.0, 90.0);
      assert!(eq001(-251.349, hcl.center_y()));
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, -250.0, 90.0);
      assert!(eq001(251.349, hcl.center_y()));
   }
   #[test]
   fn test_center() {
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, 250.0, 90.0);
      assert!(eq001(44.951, hcl.center_x()));
      assert!(eq001(-251.349, hcl.center_y()));
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, -250.0, 90.0);
      assert!(eq001(44.951, hcl.center_x()));
      assert!(eq001(251.349, hcl.center_y()));      
   }
   #[test]
   fn test_retranqueo() {
      let cl = sample_clothoid_direct_positive_radius();
      let ret = cl.retranqueo();
      assert!(eq001(ret, 0.5959));
      assert!(eq001(ret, cl.length*cl.length/24.0/cl.radius_in_tangent_to_circle_point()));

      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, 250.0, 90.0);
      assert!(eq001(1.349, hcl.retranqueo()));
      let hcl = HClothoid::new(Point{x:0.0, y: 0.0}, 90.0, 0.0, -250.0, 90.0);
      assert!(eq001(1.349, hcl.retranqueo()));
   }
   #[test]
   fn test_end_point() {
      todo!();
      // let cl = HClothoid{start_point: Point{x:433067.939, y:4503906.8}, start_azimuth: 281.031, start_radius: 0.0,
      //    end_radius: 450.0, length: 80.222};
      // let end_point = cl.end_point();
      // println!("end_point.x={} end_point.y={}", end_point.x, end_point.y);
      // println!("xc={} yc={} ret={}", cl.center().x, cl.center().y, cl.retranqueo());

      // let cl = sample_clothoid_direct_positive_radius();
      // let end_point = cl.end_point();
      // println!("{} {}", end_point.x, end_point.y);
      // println!("{} {}", end_point.x, end_point.y);
      // assert!(eq001(end_point.x, 432730.377));
      // assert!(eq001(end_point.y, 4503969.09));   
   }
   #[test]
   #[should_panic]
   /// Tests if method new() panics when it 
   /// receives start_radius==0 && end_radius==0
   fn test_panic_1_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 0.0;
      let length = 80.22;
      let _cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
   }
   #[test]
   #[should_panic]
   /// One of the radius must be zero
   fn test_panic_2_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 90.0;
      let start_radius = -400.0;
      let end_radius = 400.0;
      let length = 0.0;
      let _cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
   }
   #[test]
   #[should_panic]
   /// Length can't be zero
   fn test_panic_3_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 90.0;
      let start_radius = 0.0;
      let end_radius = 400.0;
      let length = 0.0;
      let _cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
   }
   #[test]
   fn test_new() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(start_point.x, cl.start_point.x));
      assert_eq!(true, eq001(start_point.y, cl.start_point.y));
      assert_eq!(true, eq001(start_azimuth, cl.start_azimuth));
      assert_eq!(true, eq001(start_radius, cl.start_radius));
      assert_eq!(true, eq001(end_radius, cl.end_radius));
      assert_eq!(true, eq001(length, cl.length));
   }
   #[test]
   fn test_end_azimuth() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = 450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(0.08913, cl.end_azimuth()));
      // end azimuth greater than 2PI
      let start_azimuth = 2.0*PI - 0.05;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(0.03913, cl.end_azimuth()));
      // end azimuth less than zero
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let length = 80.22;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert_eq!(true, eq001(6.194055, cl.end_azimuth()));     
   }
   #[test]
   fn test_radius_in_tangent_to_circle_point() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let length = 80.22;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length); 
      assert!(eq001(cl.end_radius, cl.radius_in_tangent_to_circle_point()));
      let end_radius = 450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length);
      assert!(eq001(cl.end_radius, cl.radius_in_tangent_to_circle_point()));
      let start_radius = 450.0;
      let end_radius = 0.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length); 
      assert!(eq001(cl.start_radius, cl.radius_in_tangent_to_circle_point()));
      let start_radius = -450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, end_radius, length); 
      assert!(eq001(cl.start_radius, cl.radius_in_tangent_to_circle_point()));
   }
   #[test]
   fn test_parameter() {
      let start_point = Point::new(0.0,0.0);
      let start_azimuth = 0.0;
      let length = 80.22;
      let start_radius = 0.0;
      let end_radius = -450.0;
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      let a = (cl.length*cl.end_radius.abs()).sqrt();
      assert!(eq001(a,  cl.parameter()));
      let end_radius = 450.0;
      let a = (cl.length*cl.end_radius.abs()).sqrt();
      let cl = HClothoid::new(start_point, start_azimuth, start_radius, 
         end_radius, length); 
      assert!(eq001(a, cl.parameter()));
      let cl = sample_clothoid_direct_negative_radius();
      assert!(eq001(cl.parameter(), 190.0));
      let cl = sample_clothoid_direct_positive_radius();
      assert!(eq001(cl.parameter(), 190.0));
      let cl = sample_clothoid_inverse_negative_radius();
      assert!(eq001(cl.parameter(), 190.0));
      let cl = sample_clothoid_inverse_positive_radius();
      assert!(eq001(cl.parameter(), 190.0));
      
   }
   #[test]
   fn test_clothoid() {
      todo!();
   }
   #[test]
   fn test_alpha_l() {
      let cl = sample_clothoid_direct_positive_radius();
      assert!(eq001(cl.alpha_l(), -0.08913));
      let cl = sample_clothoid_direct_negative_radius();
      assert!(eq001(cl.alpha_l(), 0.08913));
      let cl = sample_clothoid_inverse_positive_radius();
      assert!(eq001(cl.alpha_l(), -0.08913));
      let cl = sample_clothoid_inverse_negative_radius();
      assert!(eq001(cl.alpha_l(), 0.08913));
   }

}