use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::Ship;
use crate::modules::projectiles::Ftpl;

use cgmath::{Vector2, InnerSpace};

pub struct Move {
  data: AbilityData,
}

impl Move {
  pub fn new() -> Move {
    Move {
      data: AbilityData::new_active(0.001),
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
  
  fn applied_to(&self, ship: &mut Ship, mut mouse_pos: Vector2<f32>, window_size: Vector2<f32>) {
    let ship_pos = ship.position();
   
    let ship_offset = ship_pos-window_size*0.5;
    mouse_pos += ship_offset;
    
    let direction = (mouse_pos-ship_pos).normalize();
    
    ship.apply_velocity_in_direction(direction);
  }
}
