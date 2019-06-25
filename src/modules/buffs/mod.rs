pub use self::max_speed_buff::MaxSpeedBuff;
pub use self::speed_buff::SpeedBuff;

mod max_speed_buff;
mod speed_buff;

use crate::modules::entities::Entity;

#[derive(Clone)]
pub struct BuffData {
  multiplier: f32,
  timer: f32,
  should_exist: bool,
}

impl BuffData {
  pub fn new() -> BuffData {
    BuffData {
      multiplier: 1.0,
      timer: 5.0,
      should_exist: true,
    }
  }
  
  pub fn with_multiplier(mut self, multi: f32) -> BuffData {
    self.multiplier = multi;
    self
  }
  
  pub fn with_timer(mut self, timer: f32) -> BuffData {
    self.timer = timer;
    self
  }
}

pub trait BuffClone {
  fn clone_buff(&self) -> Box<Buff>;
}

impl<T: 'static + Buff + Clone> BuffClone for T {
  fn clone_buff(&self) -> Box<Buff> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Buff> {
  fn clone(&self) -> Box<Buff> {
    self.clone_buff()
  }
}

pub trait Buff: BuffClone {
  fn data(&self) -> &BuffData;
  fn mut_data(&mut self) -> &mut BuffData;
  
  fn reapply_buff(&self, entity: &mut Box<Entity>);
  
  fn apply_buff(&self, entity: &mut Box<Entity>);
  fn unapply_buff(&self, entity: &mut Box<Entity>);
  
  fn should_exist(&self) -> bool {
    self.data().should_exist
  }
  
  fn update(&mut self, entity: &mut Box<Entity>, delta_time: f32) {
    self.mut_data().timer -= delta_time;
    if self.data().timer <= 0.0 {
      self.mut_data().should_exist = false;
      self.mut_data().timer = 0.0;
    } else {
      self.reapply_buff(entity);
    }
  }
}
