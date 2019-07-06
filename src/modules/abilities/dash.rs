use maat_graphics::math;

use crate::modules::abilities::{Ability, AbilityData};
use crate::modules::entities::{Entity, BoxEntity, Hostility};
use crate::modules::projectiles::{Projectile, BoxProjectile};
use crate::modules::buffs::{MaxSpeedBuff, SpeedBuff, PhaseBuff};

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
  
  fn apply_passive_effect(&self, projectile: &mut BoxProjectile) {
    
  }
  
  fn applied_to(&self, ship: &mut BoxEntity, target: Vector2<f32>, window_size: Vector2<f32>, _parent_hostility: &Hostility) {
    let ship_pos = ship.position();
    
    let direction = math::normalise_vector2(target-ship_pos);
    
    let timer = 0.25;
    ship.activate_buff(Box::new(MaxSpeedBuff::new().with_timer(timer).with_multiplier(5.0)));
    ship.activate_buff(Box::new(SpeedBuff::new().with_timer(timer)));
    ship.activate_buff(Box::new(PhaseBuff::new().with_timer(timer)));
  }
}
