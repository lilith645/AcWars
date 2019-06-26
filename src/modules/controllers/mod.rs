pub use self::ability_spam_ai::AbilitySpamAi;

mod ability_spam_ai;

use crate::modules::abilities::{Ability};
use crate::modules::entities::Entity;

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct EntityControllerData {
  abilities: Vec<Box<Ability>>,
}

impl EntityControllerData {
  pub fn new() -> EntityControllerData {
    EntityControllerData {
      abilities: Vec::new(),
    }
  }
  
  pub fn with_ability(mut self, ability: Box<Ability>) -> EntityControllerData {
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
  
  fn update(&mut self, ship: &mut Box<Entity>, target: Vector2<f32>, window_size: Vector2<f32>, delta_time: f32);
}
