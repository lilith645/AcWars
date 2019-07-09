use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{BoxEntity, Hostility};
use crate::modules::projectiles::{BoxProjectile};
use crate::modules::buffs::MaxSpeedBuff;

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct Haste {
  data: AbilityData,
}

impl Haste {
  pub fn new() -> Haste {
    Haste {
      data: AbilityData::new_active("HasteIcon".to_string(), 5.0),
    }
  }
}

impl Ability for Haste {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, _projectile: &mut BoxProjectile) {
    
  }
  
  fn applied_to(&self, ship: &mut BoxEntity, _target: Vector2<f32>, _window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    ship.activate_buff(Box::new(MaxSpeedBuff::new()));
  }
}
