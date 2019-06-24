use crate::modules::entities::{Entity, EntityData};
use crate::modules::projectiles::Projectile;

use cgmath::Vector2;

// Brew - Battle ready energy warBrew

#[derive(Clone)]
pub struct Brew {
  data: EntityData,
}

impl Brew {
  pub fn new() -> Brew {
     let position = Vector2::new(640.0, 1500.0);
     let size = Vector2::new(90.0, 90.0);
     let texture = "Brew".to_string();
     
     Brew {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(500.0)
                        .with_inertia(0.33)
    }
  }
  
  pub fn as_hostile(mut self) -> Brew {
    self.data = self.data.as_hostile();
    self
  }
}

impl Entity for Brew {
  fn data(&self) -> &EntityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.min(self.data().size.y)*0.5 * 0.8;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
  
  fn update(&mut self, delta_time: f32) -> Vec<Box<Projectile>> {
    self.physics(delta_time);
    
    self.return_projectiles()
  }
}
