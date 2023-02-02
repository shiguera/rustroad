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
    

}

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let p1 = Point::new(1.0,1.0);
        let r1 = HTangent::new(p1, 45.0, 100.0);
        let mut axis = HAlignment::new();
        axis.add(Box::new(r1));

    }
}