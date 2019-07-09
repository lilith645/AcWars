use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{BoxEntity, Hostility};
use crate::modules::projectiles::{BoxProjectile};

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct NoAbility {
  data: AbilityData,
}

impl NoAbility {
  pub fn new() -> NoAbility {
    NoAbility {
      data: AbilityData::new_active("NoAbilityIcon".to_string(), 0.001),
    }
  }
}

impl Ability for NoAbility {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, _projectile: &mut BoxProjectile) {
    
  }
  
  fn applied_to(&self, _ship: &mut BoxEntity, _target: Vector2<f32>, _window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    
  }
}
