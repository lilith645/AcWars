use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Sun, Astroid};
use crate::modules::controllers::{IdleAi, AbilitySpamAi};
use crate::modules::abilities::{Ability, SingleShot, Shatter, SunDamage};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct SolarSystem {
  data: AreaData,
}

impl SolarSystem {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> SolarSystem {
    let sun = FullEntity { 
          ai: Box::new(IdleAi::new().with_ability(Box::new(SunDamage::new()))), 
          entity: Box::new(Sun::new(Vector2::new(840.0, 1500.0)).as_neutral()),
          buffs: Vec::new(),
    };
    
    let astroid = FullEntity { 
          ai: Box::new(AbilitySpamAi::new()), 
          entity: Box::new(Astroid::new(Vector2::new(-500.0, -1500.0), Vector2::new(100.0, 100.0)).as_hostile()),
          buffs: Vec::new(),
    };
    
    SolarSystem {
      data: AreaData::new(position, size)
                      .with_entity(sun)
                      .with_entity(astroid),
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
