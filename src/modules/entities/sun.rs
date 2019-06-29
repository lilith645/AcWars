use crate::modules::entities::sections::{Hull, Wing, Thruster, WeaponMount};
use crate::modules::entities::{Entity, EntityData};
use crate::modules::projectiles::Projectile;

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
                        .with_health(1000.0),
    }
  }
  
  pub fn as_hostile(mut self) -> Sun {
    self.data = self.data.as_hostile();
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
    let radius = self.data().size.x.min(self.data().size.y)*0.5 * 0.7;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
}
