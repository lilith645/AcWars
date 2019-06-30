use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Sun};
use crate::modules::controllers::AbilitySpamAi;
use crate::modules::abilities::{Ability, SingleShot, Shatter};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct SolarSystem {
  data: AreaData,
}

impl SolarSystem {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> SolarSystem {
    let mut singleshot = Box::new(SingleShot::new());
    singleshot.add_passive(Box::new(Shatter::new()));
    let sun = FullEntity { 
          ai: Box::new(AbilitySpamAi::new().with_ability(singleshot)), 
          entity: Box::new(Sun::new(Vector2::new(840.0, 1500.0)).as_neutral()),
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
