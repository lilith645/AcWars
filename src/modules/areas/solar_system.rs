use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Sun};
use crate::modules::controllers::AbilitySpamAi;

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct SolarSystem {
  data: AreaData,
}

impl SolarSystem {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> SolarSystem {
    let sun = FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Sun::new(Vector2::new(840.0, 1500.0)).as_hostile()),
          buffs: Vec::new(),
    };
    
    SolarSystem {
      data: AreaData::new(position, size)
                      .with_entity(sun),
    }
  }
}

impl Area for SolarSystem {
  fn data(&self) -> &AreaData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AreaData {
    &mut self.data
  }
  
  fn update_area(&mut self, delta_time: f32) {
    
  }
}
