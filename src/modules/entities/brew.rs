use crate::modules::entities::sections::{Hull, Thruster, WeaponMount};
use crate::modules::entities::{Entity, EntityData};
use crate::modules::projectiles::Projectile;

use crate::cgmath::Vector2;

// Brew - Battle ready energy warBrew

#[derive(Clone)]
pub struct Brew {
  data: EntityData,
}

impl Brew {
  pub fn new() -> Brew {
     let position = Vector2::new(640.0, 1500.0);
     let size = Vector2::new(70.0, 70.0);
     let texture = "Brew".to_string();
     
     Brew {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(500.0)
                        .with_inertia(0.33)
                        .with_health(50.0)
                        .with_ship_section(Box::new(Hull::new()))
                        .with_ship_section(Box::new(Thruster::new()))
                        .with_ship_section(Box::new(WeaponMount::new()))
                        .with_ship_section(Box::new(WeaponMount::new()))
                        .with_ship_section(Box::new(WeaponMount::new()))
    }
  }
  
  pub fn with_position(mut self, position: Vector2<f32>) -> Brew {
    self.data.position = position;
    self
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
}
