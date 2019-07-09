pub use self::singleshot::SingleShot;
pub use self::doubleshot::DoubleShot;
pub use self::movement::Move;
pub use self::laser::Laser;
pub use self::shatter::Shatter;
pub use self::shield::Shield;
pub use self::dash::Dash;
pub use self::haste::Haste;
pub use self::sun_damage::SunDamage;
pub use self::no_ability::NoAbility;

pub use self::projectile_speed::ProjectileSpeed;

// actives
mod singleshot;
mod doubleshot;
mod laser;
mod shield;
mod dash;
mod haste;
mod sun_damage;

// passives
mod projectile_speed;
mod shatter;

// other
mod movement;
mod no_ability;

use maat_graphics::DrawCall;

use crate::modules::entities::{BoxEntity, Hostility};
use crate::modules::projectiles::{BoxProjectile};

use crate::cgmath::{Vector2, Vector4};

pub type BoxAbility = Box<Ability>;

#[derive(Clone, PartialEq)]
pub enum AbilityType {
  Active,
  Passive,
}

#[derive(Clone)]
pub struct AbilityData {
  ability_type: AbilityType,
  texture: String,
  timer: f32,
  time_left: f32,
  passives: Vec<Box<Ability>>,
}

unsafe impl Send for AbilityData {
}
unsafe impl Sync for AbilityData {
}


impl AbilityData {
  pub fn new_active(texture: String, timer: f32) -> AbilityData {
    AbilityData {
      ability_type: AbilityType::Active,
      texture,
      timer,
      time_left: 0.0,
      passives: Vec::new(),
    }
  }
  
  pub fn new_passive(texture: String, timer: f32) -> AbilityData {
    AbilityData {
      ability_type: AbilityType::Passive,
      texture,
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
  
  fn texture(&self) -> String {
    self.data().texture.to_string()
  }
  
  fn ability_type(&self) -> &AbilityType {
    &self.data().ability_type
  }
  
  fn apply_passive_abilities(&self, mut projectile: &mut BoxProjectile) {
    for passive in &self.data().passives {
      passive.apply_passive_effect(&mut projectile);
    }
  }
  
  fn add_passive(&mut self, passive: Box<Ability>) {
    if passive.ability_type() == &AbilityType::Passive {
      self.mut_data().passives.push(passive);
    }
  }
  
  fn activate(&mut self, ship: &mut BoxEntity, target: Vector2<f32>, window_size: Vector2<f32>, parent_hostility: &Hostility) {
    if self.can_activate() {
      self.applied_to(ship, target, window_size, parent_hostility);
      self.mut_data().time_left = self.data().timer;
    }
  }
  
  fn can_activate(&self) -> bool {
    (self.data().ability_type == AbilityType::Active) && (self.data().time_left <= 0.0)
  }
  
  fn applied_to(&self, ship: &mut BoxEntity, target: Vector2<f32>, window_size: Vector2<f32>, parent_hostility: &Hostility);
  fn apply_passive_effect(&self, projectile: &mut BoxProjectile);
  
  fn draw(&self, position: Vector2<f32>, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_textured(position, Vector2::new(50.0, 50.0), 0.0, self.data().texture.to_string()));
    
    let time_left_percentage = self.data().time_left / self.data().timer;
    if time_left_percentage > 0.0 {
      draw_calls.push(DrawCall::draw_coloured(Vector2::new(position.x, position.y), 
                                              Vector2::new(50.0, 50.0*time_left_percentage),
                                              Vector4::new(1.0, 1.0, 1.0, 0.3), 0.0));
    }
  }
}
