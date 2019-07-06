use maat_graphics::math;

use crate::modules::buffs::{Buff, BuffData};

use crate::modules::entities::{Entity, BoxEntity};

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct PhaseBuff {
  data: BuffData,
}

impl PhaseBuff {
  pub fn new() -> PhaseBuff {
    PhaseBuff {
      data: BuffData::new(),
    }
  }
  
  pub fn with_timer(mut self, timer: f32) -> PhaseBuff {
    self.data = self.data.with_timer(timer);
    self
  }
}

impl Buff for PhaseBuff {
  fn data(&self) -> &BuffData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut BuffData {
    &mut self.data
  }
  
  fn reapply_buff(&self, entity: &mut BoxEntity) {
    
  }
  
  fn apply_buff(&self, entity: &mut BoxEntity) {
    entity.set_phase_mode(true);
  }
  
  fn unapply_buff(&self, entity: &mut BoxEntity) {
    entity.set_phase_mode(false);
  }
}
