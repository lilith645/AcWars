use maat_graphics::DrawCall;

use crate::modules::entities::sections::{ShipSection, ShipSectionData};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct Wing {
  data: ShipSectionData,
}

impl Wing {
  pub fn new(offset: Vector2<f32>, size: Vector2<f32>) -> Wing {
    Wing {
      data: ShipSectionData::new(offset, size),
    }
  }
}

impl ShipSection for Wing {
  fn data(&self) -> &ShipSectionData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut ShipSectionData {
    &mut self.data
  }
  
  fn draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    
  }
}


