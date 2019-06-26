use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::Entity;
use crate::modules::projectiles::Projectile;

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct Shield {
  data: AbilityData,
}

impl Shield {
  pub fn new() -> Shield {
    Shield {
      data: AbilityData::new_active("ShieldIcon".to_string(), 5.0),
    }
  }
}

impl Ability for Shield {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>) {
    ship.gain_shield(50.0);
  }
}
