use crate::modules::entities::{Entity, EntityData};
use crate::modules::projectiles::Projectile;

use cgmath::Vector2;

#[derive(Clone)]
pub struct Ship {
  data: EntityData,
}

impl Ship {
  pub fn new() -> Ship {
     let position = Vector2::new(640.0, 520.0);
     let size = Vector2::new(150.0, 150.0);
     let texture = "Bulbz".to_string();
     
     Ship {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(500.0)
                        .with_inertia(0.33)
    }
  }
  
  pub fn as_hostile(mut self) -> Ship {
    self.data = self.data.as_hostile();
    self
  }
}

impl Entity for Ship {
  fn data(&self) -> &EntityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.min(self.data().size.y)*0.5 * 0.7;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
  
  fn update(&mut self, delta_time: f32) -> Vec<Box<Projectile>> {
    self.physics(delta_time);
    
    self.return_projectiles()
  }
}
