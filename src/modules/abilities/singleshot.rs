use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::Entity;
use crate::modules::projectiles::{Projectile, Gob};

use cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
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
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>) {
    let ship_pos = ship.position();
    let ship_size = ship.size();
    
    let proj_dir = (target-ship_pos).normalize();
    
    let mut projectile: Box<Projectile> = Box::new(Gob::new(ship_pos, ship_size*0.5, proj_dir));
    
    self.apply_passive_abilities(&mut projectile);
    
    ship.fire_projectile(projectile);
  }
}
