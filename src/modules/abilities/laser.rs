use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{Entity, BoxEntity, Hostility};
use crate::modules::projectiles::{Projectile, BoxProjectile, LaserBeam};

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct Laser {
  data: AbilityData,
}

impl Laser {
  pub fn new() -> Laser {
    Laser {
      data: AbilityData::new_active("LaserBeamIcon".to_string(), 1.25),
    }
  }
}

impl Ability for Laser {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut BoxProjectile) {
    
  }
  
  fn applied_to(&self, ship: &mut BoxEntity, target: Vector2<f32>, window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    let ship_size = ship.size();
    
    let proj_dir = math::normalise_vector2(target-ship_pos);
    
    let mut projectile: Box<Projectile> = Box::new(LaserBeam::new(ship_pos, ship_size*0.5, proj_dir));
    
    self.apply_passive_abilities(&mut projectile);
    
    ship.fire_projectile(projectile);
  }
}
