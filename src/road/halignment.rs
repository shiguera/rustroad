/// Eje de una carretera
/// Es un vector de alineaciones hsection
/// 
/// 
use crate::geom::point::Point;
use crate::road::htangent::HTangent;
use crate::road::hclothoid::HClothoid;
use crate::road::hcircle::HCircle;
use crate::road::hsection::HSection;

pub struct HAlignmentSummarized {
    // Conocido el punto inicial y el azimuth inicial de una alineación
    // con estos tres datos se puede reconstruir la alineación
    start_radius: f64,
    end_radius: f64,
    length: f64
}

pub struct HAlignment {
    // Coordenadas del punto inicial de la alineación horizontal
    pub start_x: f64,
    pub start_y: f64,
    // Azimuth inicial en grados sexagesimales
    pub start_azimuth: f64, 
    pub sections: Vec<HAlignmentSummarized>, 
}

impl HAlignment {
    pub fn new(start_x: f64, start_y: f64, start_azimuth: f64) -> Self {
        HAlignment{start_x, start_y, start_azimuth, sections: Vec::<HAlignmentSummarized>::new()}
    }
    pub fn add(&mut self, section: HAlignmentSummarized) {
        // Tal como está, no funciona bien. Habría que añadir cada alineación
        // en el punto final de la anterior, mediante los parámetros:
        // start_radius, end_radius, length

    }
    pub fn len(&self) -> usize {
        self.sections.len()
    }

}

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let axis = HAlignment::new(0.0, 0.0, 100.0);
        assert!(axis.len()==0);
    }
    #[test]
    fn test_len() {
        let mut axis = HAlignment::new(0.0, 0.0, 100.0);
        assert!(axis.len()==0);
    }
}