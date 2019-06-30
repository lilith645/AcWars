use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{Entity, Hostility};
use crate::modules::projectiles::Projectile;

use crate::cgmath::{Vector2, InnerSpace};

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
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    
  }
}
