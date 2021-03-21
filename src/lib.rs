// Permitir a nivel crate la existencia de código
// no utilizado sin emitir el warning al compilar
#![allow(dead_code)]

use float_cmp::approx_eq;

mod geom;
mod road;

pub fn eq(x:f64, y:f64) -> bool {
   approx_eq!(f64, x, y, ulps=2)
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
}
