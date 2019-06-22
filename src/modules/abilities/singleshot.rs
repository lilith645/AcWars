use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::Ship;
use crate::modules::projectiles::Ftpl;

use cgmath::{Vector2, InnerSpace};

pub struct SingleShot {
  data: AbilityData,
}

impl SingleShot {
  pub fn new() -> SingleShot {
    SingleShot {
      data: AbilityData::new_active(0.25),
    }
  }
}

impl Ability for SingleShot {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn applied_to(&self, ship: &mut Ship, mut mouse_pos: Vector2<f32>, window_size: Vector2<f32>) {
    let ship_pos = ship.position();
    let ship_size = ship.size();
    let ship_offset = ship_pos-window_size*0.5;
    mouse_pos += ship_offset;
    
    let proj_dir = (mouse_pos-ship_pos).normalize();
    
    let projectile = Ftpl::new(ship_pos, ship_size*0.5, proj_dir);
    ship.fire_projectile(Box::new(projectile));
  }
}
