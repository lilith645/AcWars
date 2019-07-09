use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{BoxEntity, Hostility};
use crate::modules::projectiles::{BoxProjectile, Gob};

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct SingleShot {
  data: AbilityData,
}

impl SingleShot {
  pub fn new() -> SingleShot {
    SingleShot {
      data: AbilityData::new_active("SingleShotIcon".to_string(), 0.75),
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
  
  fn apply_passive_effect(&self, _projectile: &mut BoxProjectile) {
    
  }
  
  fn applied_to(&self, ship: &mut BoxEntity, target: Vector2<f32>, _window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    let ship_size = ship.size();
    
    let proj_dir = math::normalise_vector2(target-ship_pos);
    
    let mut projectile: BoxProjectile = Box::new(Gob::new(ship_pos, ship_size*0.5, proj_dir));
    
    self.apply_passive_abilities(&mut projectile);
    
    ship.fire_projectile(projectile);
  }
}
