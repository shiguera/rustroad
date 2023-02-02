/// Eje de una carretera
/// Es un vector de alineaciones hsection
/// 
/// 
use crate::geom::point::Point;
use crate::road::htangent::HTangent;
use crate::road::hclothoid::HClothoid;
use crate::road::hcircle::HCircle;
use crate::road::hsection::HSection;

pub struct HAlignment {
    pub sections: Vec<Box<dyn HSection>>
}

impl HAlignment {
    pub fn new() -> Self {
        HAlignment{sections: Vec::<Box<dyn HSection>>::new()}
    }
    pub fn add(&mut self, box_section: Box<dyn HSection>) {
        self.sections.push(box_section);
    }
    pub fn add_htangent(&mut self, tangent: HTangent) {
        self.add(Box::new(tangent));
    }
    pub fn add_hcircle(&mut self, circle: HCircle) {
        self.add(Box::new(circle));
    }
    pub fn add_hclothoid(&mut self, clothoid: HClothoid) {
        self.add(Box::new(clothoid));
    }
    pub fn len(&self) -> usize {
        self.sections.len()
    }

}

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let axis = HAlignment::new();
        assert!(axis.len()==0);
    }
    #[test]
    fn test_len() {
        let mut axis = HAlignment::new();
        let p1 = Point::new(1.0,1.0);
        let r1 = HTangent::new(p1, 45.0, 100.0);
        let end_point = r1.end_point();
        axis.add(Box::new(r1));
        assert!(axis.len()==1);
        let r2 = HTangent::new(end_point, 45.0, 100.0);
        axis.add(Box::new(r2));
        assert!(axis.len()==2);       
    }
}