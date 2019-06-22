pub use singleshot::SingleShot;
pub use movement::Move;

mod singleshot;
mod movement;

use crate::modules::Ship;

use cgmath::Vector2;

pub enum AbilityType {
  Active,
  Passive,
}

pub struct AbilityData {
  ability_type: AbilityType,
  timer: f32,
  time_left: f32,
}

impl AbilityData {
  pub fn new_active(timer: f32) -> AbilityData {
    AbilityData {
      ability_type: AbilityType::Active,
      timer,
      time_left: 0.0,
    }
  }
  
  pub fn new_passive(timer: f32) -> AbilityData {
    AbilityData {
      ability_type: AbilityType::Passive,
      timer,
      time_left: 0.0,
    }
  }
}

pub trait Ability {
  fn data(&self) -> &AbilityData;
  fn mut_data(&mut self) -> &mut AbilityData;
  
  fn update(&mut self, delta_time: f32) {
    self.mut_data().time_left -= delta_time;
  }
  
  fn ability_type(&self) -> &AbilityType {
    &self.data().ability_type
  }
  
  fn activate(&mut self, ship: &mut Ship, mouse_pos: Vector2<f32>, window_size: Vector2<f32>) {
    if self.can_activate() {
      self.applied_to(ship, mouse_pos, window_size);
      self.mut_data().time_left = self.data().timer;
    }
  }
  
  fn can_activate(&self) -> bool {
    (self.data().time_left <= 0.0)
  }
  
  fn applied_to(&self, ship: &mut Ship, mouse_pos: Vector2<f32>, window_size: Vector2<f32>);
}
