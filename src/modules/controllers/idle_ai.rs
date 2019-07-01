use maat_graphics::math;

use crate::modules::entities::Entity;
use crate::modules::controllers::{EntityController, EntityControllerData};
use crate::modules::abilities::Ability;

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct IdleAi {
  data: EntityControllerData,
}

impl IdleAi {
  pub fn new() -> IdleAi {
    IdleAi {
      data: EntityControllerData::new()
    }
  }
  
  pub fn with_ability(mut self, ability: Box<Ability>) -> IdleAi {
    self.data = self.data.with_ability(ability);
    self
  }
}

impl EntityController for IdleAi {
  fn data(&self) -> &EntityControllerData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityControllerData {
    &mut self.data
  }
  
  fn update(&mut self, ship: &mut Box<Entity>, target: Vector2<f32>, area_pos: Vector2<f32>, area_size: Vector2<f32>, window_size: Vector2<f32>, delta_time: f32) {
    let hostility = ship.hostility().clone();
    
    for ability in &mut self.mut_data().abilities {
      ability.update(delta_time);
      ability.activate(ship, target, window_size, &hostility);
    }
  }
}
