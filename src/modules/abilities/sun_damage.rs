use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{Entity, Hostility};
use crate::modules::projectiles::{Projectile, Aoe};

use crate::cgmath::{Vector2, InnerSpace};

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
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    let radius = ship.size().x.max(ship.size().y) * 0.75;
    
    let mut projectile: Box<Projectile> = Box::new(Aoe::new(ship_pos, radius));
    
    self.apply_passive_abilities(&mut projectile);
    
    ship.fire_projectile(projectile);
  }
}
