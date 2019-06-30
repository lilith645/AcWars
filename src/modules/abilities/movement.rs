use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{Entity, Hostility};
use crate::modules::projectiles::Projectile;

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct Move {
  data: AbilityData,
}

impl Move {
  pub fn new() -> Move {
    Move {
      data: AbilityData::new_active("MoveIcon".to_string(), 0.001),
    }
  }
}

impl Ability for Move {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    
    let mut direction = math::normalise_vector2(target-ship_pos);
    
    ship.apply_acceleration_in_direction(direction);
  }
}
