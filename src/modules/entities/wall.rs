use maat_graphics::math;

use crate::modules::entities::{Entity, EntityData};

use crate::cgmath::Vector2;

#[derive(Clone)]
pub struct Wall {
  data: EntityData,
}

impl Wall {
  pub fn new(position: Vector2<f32>) -> Wall {
     //let position = Vector2::new(-1500.0, 1000.0);
     let size = Vector2::new(750.0, 50.0);
     let texture = "Wall".to_string();
     
     Wall {
      data: EntityData::new(position, size, texture.to_string())
                        .with_max_velocity(0.0)
                        .with_inertia(0.0)
                        .with_health(1000.0)
                        .with_health_regen(500.0),
    }
  }
  
  pub fn as_hostile(mut self) -> Wall {
    self.data = self.data.as_hostile();
    self
  }
  
  pub fn as_neutral(mut self) -> Wall {
    self.data = self.data.as_neutral();
    self
  }
}

impl Entity for Wall {
  fn data(&self) -> &EntityData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut EntityData {
    &mut self.data
  }
  
  fn collision_information(&self) -> Vec<(Vector2<f32>, f32)> {
    let radius = self.data().size.x.min(self.data().size.y)*0.5;
    
    let goal = self.data().size.x.max(self.data().size.y)*0.5;
    
    let mut wall_circles = Vec::new();
    wall_circles.push((Vector2::new(0.0, 0.0), radius));
    
    let _pos = self.data().position;
    let rotation = self.data().rotation;
    
    let num_circles = ((goal / radius).floor()*0.5) as i32 -1;
    
    for i in 0..num_circles {
      
      let offset = Vector2::new(radius*2.5*(i+1) as f32*(math::to_radians(rotation)).cos(),
                                radius*2.5*(i+1) as f32*(math::to_radians(rotation)).sin());
      
      wall_circles.push((offset, radius));
      wall_circles.push((-offset, radius));
    }
    
    wall_circles
  }
  
  fn collide_with(&mut self, entity: &mut Box<Entity>) {
    self.entity_collision(entity, 2.0, 200.0);
  }
}
