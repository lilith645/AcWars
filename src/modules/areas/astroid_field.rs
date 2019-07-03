use crate::modules::areas::{Area, AreaData};
use crate::modules::entities::{FullEntity, Astroid};
use crate::modules::controllers::{IdleAi, FloatingAi};
use crate::modules::abilities::{Ability, SingleShot, Shatter, SunDamage};

use crate::cgmath::Vector2;

use std::sync::Arc;

#[derive(Clone)]
pub struct AstroidField {
  data: AreaData,
}

impl AstroidField {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> AstroidField {
    let astroid1 = FullEntity::new(Box::new(FloatingAi::new()), 
                                   Box::new(Astroid::new(position, Vector2::new(750.0, 750.0)).as_hostile()));
    
    let astroid2 = FullEntity::new(Box::new(FloatingAi::new()), 
                                   Box::new(Astroid::new(Vector2::new(-500.0, -1500.0), 
                                                         Vector2::new(200.0, 200.0)).as_hostile()));
    
    AstroidField {
      data: AreaData::new(position, size)
                      .with_entity(astroid1)
                      .with_entity(astroid2),
    }
  }
}

impl Area for AstroidField {
  fn data(&self) -> &AreaData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut AreaData {
    &mut self.data
  }
  
  fn update_area(&mut self, delta_time: f32) {
    
  }
}
