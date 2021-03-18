// 2 D vector
pub struct Vector {
   pub x: f64,
   pub y: f64
}

impl Vector {
   pub fn new(x: f64, y: f64) -> Self {
      Vector{x,y}
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test]
   fn test_new() {
      let v = Vector::new(1.0,-1.0);
      assert_eq!(true, v.x-1.0 < 0.001);
      assert_eq!(true, v.y-(-1.0) < 0.001);
   }
}
