use crate::modules::entities::sections::{Hull, Wing, Thruster, WeaponMount};
use crate::modules::entities::{Entity, EntityData};
use crate::modules::projectiles::Projectile;

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct Ship {
  data: EntityData,
}

impl Ship {
  pub fn new(position: Vector2<f32>) -> Ship {
     let size = Vector2::new(150.0, 150.0);
     let texture = "Bulbz".to_string();
     
     Ship {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(800.0)
                        .with_inertia(0.33)
                        .with_health(500.0)
                        .with_ship_section(Box::new(Hull::new()))
                        .with_ship_section(Box::new(Wing::new()))
                        .with_ship_section(Box::new(Wing::new()))
                        .with_ship_section(Box::new(Thruster::new()))
                        .with_ship_section(Box::new(Thruster::new()))
                        .with_ship_section(Box::new(WeaponMount::new())),
    }
  }
  
  pub fn as_hostile(mut self) -> Ship {
    self.data = self.data.as_hostile();
    self
  }
  
  pub fn with_health(mut self, health: f32) -> Ship {
    self.data = self.data.with_health(health);
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
  
  fn collide_with(&mut self, entity: &mut Box<Entity>) {
    
  }
}
