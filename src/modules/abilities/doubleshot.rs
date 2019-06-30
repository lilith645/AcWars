use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{Entity, Hostility};
use crate::modules::projectiles::{Projectile, Ftpl};

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct DoubleShot {
  data: AbilityData,
}

impl DoubleShot {
  pub fn new() -> DoubleShot {
    DoubleShot {
      data: AbilityData::new_active("DoubleShotIcon".to_string(), 0.15),
    }
  }
}

impl Ability for DoubleShot {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, mut target: Vector2<f32>, window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    let ship_size = ship.size();
    
    let proj_dir = math::normalise_vector2(target-ship_pos);
    
    let left_normal = Vector2::new(-proj_dir.y, proj_dir.x);
    let right_normal = Vector2::new(proj_dir.y, -proj_dir.x);
    
    let mut projectile0: Box<Projectile> = Box::new(Ftpl::new(ship_pos+left_normal*25.0, ship_size*0.5, proj_dir));
    let mut projectile1: Box<Projectile> = Box::new(Ftpl::new(ship_pos+right_normal*25.0, ship_size*0.5, proj_dir));
    
    self.apply_passive_abilities(&mut projectile0);
    self.apply_passive_abilities(&mut projectile1);
    
    ship.fire_projectile(projectile0);
    ship.fire_projectile(projectile1);
  }
}
