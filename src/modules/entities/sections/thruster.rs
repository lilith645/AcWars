use maat_graphics::DrawCall;

use crate::modules::entities::sections::{ShipSection, ShipSectionData};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct Thruster {
  data: ShipSectionData,
}

impl Thruster {
  pub fn new(offset: Vector2<f32>, size: Vector2<f32>) -> Thruster {
    Thruster {
      data: ShipSectionData::new(offset, size),
    }
  }
}

impl ShipSection for Thruster {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
  
  fn draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}


