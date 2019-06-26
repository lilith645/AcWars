use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::Entity;
use crate::modules::projectiles::Projectile;
use crate::modules::buffs::{MaxSpeedBuff, SpeedBuff};

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct Dash {
  data: AbilityData,
}

impl Dash {
  pub fn new() -> Dash {
    Dash {
      data: AbilityData::new_active("DashIcon".to_string(), 5.0),
    }
  }
}

impl Ability for Dash {
  fn data(&self) -> &AbilityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AbilityData {
    &mut self.data
  }
  
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>) {
    
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>) {
    let ship_pos = ship.position();
    
    let direction = (target-ship_pos).normalize();
    
    let timer = 0.25;
    ship.activate_buff(Box::new(MaxSpeedBuff::new().with_timer(timer).with_multiplier(5.0)));
    ship.activate_buff(Box::new(SpeedBuff::new().with_timer(timer)));
  }
}
