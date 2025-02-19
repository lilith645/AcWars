pub use self::ability_spam_ai::AbilitySpamAi;
pub use self::idle_ai::IdleAi;
pub use self::floating_ai::FloatingAi;

mod ability_spam_ai;
mod idle_ai;
mod floating_ai;

use crate::modules::abilities::{BoxAbility};
use crate::modules::entities::{BoxEntity};

use crate::cgmath::Vector2;

pub type BoxEntityController = Box<EntityController>;

#[derive(Clone)]
pub struct EntityControllerData {
  abilities: Vec<BoxAbility>,
}

impl EntityControllerData {
  pub fn new() -> EntityControllerData {
    EntityControllerData {
      abilities: Vec::new(),
    }
  }
  
  pub fn with_ability(mut self, ability: BoxAbility) -> EntityControllerData {
    self.abilities.push(ability);
    self
  }
}

pub trait EntityControllerClone {
  fn clone_entity_controller(&self) -> Box<EntityController>;
}

impl<T: 'static + EntityController + Clone> EntityControllerClone for T {
  fn clone_entity_controller(&self) -> Box<EntityController> {
    Box::new(self.clone())
  }
}

impl Clone for Box<EntityController> {
  fn clone(&self) -> Box<EntityController> {
    self.clone_entity_controller()
  }
}

pub trait EntityController: EntityControllerClone {
  fn data(&self) -> &EntityControllerData;
  fn mut_data(&mut self) -> &mut EntityControllerData;
  
  fn update(&mut self, ship: &mut BoxEntity, target: Vector2<f32>, area_pos: Vector2<f32>, area_size: Vector2<f32>, window_size: Vector2<f32>, delta_time: f32);
}
