// Permitir a nivel crate la existencia de código
// no utilizado sin emitir el warning al compilar
#![allow(dead_code)]

use float_cmp::approx_eq;

mod geom;
mod road;

pub fn eq(x:f64, y:f64) -> bool {
   approx_eq!(f64, x, y, ulps=2)
}
pub fn eq001(x:f64, y:f64) -> bool {
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
/// This function is a wrapper of Angle::normalize()
pub fn normalize(angle: f64) -> f64 {
   geom::angles::Angle::normalize(angle)   
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
       assert_eq!(true, eq001(x,y));
       let y = 0.9999;
       assert_eq!(true, eq001(x,y));
       let x = -1.0;
       let y = -1.0001;
       assert_eq!(true, eq001(x,y));
       let y = -0.9999;
       assert_eq!(true, eq001(x,y));
       let x = 0.000005;
       let y = -0.000005;
       assert_eq!(true, eq001(x,y));
       
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
}
