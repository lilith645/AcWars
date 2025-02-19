use crate::modules::entities::{Entity, BoxEntity, EntityData};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct Sun {
  data: EntityData,
}

impl Sun {
  pub fn new(position: Vector2<f32>) -> Sun {
     //let position = Vector2::new(-1500.0, 1000.0);
     let size = Vector2::new(750.0, 750.0);
     let texture = "Sun".to_string();
     
     Sun {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(0.0)
                        .with_inertia(0.0)
                        .with_health(1000.0)
                        .with_health_regen(500.0),
    }
  }
  
  pub fn as_hostile(mut self) -> Sun {
    self.data = self.data.as_hostile();
    self
  }
  
  pub fn as_neutral(mut self) -> Sun {
    self.data = self.data.as_neutral();
    self
  }
}

impl Entity for Sun {
  fn data(&self) -> &EntityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.max(self.data().size.y)*0.5 * 0.7;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
  
  fn collide_with(&mut self, entity: &mut BoxEntity) {
    self.entity_collision(entity, 10.0, 900.0);
  }
}
