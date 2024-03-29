// Permitir a nivel crate la existencia de código
// no utilizado sin emitir el warning al compilar
#![allow(dead_code)]

use std::f64::consts::PI;

mod geom;
mod road;

// Value used in equal comparisons with eq001
const EPS:f64 = 1e-8;

/// Compares two f64 and returns true if
/// the diference is less than EPS in absolute value
pub fn eq(x: f64, y:f64) -> bool {
   if (x-y).abs() < EPS {
      true
   } else {
      false
   }
}
/// Compares two f64 and returns true if
/// the diference is less than 0.001 in absolute value
pub fn eq001(x:f64, y:f64) -> bool {
   if (x-y).abs() < 0.001 {
      true
   } else {
      false
   }
}
/// Compares two f64 and returns true if
/// the diference is less than 0.1 in absolute value
pub fn eq01(x:f64, y:f64) -> bool {
   if (x-y).abs() < 0.01 {
      true
   } else {
      false
   }
}
pub fn factorial(num: u64) -> u64 {
   match num {
       0 => 1,
       1 => 1,
       _ => factorial(num - 1) * num,
   }
}
/// Devuelve las soluciones de una ecuación de segundo grado ax2 + bx + c = 0
pub fn second_degree_equation(a: f64, b: f64, c: f64) -> (f64, f64) {
   if eq001(a, 0.0) && eq001(b, 0.0) && eq001(c, 0.0) {
      panic!("Coeficientes nulos!");
   }
   let disc = b*b - 4.0*a * c;
   if disc < 0.0 {
      panic!("Discriminante negativo!");
   }
   let disc = disc.sqrt();
   let x1 = (-b + disc)/2.0/a;
   let x2 = (-b - disc)/2.0/a;
   (x1, x2)
}
/// If the angle's absolute value is greater than 2*PI, 
/// it is changed by its equivalent between 0 and 2*PI. 
/// If the angle is negative, it is changed by its 
/// equivalent positive angle
pub fn normalize_radian(rad_value: f64) -> f64 {
   let mut newangle = rad_value;
   if rad_value.abs() > 2.0*PI {
      newangle = rad_value % (2.0*PI);
   }
   if newangle < 0.0 {
      newangle = 2.0*PI + newangle;
   }
   newangle  
}
/// If the angle's absolute value is greater than 360, 
/// it is changed by its equivalent between 0 and 360. 
/// If the angle is negative, it is changed by its 
/// equivalent positive angle
pub fn normalize_360(deg_value: f64) -> f64 {
   let mut newangle = deg_value;
   if deg_value.abs() > 360.0 {
      newangle = deg_value % (360.0);
   }
   if newangle < 0.0 {
      newangle = 360.0 + newangle;
   }
   newangle  
}
/// Converts from degrees to radians
pub fn deg2rad(value360: f64) -> f64 {
   value360*PI/180.0
}
/// Converts from radians to degrees
pub fn rad2deg(valuerad: f64) -> f64 {
   valuerad*180.0/PI
}
/// Converts from gon to degrees
pub fn gon2deg(gon_value: f64) -> f64 {
   gon_value*0.9
}

/// Converts from degrees to gon
pub fn deg2gon(deg_value: f64) -> f64 {
   deg_value*10.0/9.0
}

/// Converts an azimuth value measured from the North
/// toward the East into an angle in radians
/// measured from the East counter-clockwise
pub fn azimuth_to_angle(az: f64) -> f64{
   deg2rad(normalize_360(90.0-az))
}
/// Converts an angle value measured in radians
/// from the East counter-clockwise into an 
/// azimuth in degrees measured from the North 
/// toward the East
pub fn angle_to_azimuth(ang: f64) -> f64{
   rad2deg(normalize_radian(PI/2.0-ang))
}
/// Cambio de ejes de un punto P(x,y) a unos ejes
/// girados theta radianes
pub fn rotation(x:f64, y: f64, theta: f64) -> (f64, f64) {
   let xprim = x*theta.cos() + y*theta.sin();
   let yprim = -x*theta.sin() + y* theta.cos();
   (xprim, yprim)
}


#[cfg(test)]
mod tests {
   use super::*;
   #[test]
    fn test_eq() {
      let x = 1.0;
      let y = 1.0+EPS/2.0;
      assert!(eq001(x,y));
      let y = 1.0-EPS/2.0;
      assert!(eq001(x,y));
      let x = -1.0;
      let y = -1.0-EPS/2.0;
      assert!(eq001(x,y));
      let y = -1.0+EPS/2.0;
      assert!(eq001(x,y));      
      assert!(eq(0.15+0.15+0.15+0.15, 0.20+0.20+0.20));
    }

   #[test]
   fn test_eq001() {
      assert!(eq(0.15+0.15+0.15+0.15, 0.20+0.20+0.20));     
   }
   #[test]
   #[should_panic]
   fn test_eq_panic() {
      let x = 1.0;
      let y = 1.0001;
      assert!(eq(x,y));
   }

