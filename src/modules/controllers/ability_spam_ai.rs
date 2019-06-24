use crate::modules::entities::Entity;
use crate::modules::controllers::{EntityController, EntityControllerData};
use crate::modules::abilities::SingleShot;

use cgmath::Vector2;

#[derive(Clone)]
pub struct AbilitySpamAi {
  data: EntityControllerData,
}

impl AbilitySpamAi {
  pub fn new() -> AbilitySpamAi {
    AbilitySpamAi {
      data: EntityControllerData::new()
                                  .with_ability(Box::new(SingleShot::new())) 
    }
  }
}

impl EntityController for AbilitySpamAi {
  fn data(&self) -> &EntityControllerData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityControllerData {
    &mut self.data
  }
  
  fn update(&mut self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>, delta_time: f32) {
    for ability in &mut self.mut_data().abilities {
      ability.update(delta_time);
      ability.activate(ship, target, window_size);
    }
    
    ship.set_facing(target);
  }
}
