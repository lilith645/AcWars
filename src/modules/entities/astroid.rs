use crate::modules::entities::{Entity, EntityData};
use crate::modules::projectiles::Projectile;

use crate::cgmath::Vector2;

use maat_graphics::math;

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
                        .with_max_velocity(3000.0)
                        .with_inertia(0.33)
                        .with_health(50.0),
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
}

impl Entity for Astroid {
  fn data(&self) -> &EntityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.max(self.data().size.y)*0.5*0.8;
    
    vec!((Vector2::new(0.0, 0.0), radius))
  }
  
  fn collide_with(&mut self, entity: &mut Box<Entity>) {
    let entity_circles = entity.collision_circles();
    let astroid_circles = self.collision_circles();
    
    let mut collided = false;
    
    for e_circle in entity_circles {
      for p_circle in &astroid_circles {
        if math::circle_collision(e_circle, *p_circle) {
          //entity.hit(10.0);
          //self.hit(10.0);
          
          let center = (self.position()+entity.position())*0.5;
          
          let direction = math::normalise_vector2(self.position()-center);
          self.add_acceleration(direction*3000.0);
          
          let direction =  -1.0*direction;
          entity.add_acceleration(direction*3000.0);
          
          collided = true;
          break;
        }
      }
      if collided { break; }
    }
  }
}
