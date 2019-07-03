use crate::modules::buffs::{Buff, BuffData};

use crate::modules::entities::{Entity, BoxEntity};

#[derive(Clone)]
pub struct MaxSpeedBuff {
  data: BuffData,
}

impl MaxSpeedBuff {
  pub fn new() -> MaxSpeedBuff {
    MaxSpeedBuff {
      data: BuffData::new()
                      .with_timer(5.0)
                      .with_multiplier(1.5),
    }
  }
  
  pub fn with_multiplier(mut self, multiplier: f32) -> MaxSpeedBuff {
    self.data = self.data.with_multiplier(multiplier);
    self
  }
  
  pub fn with_timer(mut self, timer: f32) -> MaxSpeedBuff {
    self.data = self.data.with_timer(timer);
    self
  }
}

impl Buff for MaxSpeedBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn reapply_buff(&self, entity: &mut BoxEntity) {
    
  }
  
  fn apply_buff(&self, entity: &mut BoxEntity) {
    let max_vel = entity.max_velocity();
    entity.set_max_velocty(max_vel * self.data().multiplier);
  }
  
  fn unapply_buff(&self, entity: &mut BoxEntity) {
    let max_vel = entity.max_velocity();
    entity.set_max_velocty(max_vel / self.data().multiplier);
  }
}
