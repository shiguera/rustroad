// Permitir a nivel crate la existencia de código
// no utilizado sin emitir el warning al compilar
#![allow(dead_code)]

use float_cmp::approx_eq;
use std::f64::consts::PI;

mod geom;
mod road;

pub fn eq(x:f64, y:f64) -> bool {
   approx_eq!(f64, x, y, ulps=2)
}
pub fn assert_eq001(x:f64, y:f64) -> bool {
   // Checks if two numbers differ in less than 0.001
   if (x-y).abs() < 0.001 {
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
pub fn deg2rad(value360: f64) -> f64 {
   value360*PI/180.0
}
pub fn rad2deg(valuerad: f64) -> f64 {
   valuerad*180.0/PI
}
#[cfg(test)]
mod tests {
   use super::*;
   #[test]
    fn test_eq() {
      let x = 0.15+0.15+0.15+0.15;
      let y = 0.20+0.20+0.20;
      println!("x={} y={} x==y {}", x, y, x==y);
      assert_eq!(true, eq(0.15+0.15+0.15+0.15, 0.20+0.20+0.20));
    }

    #[test]
    fn test_eq001() {
       let x = 1.0;
       let y = 1.0001;
       assert_eq!(true, assert_eq001(x,y));
       let y = 0.9999;
       assert_eq!(true, assert_eq001(x,y));
       let x = -1.0;
       let y = -1.0001;
       assert_eq!(true, assert_eq001(x,y));
       let y = -0.9999;
       assert_eq!(true, assert_eq001(x,y));
       let x = 0.000005;
       let y = -0.000005;
       assert_eq!(true, assert_eq001(x,y));
       
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
    fn test_rad2deg() {
       let x = 0.0f64;
       assert_eq001(0.0, rad2deg(x));
       let x = 1.5;
       assert_eq001(180.0/PI*1.5, rad2deg(x));
    }

    #[test]
    fn test_deg2rad() {
       let x = 0.0f64;
       assert_eq001(0.0, deg2rad(x));
       let x = 45.0;
       assert_eq001(PI/4.0, deg2rad(x));
    }
    #[test]
    fn test_normalize_radian() {
       assert_eq001(normalize_radian(0f64), 0.0f64);   
       assert_eq001(normalize_radian(deg2rad(45.0)), deg2rad(45.0));
       assert_eq001(normalize_radian(deg2rad(405.50)), deg2rad(45.50));
       assert_eq001(normalize_radian(deg2rad(765.0)), deg2rad(45.0));
       assert_eq001(normalize_radian(deg2rad(-45.0)), deg2rad(315.0));
       assert_eq001(normalize_radian(deg2rad(-450.0)), deg2rad(270.0));
       assert_eq001(normalize_radian(deg2rad(-765.0)), deg2rad(315.0));               
    }
   
}
