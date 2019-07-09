use maat_graphics::math;

use crate::modules::entities::{BoxEntity};
use crate::modules::controllers::{EntityController, EntityControllerData};
use crate::modules::abilities::{BoxAbility};

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct AbilitySpamAi {
  data: EntityControllerData,
}

impl AbilitySpamAi {
  pub fn new() -> AbilitySpamAi {
    AbilitySpamAi {
      data: EntityControllerData::new()
    }
  }
  
  pub fn with_ability(mut self, ability: BoxAbility) -> AbilitySpamAi {
    self.data = self.data.with_ability(ability);
    self
  }
}

impl EntityController for AbilitySpamAi {
  fn data(&self) -> &EntityControllerData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityControllerData {
    &mut self.data
  }
  
  fn update(&mut self, ship: &mut BoxEntity, target: Vector2<f32>, _area_pos: Vector2<f32>, _area_size: Vector2<f32>, window_size: Vector2<f32>, delta_time: f32) {
    let hostility = ship.hostility().clone();
    
    for ability in &mut self.mut_data().abilities {
      ability.update(delta_time);
      ability.activate(ship, target, window_size, &hostility);
    }
    
    let mut vel_dir = target - ship.position();
    if vel_dir.magnitude() < 400.0 {
      vel_dir = math::rotate_vector2(vel_dir, 90.0);
    }
    ship.apply_acceleration_in_direction(vel_dir);
    
    ship.set_facing(target);
  }
}
