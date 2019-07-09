

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{BoxEntity, Hostility};
use crate::modules::projectiles::{BoxProjectile, Aoe};

use crate::cgmath::{Vector2};

#[derive(Clone)]
pub struct SunDamage {
  data: AbilityData,
}

impl SunDamage {
  pub fn new() -> SunDamage {
    SunDamage {
      data: AbilityData::new_active("SunDamage".to_string(), 0.05),
    }
  }
}

impl Ability for SunDamage {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, _projectile: &mut BoxProjectile) {
    
  }
  
  fn applied_to(&self, ship: &mut BoxEntity, _target: Vector2<f32>, _window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    let radius = ship.size().x.max(ship.size().y) * 0.75;
    
    let mut projectile: BoxProjectile = Box::new(Aoe::new(ship_pos, radius));
    
    self.apply_passive_abilities(&mut projectile);
    
    ship.fire_projectile(projectile);
  }
}
