use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{BoxEntity, Hostility};
use crate::modules::projectiles::{BoxProjectile};

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct ProjectileSpeed {
  data: AbilityData,
}

impl ProjectileSpeed {
  pub fn new() -> ProjectileSpeed {
    ProjectileSpeed {
      data: AbilityData::new_passive("".to_string(), 0.0),
    }
  }
}

impl Ability for ProjectileSpeed {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut BoxProjectile) {
    projectile.multiply_velocity(1.2);
  }
  
  fn applied_to(&self, _ship: &mut BoxEntity, _target: Vector2<f32>, _window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    
  }
}