   #[test]
   fn test_factorial() {
      let n = 0;
      assert_eq!(true, factorial(n)==1);
      let n = 1;
      assert_eq!(true, factorial(n)==1);
      let n = 5;
      assert_eq!(true, factorial(n)==120);
      let x = factorial(5) as f64;
      println!("{}", x*2.0);
   }
   #[test]
   fn test_second_degree_equation() {
      let (a, b, c) = (1.0f64, 0.0f64, -1.0f64);
      let (x1, x2) = second_degree_equation(a, b, c);
      assert!(eq001(1.0, x1));
      assert!(eq001(-1.0, x2));      

      let (a, b, c) = (1.0f64, 1.0f64, -2.0f64);
      let (x1, x2) = second_degree_equation(a, b, c);
      assert!(eq001(1.0, x1));
      assert!(eq001(-2.0, x2));      
   }
   #[test]
   #[should_panic]
   fn test_second_degree_equation_panic_1() {
      let (a, b, c) = (0.0f64, 0.0f64, 0.0f64);
      let (x1, x2) = second_degree_equation(a, b, c);
      assert!(eq001(0.0, x1));
   }
   #[test]
   #[should_panic]
   fn test_second_degree_equation_panic_2() {
      let (a, b, c) = (10.0f64, 4.0f64, 1.0f64);
      let (x1, x2) = second_degree_equation(a, b, c);
      assert!(eq001(0.0, x1));
   }

   #[test]
   fn test_rad2deg() {
      let x = 0.0f64;
      assert!(eq001(0.0, rad2deg(x)));
      let x = 1.5;
      assert!(eq001(180.0/PI*1.5, rad2deg(x)));
   }

   #[test]
   fn test_deg2rad() {
      let x = 0.0f64;
      assert!(eq001(0.0, deg2rad(x)));
      let x = 45.0;
      assert!(eq001(PI/4.0, deg2rad(x)));
   }
   #[test]
   fn test_normalize_radian() {
      assert!(eq001(normalize_radian(0f64), 0.0f64));   
      assert!(eq001(normalize_radian(deg2rad(45.0)), deg2rad(45.0)));
      assert!(eq001(normalize_radian(deg2rad(405.50)), deg2rad(45.50)));
      assert!(eq001(normalize_radian(deg2rad(765.0)), deg2rad(45.0)));
      assert!(eq001(normalize_radian(deg2rad(-45.0)), deg2rad(315.0)));
      assert!(eq001(normalize_radian(deg2rad(-450.0)), deg2rad(270.0)));
      assert!(eq001(normalize_radian(deg2rad(-765.0)), deg2rad(315.0)));               
   }
   #[test]
   fn test_normalize_360() {
      assert!(eq001(normalize_360(0f64), 0.0f64));   
      assert!(eq001(normalize_360(45.0), 45.0));
      assert!(eq001(normalize_360(405.50), 45.50));
      assert!(eq001(normalize_360(765.0), 45.0));
      assert!(eq001(normalize_360(-45.0), 315.0));
      assert!(eq001(normalize_360(-450.0), 270.0));
      assert!(eq001(normalize_360(-765.0), 315.0));               
   }

   #[test] 
   fn test_gon2deg() {
      let d = 0.0;
      assert!(eq001(0.0, gon2deg(d)));
      let d = 100.0;
      assert!(eq001(90.0, gon2deg(d)));
      let d = -200.0;
      assert!(eq001(-180.0, gon2deg(d)));
      let d = 400.0;
      assert!(eq001(360.0, gon2deg(d)));
   }
   #[test] 
   fn test_deg2gon() {
      let d = 0.0;
      assert!(eq001(0.0, deg2gon(d)));
      let d = 90.0;
      assert!(eq001(100.0, deg2gon(d)));
      let d = -135.0;
      assert!(eq001(-150.0, deg2gon(d)));
      let d = 360.0;
      assert!(eq001(400.0, deg2gon(d)));
   }
   #[test]
   fn test_azimuth_to_angle() {
      let az = 0.0;
      assert!(eq001(azimuth_to_angle(az), deg2rad(90.0)));
      let az = 30.0;
      assert!(eq001(azimuth_to_angle(az), deg2rad(60.0)));
      let az = 60.0;
      assert!(eq001(azimuth_to_angle(az), deg2rad(30.0)));
      let az = 90.0;
      assert!(eq001(azimuth_to_angle(az), deg2rad(0.0)));
      let az = -30.0;
      assert!(eq001(azimuth_to_angle(az), deg2rad(120.0)));
   }
   #[test]
   fn test_angle_to_azimuth() {
      let ang = 0.0;
      assert!(eq001(angle_to_azimuth(ang), 90.0));
      let ang = deg2rad(30.0);
      assert!(eq001(angle_to_azimuth(ang), 60.0));
      let ang = deg2rad(60.0);
      assert!(eq001(angle_to_azimuth(ang), 30.0));
      let ang = deg2rad(90.0);
      assert!(eq001(angle_to_azimuth(ang), 0.0));
      let ang = deg2rad(-30.0);
      assert!(eq001(angle_to_azimuth(ang), 120.0));
   }

   #[test]
   fn test_rotation() {
      let x = 1.0;
      let y = 1.0;
      let theta = deg2rad(45.0);
      let (xprim, yprim) = rotation(x, y, theta);
      assert!(eq001(xprim, 1.4142));
      assert!(eq001(yprim, 0.0));
   }
}
