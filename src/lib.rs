// Permitir a nivel crate la existencia de código
// no utilizado sin emitir el warning al compilar
#![allow(dead_code)]

use float_cmp::approx_eq;

mod geom;
mod road;

pub fn eq(x:f64, y:f64) -> bool {
   approx_eq!(f64, x, y, ulps=2)
}
fn factorial(num: u64) -> u64 {
   match num {
       0 => 1,
       1 => 1,
       _ => factorial(num - 1) * num,
   }
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
