use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::Entity;
use crate::modules::projectiles::{Projectile, Ftpl};

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct Shatter {
  data: AbilityData,
}

impl Shatter {
  pub fn new() -> Shatter {
    Shatter {
      data: AbilityData::new_passive("ShatterIcon".to_string(), 0.0),
    }
  }
}

impl Ability for Shatter {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    projectile.add_passive(Box::new(self.clone()));
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, _window_size: Vector2<f32>) {
    let ship_pos = ship.position();
    let ship_size = ship.size();
    
    let proj_dir = (target-ship_pos).normalize();
    
    let num_projectiles = 5;
    
    let mut projectiles = Vec::new();
    
    let mut arc = 90.0;
    
    let mut arc_increment = arc/num_projectiles as f32;
    
    // middle projectile
    projectiles.push(Box::new(Ftpl::new(ship_pos, ship_size*0.5, proj_dir)));
    
    let half_projectiles = (num_projectiles as f32*0.5).floor() as usize;
    
    for i in 0..half_projectiles {
      let dir1 = math::rotate_vector2(proj_dir,  (i+1) as f32 *  arc_increment);
      let dir2 = math::rotate_vector2(proj_dir,  (i+1) as f32 * -arc_increment);
      
      projectiles.push(Box::new(Ftpl::new(ship_pos, ship_size*0.5, dir1)));
      projectiles.push(Box::new(Ftpl::new(ship_pos, ship_size*0.5, dir2)));
    }
    
    for projectile in &mut projectiles {
      projectile.lock_hostility();
    }
    
    for projectile in projectiles {
      ship.fire_projectile(projectile)
    }
  }
}
