use float_cmp::approx_eq;

pub struct Line {
   // Straight ine in the form ax + by + c = 0
   pub a: f64,
   pub b: f64,
   pub c: f64
}
impl Line {
   pub fn new(a:f64, b:f64, c:f64) -> Self {
      Line{a,b,c}
   }
}

mod tests {
   #[cfg(test)]
   use super::*;

   #[test] 
   fn test_new() {
      let r1 = Line::new(1.0,1.0,1.0);
      assert_eq!(true, approx_eq!(f64, r1.a, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r1.b, 1.0, ulps=2));
      assert_eq!(true, approx_eq!(f64, r1.c, 1.0, ulps=2));
      
   }
}