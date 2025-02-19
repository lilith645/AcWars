use crate::modules::entities::{Entity, BoxEntity, EntityData};

use crate::cgmath::Vector2;



#[derive(Clone)]
pub struct Astroid {
  data: EntityData,
}

impl Astroid {
  pub fn new(position: Vector2<f32>, size: Vector2<f32>) -> Astroid {
     //let size = Vector2::new(100.0, 100.0);
     let texture = "Astroid".to_string();
     
     Astroid {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(300.0)
                        .with_inertia(0.59)
                        .with_health(150.0),
    }
  }
  
  pub fn as_hostile(mut self) -> Astroid {
    self.data = self.data.as_hostile();
    self
  }
  
  pub fn as_neutral(mut self) -> Astroid {
    self.data = self.data.as_neutral();
    self
  }
  
  pub fn as_misc(mut self) -> Astroid {
    self.data = self.data.as_misc();
    self
  }
}

impl Entity for Astroid {
  fn data(&self) -> &EntityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.max(self.data().size.y)*0.5*0.7;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
  
  fn collide_with(&mut self, entity: &mut BoxEntity) {
    let dmg = if entity.texture() == self.texture() {
      0.0
    } else {
      0.5
    };
    self.entity_collision(entity, dmg, 500.0);
  }
}
