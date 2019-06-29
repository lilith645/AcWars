use maat_graphics::math;

use crate::modules::buffs::{Buff, BuffData};

use crate::modules::entities::Entity;

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct SpeedBuff {
  data: BuffData,
}

impl SpeedBuff {
  pub fn new() -> SpeedBuff {
    SpeedBuff {
      data: BuffData::new(),
    }
  }
  
  pub fn with_timer(mut self, timer: f32) -> SpeedBuff {
    self.data = self.data.with_timer(timer);
    self
  }
}

impl Buff for SpeedBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn reapply_buff(&self, entity: &mut Box<Entity>) {
    
  }
  
  fn apply_buff(&self, entity: &mut Box<Entity>) {
    let rotation = math::to_radians(entity.rotation());
    let max_vel = entity.max_velocity();
    
    let dir = math::rotate_vector2(Vector2::new(rotation.cos(), rotation.sin()), 90.0);
    
    entity.set_velocity(dir*max_vel);
  }
  
  fn unapply_buff(&self, entity: &mut Box<Entity>) {
    let dir = math::normalise_vector2(entity.velocity());
    let max_vel = entity.max_velocity();
    entity.set_velocity(dir*max_vel);
  }
}
