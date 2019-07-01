use maat_graphics::math;

use crate::modules::entities::Entity;
use crate::modules::controllers::{EntityController, EntityControllerData};
use crate::modules::abilities::Ability;

use crate::cgmath::{Vector2, InnerSpace};

#[derive(Clone)]
pub struct FloatingAi {
  data: EntityControllerData,
  rotation: f32,
  direction: Vector2<f32>,
}

impl FloatingAi {
  pub fn new() -> FloatingAi {
    FloatingAi {
      data: EntityControllerData::new(),
      rotation: 60.0,
      direction: Vector2::new(-0.5, 0.5),
    }
  }
  
  pub fn with_ability(mut self, ability: Box<Ability>) -> FloatingAi {
    self.data = self.data.with_ability(ability);
    self
  }
}

impl EntityController for FloatingAi {
  fn data(&self) -> &EntityControllerData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityControllerData {
    &mut self.data
  }
  
  fn update(&mut self, ship: &mut Box<Entity>, target: Vector2<f32>, area_pos: Vector2<f32>, area_size: Vector2<f32>, window_size: Vector2<f32>, delta_time: f32) {
    let position = ship.position();
    
    if (position - area_pos).magnitude() > (area_size*0.5).magnitude() {
      self.direction = math::rotate_vector2(math::normalise_vector2(area_pos-position), 35.0);
     // self.rotation *= -1.0;
    }
    
    ship.apply_acceleration_in_direction(self.direction);
    
    let rotation = ship.rotation();
    
    ship.set_rotation(rotation + self.rotation*delta_time);
  }
}
