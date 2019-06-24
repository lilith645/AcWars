pub use singleshot::SingleShot;
pub use doubleshot::DoubleShot;
pub use movement::Move;
pub use laser::Laser;
pub use shatter::Shatter;
pub use shield::Shield;

pub use projectile_speed::ProjectileSpeed;

// actives
mod singleshot;
mod doubleshot;
mod laser;
mod shield;

// passives
mod projectile_speed;
mod shatter;

// other
mod movement;

use crate::modules::entities::Entity;
use crate::modules::projectiles::Projectile;

use cgmath::Vector2;

#[derive(Clone, PartialEq)]
pub enum AbilityType {
  Active,
  Passive,
}

#[derive(Clone)]
pub struct AbilityData {
  ability_type: AbilityType,
  timer: f32,
  time_left: f32,
  passives: Vec<Box<Ability>>,
}

impl AbilityData {
  pub fn new_active(timer: f32) -> AbilityData {
    AbilityData {
      ability_type: AbilityType::Active,
      timer,
      time_left: 0.0,
      passives: Vec::new(),
    }
  }
  
  pub fn new_passive(timer: f32) -> AbilityData {
    AbilityData {
      ability_type: AbilityType::Passive,
      timer,
      time_left: 0.0,
      passives: Vec::new(),
    }
  }
}

pub trait AbilityClone {
  fn clone_ability(&self) -> Box<Ability>;
}

impl<T: 'static + Ability + Clone> AbilityClone for T {
  fn clone_ability(&self) -> Box<Ability> {
    Box::new(self.clone())
  }
}

impl Clone for Box<Ability> {
  fn clone(&self) -> Box<Ability> {
    self.clone_ability()
  }
}

pub trait Ability: AbilityClone {
  fn data(&self) -> &AbilityData;
  fn mut_data(&mut self) -> &mut AbilityData;
  
  fn update(&mut self, delta_time: f32) {
    self.mut_data().time_left -= delta_time;
    if self.data().time_left <= 0.0 {
      self.mut_data().time_left = 0.0;
    }
  }
  
  fn ability_type(&self) -> &AbilityType {
    &self.data().ability_type
  }
  
  fn apply_passive_abilities(&self, mut projectile: &mut Box<Projectile>) {
    for passive in &self.data().passives {
      passive.apply_passive_effect(&mut projectile);
    }
  }
  
  fn add_passive(&mut self, passive: Box<Ability>) {
    if passive.ability_type() == &AbilityType::Passive {
      self.mut_data().passives.push(passive);
    }
  }
  
  fn activate(&mut self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>) {
    if self.can_activate() {
      self.applied_to(ship, target, window_size);
      self.mut_data().time_left = self.data().timer;
    }
  }
  
  fn can_activate(&self) -> bool {
    (self.data().ability_type == AbilityType::Active) && (self.data().time_left <= 0.0)
  }
  
  fn applied_to(&self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>);
  fn apply_passive_effect(&self, projectile: &mut Box<Projectile>);
}
